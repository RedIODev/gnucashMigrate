use std::{env::{self, Args}, error::Error, fs::{self, File}, io::{stdin, stdout, BufWriter, Write}};

use generator::generate_structs_from_xml;
use mapper::{parse_note_xml, wrappers::{Account, Book, Transaction}};
use read::{read_old_db, DBInfo};

mod read;
mod write;
mod common;
mod mapper;
mod generator;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    //let structs = generate_structs_from_xml("/home/rediodev/Documents/Bierwart/kasse_ss_24.gnucash");
    //fs::write("src/mapper/xml_bindings.rs", structs).expect("write failed");
    
    let xml = parse_note_xml("/home/rediodev/Documents/Bierwart/kasse_ss_24.gnucash")?;
    // println!("Accounts:");
    // for ele in xml.gnc_book.gnc_account {
    //     println!("{}", Account::try_from(&ele)?);
    // }
    // println!("\nTransactions:");
    // for ele in xml.gnc_book.gnc_transaction {
    //     println!("{}", Transaction::try_from(&ele)?)
    // }
    let mut book = Book::try_from(&xml.gnc_book)?;
    book.clear_transactions();
    xml.gnc_book.update(book);
    // let info = read_old_db("/home/rediodev/Documents/Bierwart/kasse_ss_24.gnucash")?;
    // //println!("{:#?}", info);
    // for account in info.accounts() {
    //     println!("'{}'", account.name());
    // }
    // write::write_start_state(&info, &info)?;
    // // loop {
    // //     query_total(&info)?
    // // }
    // // let mut file = File::create("out.txt")?;
    // // write!(file, "{:#?}", info)?;
    Ok(())
}


fn query_total(dbinfo:&DBInfo) -> Result<(), Box<dyn Error>>{
    let mut input = String::new();
    print!("Total for:");
    stdout().flush()?;
    stdin().read_line(&mut input)?;
    let input = input.trim();
    let account = dbinfo.accounts().iter()
            .find(|ac| ac.name() == input);
    let Some(account) = account else {
        println!("account {} not found.", input);
        return Ok(());
    };
    println!("Total Balance for {} is {:?}", input, dbinfo.get_total(account));
    Ok(())
}