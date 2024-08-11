use std::{collections::HashMap, error::Error, fs::File, io::{self, BufReader}};

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
                if let Some(transaction) = Transaction::new(&mut reader_iter, name) {
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
}


impl Transaction {
    pub fn new<R: io::Read>(xml_iter: &mut Events<R>, name: OwnedName) -> Option<Transaction> {
        if name.local_name != "transaction" {
            return None;
        }
        let mut accounts = HashMap::new();
        while let Some((account, value)) = Self::get_split(xml_iter) {
            accounts.insert(account, value);
        }
        Some(Transaction { accounts })
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

#[derive(Debug)]
pub struct Value(i32, u8);
#[derive(Debug)]
pub struct Quantity(i32, u8);