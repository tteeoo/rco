/* Copyright (c) 2020 Theo Henson, MIT License
 *
 * A remake of my program comma, now made in rust
 *
 * TODO:
 * + Clean and optimize the codebase
 *  - Not too sure what this entails, but I will fix, improve, and streamline
 *      bad code (the whole thing).
 *
 * + (Maybe) get in the AUR
 *  - Learn how PKGBUILD works
 *  - Make one
 *  - Submit to AUR? -Not too sure how
*/

// Use code from the other modules
mod exits;
mod parse;
mod actions;
mod files;

// Use code to keep everything neat
use std::env;

fn main() {

    // FILESYSTEM HANDLING
    
    // See verify function for what it does; returns ~/.config/
    let conf_dir = files::verify(); 


    // VARIABLES AND PARSING

    // Make some variables for easy refernce
    let obj_name = conf_dir.to_owned() + "/rco/objects.csv";
    let conf_name = conf_dir.to_owned() + "/rco/config.csv";
    let args: Vec<_> = env::args().collect();
    let obj_count = match parse::line_count(&obj_name) {
        Err(_why) => 0,
        Ok(x) => x - 1,
    };

    // Verify objects file
    if obj_count <= 0 {
        exits::obj_file();
    }

    // Creates a vector to represent every object
    let objs = match parse::get_records(&obj_name) {
        Err(why) => panic!("{}", why),
        Ok(x) => x,
    };

    // Creates a struct to represent every configuration
    let confs = match parse::get_records(&conf_name) {
        Err(why) => panic!("{}", why),
        Ok(x) => x,
    };
    let confs = parse::make_conf(&confs);
   
    
    // MAIN FUNCTIONALITY

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

    // Help
    else if args[1] == "-h" || args[1] == "--help" {
        actions::help();
    } 

    // Edit
    else if args.len() == 2 {
        actions::edit(&args[1], &confs, &objs);
    }

    // Argument error
    else {
        exits::arg();
    }
}
