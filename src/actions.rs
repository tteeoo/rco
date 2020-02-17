use crate::parse;
use crate::exits;
use colored::*;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn list(conf_vector: &Vec<Vec<String>>, conf_struct: &parse::Conf) {
    println!("Nickname{:<5}Filepath{:<5}Description{:<5}", "", "", "");
    if conf_struct.color == "true" {
        let mut x = 0;
        for i in conf_vector {
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
        for i in conf_vector {
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

    println!("{}", obj_in);
    exits::success();
}

pub fn unload(nick: &String) {
    println!("{}", nick);
    exits::success();
}

pub fn help() {
    print!("\n");
    exits::success();
}

pub fn edit(conf_struct: &parse::Conf) {

    exits::success();
}
