use std::{env::{self, Args}, error::Error, fs::File, io::{stdin, stdout, BufWriter, Write}};

use read::{read_old_db, DBInfo};

mod read;
mod write;


fn main() -> Result<(), Box<dyn Error>> {
    let info = read_old_db("/home/rediodev/Documents/Bierwart/kasse_ss_24.gnucash")?;
    //println!("{:#?}", info);
    for account in info.accounts() {
        println!("'{}'", account.name());
    }
    loop {
        query_total(&info)?
    }
    // let mut file = File::create("out.txt")?;
    // write!(file, "{:#?}", info)?;
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
    println!("Total Balance for {} is {:?}", input, dbinfo.get_total_value(account));
    Ok(())
}