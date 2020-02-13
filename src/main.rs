mod error;
mod parse;

/*struct Conf {
    editor: &str,
    color: bool,
}*/

fn main() {
    let obj_count = match parse::line_count("/home/theo/Repos/rco/src/foow.csv") {
        Err(parse::error::Errs::ACCESS) => return error::access(),
        Ok(x) =>  x,
    };
    println!("{}", obj_count);
    //parse::go();
}

