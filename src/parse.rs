use std::fs::File;
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

pub fn line_count(filename: &str) -> std::io::Result<isize> {
    let mut count: isize = 0;
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for _line in reader.lines() {
        count += 1;
    }
    Ok(count)
}
