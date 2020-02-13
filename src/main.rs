mod error_exits;
mod parse;

/*struct Conf {
    editor: &str,
    color: bool,
}*/

fn main() {
    let obj_count = match parse::line_count("/home/theo/Repos/rco/src/objects.csv") {
        Err(_why) => 0,
        Ok(x) => x,
    };
    if obj_count == 0 {
        error_exits::obj_file();
    }
    println!("{:?}", obj_count);

    //parse::go();
}

