use crate::parse;
use crate::exits;
use colored::*;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::Command;

pub fn list(obj_vector: &Vec<Vec<String>>, conf_struct: &parse::Conf) {
    println!("Nickname{:<5}Filepath{:<5}Description{:<5}", "", "", "");
    if conf_struct.color == "true" {
        let mut x = 0;
        for i in obj_vector {
            for j in i.iter().enumerate() {
                if x % 2 == 0 {
                    print!("{}{:>5}", j.1.on_white().black(), "".on_white().black());
                } else {

                    print!("{}{:>5}", j.1.on_black().white(), "".on_black().white());
                }
            }
            println!("");
            x += 1;
        }
    } else {
        for i in obj_vector {
            for j in i {
                print!("{}{:->5}", j, "");
            }
            print!("\n");
        }
    }
}

pub fn load(nick: &String, path: &String, desc: &String, obj_name: &String) {
    let obj_in = format!("{},{},{}", nick, path, desc);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(obj_name)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", obj_in) {
        exits::file_write(e);
    }

    exits::success();
}

pub fn unload(nick: &String, obj_name: &String) {

    let file = File::open(obj_name).unwrap();
    let mut new_file: String = "".to_string();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() >= nick.len() + 1 && line[..(nick.len() + 1)].to_owned() == nick.to_owned() + "," {
        } else {
            new_file += &(line + "\n");
        }
    }
    let mut file = match std::fs::File::create(obj_name) {
        Ok(x) => x,
        Err(why) => panic!("{}", why)
    };
    match file.write_all(new_file.as_bytes()) {
        Ok(()) => (),
        Err(why) => panic!("{}", why)
    }

    exits::success();
}

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
        Note that if objects share a nickname, all will be unloaded.

    -h, --help: Display a help (this) message,
        explaining how to use rco, takes no arguments.

    
Edit: Edit the file with corresponding nickname,
    the nickname being the only argument,
    e.g., `rco nickame`

List: Lists all tracked files, ran when provided no arguments,
    e.g., `rco`

Repository hosted on GitHub: https://github.com/tteeoo/rco
Copyright (c) 2020 Theo Henson, MIT License
");
    exits::success();
}

pub fn edit(nickname: &String, conf_struct: &parse::Conf, obj_vector: &Vec<Vec<String>>) {
    let mut path: &String = &"".to_string();

    for i in obj_vector {
        if &i[0] == nickname {
           path = &i[1];
        }
    }
    if path == &"".to_string() {
        exits::obj();
    }
    let command = "".to_owned() + &conf_struct.editor + " " + path;
    println!("Edit is WIP {}", command);
    Command::new(&conf_struct.shell)
        .arg("-c")
        .arg(&command)
        .spawn()
        .expect("Error: Failed to run editor")
        .wait()
        .expect("Error: Editor returned a non-zero status");
    exits::success();
}
