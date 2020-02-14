use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use csv;

pub struct Conf {
    pub editor: String,
    pub color: String,
}

// Returns csv file as a vector of arrays
// Essentially the function that parses .csv
pub fn get_records(filename: &str) -> std::io::Result<Vec<Vec<String>>> {
    // Initializes vector of arrays
    let mut objs: Vec<Vec<String>> = Vec::new();

    // Initializes csv reader
    let mut rdr = csv::Reader::from_path(filename)?;

    // Iterates through the read records and their fields
    for result in rdr.records() {
        let record = result?;

        // Init array
        let mut obj: Vec<String> = Vec::new();

        // Adds fields to array
        for field in record.iter() {
           obj.push(field.to_string());
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

pub fn make_conf(conf_vector: &Vec<Vec<String>>) -> Conf {
    let mut editor: String = "vi".to_string();
    let mut color: String = "true".to_string();

    for i in conf_vector { 
        meta_conf_setter(&i, &mut editor, &mut color);
    }

    Conf { editor: editor, color: color }
}

fn meta_conf_setter(i: &Vec<String>, editor: &mut String, color: &mut String) {
    if i[0] == "editor".to_string() {
        *editor = (*i[1]).to_string();
    }

    if i[0] == "color".to_string() {
        *color = (*i[1]).to_string();
    }
}
