use std::process::exit;

pub fn obj_file() {
    eprintln!("Error: Object file non-existent or empty");
    exit(1);
}
