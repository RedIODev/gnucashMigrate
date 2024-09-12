use std::error::Error;

use xml::{attribute::Attribute, name::{Name, OwnedName}, namespace::Namespace, reader::XmlEvent};

use crate::read::{Account, DBInfo, Transaction};



pub fn write_start_state(old_dbinfo:&DBInfo, new_dbinfo:&DBInfo) -> Result<(), Box<dyn Error>> {
    let source = new_dbinfo.accounts()
            .iter()
            .find(|account| account.name() == "Anfangsbestand");
    let Some(source) = source else {
        println!("Couldn't find Anfangsbestand");
        return Ok(());
    };
    let start_balance_iter = old_dbinfo.accounts()
        .iter()
        .map(|account| (map_account(new_dbinfo, account), old_dbinfo.get_total_value(account)));
    let mut start_balance = Vec::new();
    for account_balance in start_balance_iter {
        let Some(account) = account_balance.0 else {
            println!("couldn't map account");
            return Ok(());
        };
        //TODO: complete quantity and get_total create transaction from account_balance
        start_balance.push((account, account_balance.1));
    }
 
    Ok(())
}

fn create_transaction_xml(transaction:&Transaction) -> Box<[XmlEvent]> {
    let mut events = Vec::new();
    //start transaction
    events.push(XmlEvent::StartElement { 
        name: Name::prefixed("transaction", "gnc").to_owned(), 
        attributes: vec![Attribute::new(Name::local("version"), "2.0.0").to_owned()], 
        namespace: Namespace::empty() 
    });

    events.into_boxed_slice()
}

fn map_account<'a>(new_dbinfo:&'a DBInfo, account:&Account) -> Option<&'a Account> {
    new_dbinfo.accounts().iter().find(|acc| acc.name() == account.name())
}