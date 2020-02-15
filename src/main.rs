/* Copyright (c) 2020 Theo Henson, MIT License
 *
 * A remake of my program comma, now made in rust
 *
*/

mod error_exits;
mod parse_funcs;
mod actions;
extern crate dirs;
use std::path::Path;
use std::io::Write;
use std::env;

fn main() {

    // VARIABLE FETCH
    let conf_dir_path = match dirs::config_dir() {
        Some(x) => x,
        None => panic!()
    };
    let conf_dir = match conf_dir_path.to_str() {
        Some(text) => text,
        None => panic!()
    };
    if !(Path::new(&(conf_dir.to_owned() + "/rco")).exists()) {
        println!("No config directory found, making one (~/.config/rco)");
        match parse_funcs::make_dir(&(conf_dir.to_owned() + "/rco")) {
            Ok(()) => (),
            Err(why) => panic!(why)
        }
    }
    if !(Path::new(&(conf_dir.to_owned() + "/rco/objects.csv")).exists()) {
        println!("No object file found, making one (~/.config/rco/objects.csv)");
        let mut file = match std::fs::File::create(&(conf_dir.to_owned() + "/rco/objects.csv")) {
            Ok(x) => x,
            Err(why) => panic!("{}", why)
        };
        match file.write_all("nickname,path,description\nrco,~/.config/rco/config.csv,rco config\n".as_bytes()) {
            Ok(()) => (),
            Err(why) => panic!("{}", why)
        }
    }
    if !(Path::new(&(conf_dir.to_owned() + "/rco/config.csv")).exists()) {
        println!("No config file found, making one (~/.config/rco/config.csv)");
        let mut file = match std::fs::File::create(&(conf_dir.to_owned() + "/rco/config.csv")) {
            Ok(x) => x,
            Err(why) => panic!("{}", why)
        };
        match file.write_all("property,value\neditor,vi\ncolor,true\n".as_bytes()) {
            Ok(()) => (),
            Err(why) => panic!("{}", why)
        }
    }
    let obj_name = conf_dir.to_owned() + "/rco/objects.csv";
    let conf_name = conf_dir.to_owned() + "/rco/config.csv";
    let obj_count = match parse_funcs::line_count(&obj_name) {
        Err(_why) => 0,
        Ok(x) => x - 1,
    };
    if obj_count <= 0 {
        error_exits::obj_file();
    }
    let objs = match parse_funcs::get_records(&obj_name) {
        Err(why) => panic!("{}", why),
        Ok(x) => x,
    };
    let confs = match parse_funcs::get_records(&conf_name) {
        Err(why) => panic!("{}", why),
        Ok(x) => x,
    };
    let confs = parse_funcs::make_conf(&confs);
    let args: Vec<_> = env::args().collect();
    
    if args.len() == 1 {
        actions::list(&objs, &confs);
    }

    
    // DEBUG
    for i in &objs {
        println!("{:?}", i);
    }
    println!("{}", obj_count);
    println!("{:?}", conf_dir);
    println!("{:?}", confs.editor);
    for argument in &args {
        println!("{}", argument);
    }
    println!("{}", args.len());
}

