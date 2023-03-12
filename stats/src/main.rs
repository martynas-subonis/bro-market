use lib::OUTPUT_FILE_NAME;
use std::fs::File;
use std::io::Read;

pub fn main() {
    let mut file = File::open(OUTPUT_FILE_NAME).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect(format!("Failed to read from file {}", OUTPUT_FILE_NAME).as_str());
}
