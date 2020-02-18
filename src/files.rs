// Use code from other module
use crate::parse;

// Use code to keep everything neat
use std::path::Path;
use std::io::Write;
use std::fs::File;

pub fn verify() -> String {

    // Get the path of the ~/.config/ directory
    let conf_dir_path = match dirs::config_dir() {
        Some(x) => x,
        None => panic!()
    };

    // Get the path as a str
    let conf_dir = match conf_dir_path.to_str() {
        Some(text) => text,
        None => panic!()
    };

    // Check if ~/.config/rco/ exists and make it if it doesn't
    if !(Path::new(&(conf_dir.to_owned() + "/rco")).exists()) {
        println!("No config directory found, making one (~/.config/rco)");
        match parse::make_dir(&(conf_dir.to_owned() + "/rco")) {
            Ok(()) => (),
            Err(why) => panic!(why)
        }
    }

    // Check if ~/.config/rco/objects.csv exists and make it if it doesn't
    if !(Path::new(&(conf_dir.to_owned() + "/rco/objects.csv")).exists()) {
        println!("No object file found, making one (~/.config/rco/objects.csv)");
        let mut file = match File::create(&(conf_dir.to_owned() + "/rco/objects.csv")) {
            Ok(x) => x,
            Err(why) => panic!("{}", why)
        };
        match file.write_all("nickname,path,description\nrco,~/.config/rco/config.csv,rco config\n".as_bytes()) {
            Ok(()) => (),
            Err(why) => panic!("{}", why)
        }
    }

    // Check if ~/.config/rco/config.csv exists and make it if it doesn't
    if !(Path::new(&(conf_dir.to_owned() + "/rco/config.csv")).exists()) {
        println!("No config file found, making one (~/.config/rco/config.csv)");
        let mut file = match File::create(&(conf_dir.to_owned() + "/rco/config.csv")) {
            Ok(x) => x,
            Err(why) => panic!("{}", why)
        };
        match file.write_all("property,value\neditor,vi\ncolor,true\nshell,/bin/sh\n".as_bytes()) {
            Ok(()) => (),
            Err(why) => panic!("{}", why)
        }
    }

    conf_dir.to_string()
}
