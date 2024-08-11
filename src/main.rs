use std::{env::{self, Args}, error::Error, fs::File, io::{BufWriter, Write}};

use read::read_old_db;

mod read;


fn main() -> Result<(), Box<dyn Error>> {
    let info = read_old_db("/home/rediodev/Documents/Bierwart/kasse_ss_24.gnucash")?;
    //println!("{:#?}", info);
    let mut file = File::create("out.txt")?;
    write!(file, "{:#?}", info)?;
    Ok(())
}
