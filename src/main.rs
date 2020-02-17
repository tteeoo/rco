/* Copyright (c) 2020 Theo Henson, MIT License
 *
 * A remake of my program comma, now made in rust
 *
 * TODO:
 * + Get to minimum viable product
 *  - Edit function
 *  - Help function
 *
 * + Clean and optimize the codebase
 *  - Not too sure what this entails, but I will fix, improve, and streamline
 *      bad code (the whole thing).
 *
 * + (Maybe) get in the AUR
 *  - Learn how PKGBUILD works
 *  - Make one
 *  - Submit to AUR? -Not too sure how
*/

mod exits;
mod parse;
mod actions;
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
        match parse::make_dir(&(conf_dir.to_owned() + "/rco")) {
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
    let obj_count = match parse::line_count(&obj_name) {
        Err(_why) => 0,
        Ok(x) => x - 1,
    };
    if obj_count <= 0 {
        exits::obj_file();
    }
    let objs = match parse::get_records(&obj_name) {
        Err(why) => panic!("{}", why),
        Ok(x) => x,
    };
    let confs = match parse::get_records(&conf_name) {
        Err(why) => panic!("{}", why),
        Ok(x) => x,
    };
    let confs = parse::make_conf(&confs);
    let args: Vec<_> = env::args().collect();
    
    // List
    if args.len() == 1 {
        actions::list(&objs, &confs);
    } 

    // Load
    else if args[1] == "-l" || args[1] == "--load" {
        if args.len() == 5 {
            actions::load(&args[2], &args[3], &args[4], &obj_name);
        } 
        else {
            exits::arg();
        }
    }
    // Unload
    else if args[1] == "-u" || args[1] == "--unload" {
        actions::unload(&args[2], &obj_name);
    } 
    //Help
    else if args[1] == "-h" || args[1] == "--help" {
        actions::help();
    } 
    //Edit
    else if args.len() == 2 {
        actions::edit(&args[1], &confs, &objs);
    }
    // Error
    else {
        exits::arg();
    }
}

