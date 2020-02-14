/* Copyright (c) 2020 Theo Henson, MIT License
 *
 * A remake of my program comma, now made in rust
 *
*/

mod error_exits;
mod parse_funcs;

fn main() {
    // Atempts to get the amount of objects (really just the amount of lines in the objects file)
    let obj_count = match parse_funcs::line_count("/home/theo/Repos/rco/src/objects.csv") {

        // If there was an error, it sets the object count to 0, if not it sets obj_count correctly
        Err(_why) => 0,
        Ok(x) => x - 1,
    };

    // If obj_count is 0, an error is thrown, exiting the program
    // This is caused either by there being no objects (such that the program will not work)
    // Or because there was an error reading the file, as described above
    if obj_count <= 0 {
        error_exits::obj_file();
    }

    // Fore debug purposes, prints the obj_count
    println!("{}", obj_count);

    // Runs get_records() on objects file and get vector of arrays representing objects
    let objs = match parse_funcs::get_records("/home/theo/Repos/rco/src/objects.csv") {
        Err(why) => panic!("{}", why),
        Ok(x) => x,
    };

    let confs = match parse_funcs::get_records("/home/theo/Repos/rco/src/config.csv") {
        Err(why) => panic!("{}", why),
        Ok(x) => x,
    };

    // Prints all object arrays (debug purposes)
    for i in &objs {
        println!("{:?}", i);
    }
    for i in &confs {
        println!("{:?}", i);
    }

    let confs = parse_funcs::make_conf(&confs);

    println!("{:?}", confs.editor);
}

