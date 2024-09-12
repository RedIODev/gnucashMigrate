use std::{collections::HashMap, error::Error, fs::File, io::{self, BufReader}, ops::AddAssign};

use uuid::{uuid, Uuid};
use xml::{self, name::OwnedName, reader::{Events, XmlEvent}, EventReader};


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

    pub fn get_total_value(&self, account:&Account) -> Value {
        self.transactions.iter()
                .filter_map(|transaction| transaction.accounts.get(&account.uuid))
                .map(|value| value.0)
                .sum()
    }
}

#[derive(Debug)]
pub struct Account {
    uuid: Uuid,
    name: String,
}

impl Account {
    pub fn new<R: io::Read>(xml_iter: &mut Events<R>, name: OwnedName) -> Option<Account> {
        if name.local_name != "account" {
            return None;
        }
        let mut result_name = None;
        let mut result_uuid = None;
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

                        _ => {continue;}
                    }
                }

                Ok(XmlEvent::EndElement { name }) => {
                    if name.local_name == "account" {
                        return Some(Account { uuid: result_uuid.unwrap(), name: result_name.unwrap()});
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
                Ok(XmlEvent::StartElement { name, attributes: _, namespace }) => {
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
                                result_value = Some(Value(value.parse().unwrap(), divisor.parse().unwrap()));
                            }
                        }

                        ("quantity", Some("split")) => {
                            if let Some(Ok(XmlEvent::Characters(text))) = xml_iter.next() {
                                let (value, divisor) = text.split_once('/').unwrap();
                                result_quantity = Some(Quantity(value.parse().unwrap(), divisor.parse().unwrap()));
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
    accounts:HashMap<Uuid, (Value, Quantity)>
}

#[derive(Debug, Clone, Copy)]
pub struct Value(i32, u16);

impl std::ops::Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if rhs.1 > self.1 {
            let factor = rhs.1 / self.1;
            Value(rhs.0 + self.0* factor as i32, rhs.1)
        } else {
            let factor = self.1 / rhs.1;
            Value(self.0 + rhs.0* factor as i32, self.1)
        }
    }
}

impl std::ops::AddAssign for Value {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl std::iter::Sum for Value {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = Self::default();
        for value in iter {
            sum += value;
        }
        sum
    }
}

impl Default for Value {
    fn default() -> Self {
        Self(Default::default(), 100)
    }
}


#[derive(Debug)]
pub struct Quantity(i32, u16);