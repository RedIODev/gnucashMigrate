use std::{collections::HashMap, error::Error, fs::File, io::{self, BufReader}};

use uuid::Uuid;
use xml::{self, name::OwnedName, reader::{Events, XmlEvent}, EventReader};

use crate::common::{Fixed, SumExtender};


pub fn read_old_db(path:&str) -> Result<DBInfo, Box<dyn Error>> {
    let source = BufReader::new(File::open(path)?);
    let reader = EventReader::new(source);
    let mut result = DBInfo::new(path);
    let mut reader_iter = reader.into_iter();
    while let Some(element) = reader_iter.next() {
        match element {
            Ok(XmlEvent::StartElement { name, ..}) => {
                if let Some(account) = Account::new(&mut reader_iter, name.clone()) {
                    result.accounts.push(account);
                }
                if let Some(transaction) = Transaction::from_xml(&mut reader_iter, name) {
                    result.transactions.push(transaction);
                }
            }

            Err(e) => {
                return Err(Box::new(e));
            }

            _ => {}
        }
    }
    Ok(result)
}

#[derive(Debug)]
pub struct DBInfo<'a> {
    accounts:Vec<Account>,
    transactions: Vec<Transaction>,
    path:&'a str,
}

impl<'a> DBInfo<'a> {
    pub fn new(path:&str) -> DBInfo {
        DBInfo { accounts: Vec::new(), transactions: Vec::new(), path: path }
    }

    pub fn accounts(&self) -> &Vec<Account> {
        &self.accounts
    }

    pub fn get_total(&self, account:&Account) -> (Value, Quantity) {
        let mut flag = false;
        if account.name() == "Veranstaltungen" {
            flag = true;
        }
        let (value, quantity): (SumExtender<_>, SumExtender<_>) = self.transactions.iter()
                .filter_map(|transaction| transaction.accounts.get(&account.uuid))
                .inspect(|v| if flag {println!("Veranstaltungen:{:?}", v)})
                .map(|amount| (amount.0.0, amount.1.0))
                .unzip();
        (Value(*value), Quantity(*quantity))
    }
}

#[derive(Debug)]
pub struct Account {
    uuid: Uuid,
    name: String,
    parent: Option<Uuid>,
}

impl Account {
    pub fn new<R: io::Read>(xml_iter: &mut Events<R>, name: OwnedName) -> Option<Account> {
        if name.local_name != "account" {
            return None;
        }
        let mut result_name = None;
        let mut result_uuid = None;
        let mut result_parent = None;
        while let Some(element) = xml_iter.next() {
            match element {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    match name.local_name.as_str() {
                        "name" => {
                            if let Some(Ok(XmlEvent::Characters(text))) = xml_iter.next() {
                                result_name.get_or_insert(text);
                            }
                        }

                        "id" => {
                            if let Some(Ok(XmlEvent::Characters(text))) = xml_iter.next() {
                                result_uuid.get_or_insert_with(||Uuid::parse_str(&text).expect("Invalid UUID."));
                            }
                        }

                        "parent" => {
                            if let Some(Ok(XmlEvent::Characters(text))) = xml_iter.next() {
                                result_parent.get_or_insert_with(||Uuid::parse_str(&text).expect("Invalid UUID."));
                            }
                        }
                        _ => {continue;}
                    }
                }

                Ok(XmlEvent::EndElement { name }) => {
                    if name.local_name == "account" {
                        return Some(Account { uuid: result_uuid.unwrap(), name: result_name.unwrap(), parent: result_parent});
                    }
                }

                Err(_) => {
                    return None;
                }

                _ => {}
            }
        }
        None
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn parent(&self) -> Option<Uuid> {
        self.parent
    }
}


impl Transaction {
    pub fn from_xml<R: io::Read>(xml_iter: &mut Events<R>, name: OwnedName) -> Option<Transaction> {
        if name.local_name != "transaction" {
            return None;
        }
        let mut accounts = HashMap::new();
        while let Some((account, value)) = Self::get_split(xml_iter) {
            accounts.insert(account, value);
        }
        Some(Transaction { accounts })
    }

    pub fn new(splits: Vec<(Uuid, Value, Quantity)>) -> Transaction {
        let mut accounts = HashMap::new();
        for account in splits {
            accounts.insert(account.0, (account.1, account.2));
        }
        Transaction { accounts }
    }

    fn get_split<R: io::Read>(xml_iter: &mut Events<R>) -> Option<(Uuid, (Value, Quantity))> {
        let mut result_uuid = None;
        let mut result_value = None;
        let mut result_quantity = None;
        while let Some(element) = xml_iter.next() {
            match element {
                Ok(XmlEvent::StartElement { name, attributes: _, namespace: _}) => {
                    match (name.local_name.as_str(), name.namespace_ref().map(|str| str.rsplit_once('/').unwrap().1)) {
                        ("account", Some("split")) => {
                            if let Some(Ok(XmlEvent::Characters(text))) = xml_iter.next() {
                                result_uuid.get_or_insert_with(||Uuid::parse_str(&text).expect("Invalid UUID."));
                            }
                        }

                        ("value", Some("split")) => {
                            if let Some(Ok(XmlEvent::Characters(text))) = xml_iter.next() {
                                let (value, divisor) = text.split_once('/').unwrap();
                                // println!("{value},{divisor}");
                                result_value = Some(Value(Fixed(value.parse().unwrap(), divisor.parse().unwrap())));
                            }
                        }

                        ("quantity", Some("split")) => {
                            if let Some(Ok(XmlEvent::Characters(text))) = xml_iter.next() {
                                let (value, divisor) = text.split_once('/').unwrap();
                                result_quantity = Some(Quantity(Fixed(value.parse().unwrap(), divisor.parse().unwrap())));
                            }
                        }

                        // (s1, s2) => {
                        //     println!("{}, {:?}", s1, s2);
                        //     continue;
                        // }

                        _ => {continue;}
                    }
                }

                Ok(XmlEvent::EndElement { name }) => {
                    if name.local_name == "split" {
                        return Some((result_uuid.unwrap(), (result_value.unwrap(), result_quantity.unwrap())));
                    }
                    if name.local_name == "transaction" {
                        return None;
                    }
                }

                Err(_) => {
                    return None;
                }

                _ => {}
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct Transaction {
    pub accounts:HashMap<Uuid, (Value, Quantity)>
}

#[derive(Debug, Clone, Copy)]
pub struct Value(pub Fixed<i32>);


impl Default for Value {
    fn default() -> Self {
        Self(Fixed(0, 100))
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Quantity(pub Fixed<i32>);


impl Default for Quantity {
    fn default() -> Self {
        Self(Fixed(0, 100))
    }
}