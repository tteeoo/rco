// Use code from other modules
use crate::parse;
use crate::exits;

// Use code to keep things neat
use colored::*;
use std::fs::{OpenOptions, File};
use std::io::{BufRead, BufReader, Write};
use std::process::Command;

// List function lists out every object
pub fn list(obj_vector: &Vec<Vec<String>>, conf_struct: &parse::Conf) {

    // Prints header
    println!("Nickname{:<5}Filepath{:<5}Description{:<5}", "", "", "");

    // Printing if color is enabled
    if conf_struct.color == "true" {
        let mut x = 0;
        for i in obj_vector {
            for j in i.iter().enumerate() {

                // Alternates printing black on white and white on black
                if x % 2 == 0 {
                    print!("{}{:>5}", j.1.on_white().black(), "".on_white().black());
                }
                else {
                    print!("{}{:>5}", j.1.on_black().white(), "".on_black().white());
                }
            }
            println!("");
            x += 1;
        }
    }

    // Printing if color is not enabled
    else {
        for i in obj_vector {
            for j in i {
                print!("{}{:->5}", j, "");
            }
            print!("\n");
        }
    }
}

// Load function appends object to objects.csv
pub fn load(nick: &String, path: &String, desc: &String, obj_name: &String) {

    // Formats arguments into a string that actually represents an object
    let obj_in = format!("{},{},{}", nick, path, desc);

    // Creates a file object that opens for appending
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(obj_name)
        .unwrap();

    // Writes object to file
    if let Err(e) = writeln!(file, "{}", obj_in) {
        exits::file_write(e);
    }

    exits::success();
}

// Unload function removes specific object from objects.csv
pub fn unload(nick: &String, obj_name: &String) {

    // Makes a file object for the object file
    let file = File::open(obj_name).unwrap();

    // Makes a variable for the text to be written to the file
    let mut new_file: String = "".to_string();

    // Creates a reader object to read the file
    let reader = BufReader::new(file);

    // Initialize diff variable to check if nickname actually exists
    let mut diff = 0;

    // Reads the file line by line
    for line in reader.lines().enumerate() {
        let line = (line.0, line.1.unwrap());

        // If the line does not start with the nickname + , it appends it to the new file string
        if !((line.1.len() >= nick.len() + 1) && ((line.1[..(nick.len() + 1)].to_owned()) == (nick.to_owned() + ","))) || line.0 == 0 {
            new_file += &(line.1 + "\n");
        }
        else {
            diff += 1;
        }

    }

    // Checks if there was actually an object with that nickname
    // If not, it errors out
    if diff == 0 {
        exits::obj();
    }

    // Makes a file object for writing to the object file
    let mut file = match File::create(obj_name) {
        Ok(x) => x,
        Err(why) => panic!("{}", why)
    };

    // Writes the new string to the file, overwriting everything 
    // Only losing the specified object
    match file.write_all(new_file.as_bytes()) {
        Ok(()) => (),
        Err(why) => panic!("{}", why)
    }

    exits::success();
}

// Help function prints help message
pub fn help() {
    print!("Usage: rco [OPTION... ] | [NICKNAME]

Options:
    -l, --load: Loads objects (starts tracking a file),
        takes arguments of the nickname, filepath, and description,
        e.g., `rco -l nickname /path/to/file \"description\"`
        Note the quotes which are necessary when writing multi-word descriptions.

    -u, --unload: Unloads objects,
        only takes the argument of the file's nickname,
        e.g., `rco -u nickname`

    -h, --help: Display a help (this) message,
        explaining how to use rco, takes no arguments.

    -V, --version: Display the current version of rco.

    -s, --sudo: Used before a nickname when editing,
        opens the file as root, using sudo,
        e.g., `rco -s nickname`

Edit: Edit the file with corresponding nickname,
    the nickname being the only argument,
    e.g., `rco nickame`
    Use -s before the nickname to edit as root.

List: Lists all tracked files, ran when provided no arguments,
    e.g., `rco`

Repository hosted on GitHub: https://github.com/tteeoo/rco
Copyright (c) 2020 Theo Henson <theodorehenson@protonmail.com>, MIT License
");
    exits::success();
}

// Edit function opens up a file in the chosen editor
pub fn edit(nickname: &String, conf_struct: &parse::Conf, obj_vector: &Vec<Vec<String>>, sudo: bool) {

    // Initializes the path variable which represents the path of the file to edit
    let mut path: &String = &"".to_string();

    // Searches the objects for the one with the specified nickname
    // Grabs that objects path
    for i in obj_vector {
        if &i[0] == nickname {
           path = &i[1];
        }
    }

    // If the path is still "" it errors out
    if path == &"".to_string() {
        exits::obj();
    }

    // Makes a variable for the command to be ran
    let command: String;
    if sudo {
        command = "sudo ".to_owned() + &conf_struct.editor + " " + path;
    } else {
        command = "".to_owned() + &conf_struct.editor + " " + path;
    }

    // Runs your editor on the path, through a shell
    Command::new(&conf_struct.shell)
        .arg("-c")
        .arg(&command)
        .spawn()
        .expect("Error: Failed to run editor")
        .wait()
        .expect("Error: Editor returned a non-zero status");
    exits::success();
}

// Function to print the current running version
pub fn version() {
    println!("Running rco-0.1.5");
    exits::success();
}
