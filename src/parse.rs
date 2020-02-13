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

// Counts the amount of lines in given file
// This is useful for determining the needed size of an array to hold all the objects
pub fn line_count(filename: &str) -> std::io::Result<i32> {
    let mut count = 0;

    // Opens file
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Iterates through each line and counts how many there are
    for _line in reader.lines() {
        count += 1;
    }
    Ok(count)
}
