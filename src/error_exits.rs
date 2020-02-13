use std::process::exit;

// A function that is run when there was a problem with the objects file
pub fn obj_file() {
    eprintln!("Error: Object file non-existent, couldn't be read, invalid, or empty");
    exit(1);
}
