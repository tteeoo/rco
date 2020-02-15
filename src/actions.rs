use crate::parse_funcs;
use colored::*;

pub fn list(conf_vector: &Vec<Vec<String>>, conf_struct: &parse_funcs::Conf) {
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

