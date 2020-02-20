/* Copyright (c) 2020 Theo Henson, MIT License
 *
 * rco: rusty configurer
 *
 * A remake of my program comma, now made in rust
 *
*/

// Use code from the other modules
mod exits;
mod parse;
mod actions;
mod files;

// Main funtion of entire program
fn main() {

    // FILESYSTEM HANDLING
    
    // See verify function for what it does; returns ~/.config/
    let conf_dir = files::verify(); 


    // VARIABLES AND PARSING

    // Make some variables for easy refernce
    let obj_name = conf_dir.to_owned() + "/rco/objects.csv";
    let conf_name = conf_dir.to_owned() + "/rco/config.csv";
    let args: Vec<_> = std::env::args().collect();

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

        // Make sure nickname isn't duplicate
        for i in objs {
            for j in i {
                if args[2] == j {
                    exits::nick();
                }
            }
        }

        // Verify all arguments are provided
        if args.len() == 5 {
            actions::load(&args[2], &args[3], &args[4], &obj_name);
        } 
        else {
            exits::arg();
        }
    }

    // Unload
    else if args[1] == "-u" || args[1] == "--unload" {

        // Verify all arguments are provided
        if args.len() == 3 {
            actions::unload(&args[2], &obj_name);
        }
        else {
            exits::arg();
        }
    } 

    // Help
    else if args[1] == "-h" || args[1] == "--help" {
        actions::help();
    } 

    // Version
    else if args[1] == "-V" || args[1] == "--version" {
        actions::version();
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
