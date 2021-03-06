// Use code to keep everything neat
use std::process::exit;

// Error when there is a problem writing to a file
pub fn file_write(msg: std::io::Error) {
    eprintln!("Error: Error while writing to file: {}", msg);
    exit(1);
}

// Error when an object doesn't exist
pub fn obj() {
    eprintln!("Error: an object with that nickname does not exist");
    exit(0);
}

// Error when there are invalid arguments
pub fn arg() {
    eprintln!("Error: Invalid arguments, run rco --help to see usage");
    exit(1);
}

// Error when a nickname already exists
pub fn nick() {
    eprintln!("Error: That nickname already exists");
    exit(1);
}

// Ran when code executed successfully
pub fn success() {
    
    // Exits with exit code 0 (success)
    exit(0);
}
