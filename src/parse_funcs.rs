use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use csv;

// Returns csv file as a vector of arrays
// Essentially the function that parses .csv
pub fn get_records(filename: &str) -> std::io::Result<Vec<[String; 3]>> {
    // Initializes vector of arrays
    let mut objs: Vec<[String; 3]> = Vec::new();

    // Initializes csv reader
    let mut rdr = csv::Reader::from_path(filename)?;

    // Iterates through the read records and their fields
    for result in rdr.records() {
        let record = result?;

        // Init array
        let mut obj: [String; 3] = ["EMPTY".to_string(), "EMPTY".to_string(), "EMPTY".to_string()];

        // Adds fields to array
        for field in record.iter().enumerate() {
           obj[field.0] = field.1.to_string(); 
        }

        // Adds array to vector
        objs.push(obj);
    }
    Ok(objs)
}

// Counts the amount of lines in given file
// This is useful for determining the needed size of an array to hold all the objects
pub fn line_count(filename: &str) -> std::io::Result<usize> {
    let mut count: usize = 0;

    // Opens file
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Iterates through each line and counts how many there are
    for _line in reader.lines() {
        count += 1;
    }
    Ok(count)
}
