use std::process::exit;

// A function that is run when there was a problem with the objects file
pub fn obj_file() {
    eprintln!("Error: Object file non-existent, couldn't be read, invalid, or empty");
    exit(1);
}

pub fn file_write(msg: std::io::Error) {
    eprintln!("Error: Error while writing to file: {}", msg);
    exit(1);
}

pub fn obj() {
    eprintln!("Error: an object with that nickname does not exist");
    exit(0);
}

pub fn arg() {
    eprintln!("Error: Invlalid arguments, run rco --help to see usage");
    exit(1);
}

pub fn success() {
    exit(0);
}
