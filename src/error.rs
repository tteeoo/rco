use std::process::exit;

#[derive(Debug)]
pub enum Errs {
    ACCESS
}

pub fn access() {
    eprintln!("ACCESS ERROR");
    exit(1);
}
