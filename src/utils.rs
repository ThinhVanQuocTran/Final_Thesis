use std::fs::File;
use std::io::Read;

// Utility function to read SQL from a file
pub fn read_file(filename: &str) -> String {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return String::new(),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    contents
}
