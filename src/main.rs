/* Copyright (c) 2020 Theo Henson, MIT License
 *
 * A remake of my program comma, now made in rust
 *
*/

mod error_exits;
mod parse_funcs;
extern crate dirs;

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


    
    // DEBUG
    for i in &objs {
        println!("{:?}", i);
    }
    println!("{}", obj_count);
    println!("{:?}", conf_dir);
    println!("{:?}", confs.editor);
}

