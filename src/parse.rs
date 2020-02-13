#[path = "error.rs"] pub mod error;
use std::fs::File;
use std::error::Error;
use std::io::BufReader;
use std::io::prelude::*;
//use csv::Reader;

/*pub fn go() -> Result<()> {
    let mut rdr = Reader::from_path("/home/theo/Repos/rco/src/foo.csv")?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}*/

pub fn line_count(filename: &str) -> Result<isize, error::Errs> {
    let mut _count: isize = 0;
    let file = match File::open(filename) {
        Err(Error) => return Err(error::Errs::ACCESS),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    for _line in reader.lines() {
        _count += 1;
    }
    Ok(_count) 
}
