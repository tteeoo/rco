// Use code to keep everything neat
use std::fs;
use csv;

// Defines a struct to store your configuration
pub struct Conf {
    pub editor: String,
    pub color: String,
    pub shell: String,
}

// Function to get the records of a csv file as a vector of vectors of Strings
pub fn get_records(filename: &str) -> std::io::Result<Vec<Vec<String>>> {

    // Initializes vector
    let mut objs: Vec<Vec<String>> = Vec::new();

    // Initializes reader
    let mut rdr = csv::Reader::from_path(filename)?;

    // Iterates each record
    for result in rdr.records() {

        // Validate record
        let record = result?;

        // Initializes meta vector
        let mut obj: Vec<String> = Vec::new();

        // Iterates each field of the record
        for field in record.iter() {

            // Adds the field to the meta vector
            obj.push(field.to_string());
        }
    
        // Adds the meta vector to the vector
        objs.push(obj);
    }
    Ok(objs)
}

// Function to make a Conf struct out of a vector
pub fn make_conf(conf_vector: &Vec<Vec<String>>) -> Conf {

    // Initializes variables for each value
    // These will be overwitten hopefully, and if not specified they are the default values
    let mut editor: String = "vi".to_string();
    let mut color: String = "true".to_string();
    let mut shell: String = "/bin/sh".to_string();

    // Iterates vector
    for i in conf_vector { 

        // Runs private function to detect if a config is specified and set it
        meta_conf_setter(&i, &mut editor, &mut color, &mut shell);
    }

    Conf { editor: editor, color: color, shell: shell }
}

// This function is a result of a rust beginner fighting the borrow checker until something works
// Maybe not the most effective or idiomatic way, but it works
fn meta_conf_setter(i: &Vec<String>, editor: &mut String, color: &mut String, shell: &mut String) {

    // Detects if config is a keyword
    // If so, sets a certain variable to the config's value
    if i[0] == "editor".to_string() {
        *editor = (*i[1]).to_string();
    }

    if i[0] == "color".to_string() {
        *color = (*i[1]).to_string();
    }

    if i[0] == "shell".to_string() {
        *shell = (*i[1]).to_string();
    }
}

// Function for makign a directory using Result
// Just makes other code smaller, and better checks for errors
pub fn make_dir(path: &str) -> std::io::Result<()> {
    fs::create_dir(path)?;
    Ok(())
}
