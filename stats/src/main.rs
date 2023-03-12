use lib::{AgentRunStats, OUTPUT_FILE_NAME};
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn main() {
    let mut file = File::open(OUTPUT_FILE_NAME).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect(format!("Failed to read from file {}", OUTPUT_FILE_NAME).as_str());
    let simulation_results: HashMap<&str, Vec<AgentRunStats>> =
        serde_json::from_str(&contents).unwrap();
    println!("{:?}", simulation_results);
}
