use std::{collections::HashMap, error::Error, fs::File, io::{BufWriter, Write}};

use chrono::{Date, DateTime, Local, NaiveDate};
use uuid::Uuid;
use xml::{attribute::Attribute, name::Name, namespace::Namespace, reader::XmlEvent, writer::events};

use crate::read::{Account, DBInfo, Quantity, Transaction, Value};


//refactor to do 1 account at a time
pub fn write_start_state(old_dbinfo:&DBInfo, new_dbinfo:&DBInfo) -> Result<(), Box<dyn Error>> {
    let source = new_dbinfo.accounts()
            .iter()
            .find(|account| account.name() == "Anfangsbestand");
    let Some(source) = source else {
        println!("Couldn't find Anfangsbestand");
        return Ok(());
    };

    let a = old_dbinfo.accounts()
            .iter()
            .filter_map(|account| map_account(new_dbinfo, account).map(|new_acc| (account, new_acc)))
            .map(|(old_acc, new_acc)| (new_acc, old_dbinfo.get_total(old_acc)))
            .filter(|(_, (value, quantity))| !value.0.is_zero() && !quantity.0.is_zero())
            .map(|(acc, balance)| {
                let neg_balance = (Value(-balance.0.0), Quantity(-balance.1.0));
                Transaction::new(
                    vec![(acc.uuid(), neg_balance.0, neg_balance.1), (source.uuid(), balance.0, balance.1)]
                )
            })
            .map(create_transaction_xml);
            
    let mut output = BufWriter::new(File::create("test.xml")?);    
    for transaction in a {
        output.write_all(transaction.as_bytes())?
    }
    // let start_balance_iter = old_dbinfo.accounts()
    //     .iter()
    //     .map(|account| (map_account(new_dbinfo, account), old_dbinfo.get_total(account)));
    // let mut start_balance = HashMap::new();
    // for account_balance in start_balance_iter {
    //     let Some(account) = account_balance.0 else {
    //         println!("couldn't map account");
    //         return Ok(());
    //     };
    //     start_balance.insert(account.uuid(), account_balance.1);
    // }
    
    Ok(())
}

fn map_account<'a>(new_dbinfo:&'a DBInfo, account:&Account) -> Option<&'a Account> {
    new_dbinfo.accounts().iter().find(|acc| acc.name() == account.name())
}

pub fn create_transaction_xml(transaction:Transaction) -> String {
    let mut splits = String::new();
    for (uuid, (value, quantity)) in transaction.accounts {
        splits.push_str(format!(
            include_str!("../resource/split.xml"), 
            Uuid::new_v4().as_simple(), 
            value.0.to_string_raw(), 
            quantity.0.to_string_raw(), 
            uuid.as_simple()
        ).as_str());
    }
    //change date to set date
    let date = Local::now();
    format!(
        include_str!("../resource/transaction.xml"),
        Uuid::new_v4().as_simple(),
        date.format("%Y-%m-%d %h:%M:00 +0000"),
        date.format("%Y-%m-%d %h:%M:00 +0000"),
        date.format("%Y-%m-%d"),
        splits)
}