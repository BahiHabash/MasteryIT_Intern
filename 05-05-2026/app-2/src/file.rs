use std::fs::{self, File};

pub fn simulate_reading_files(path: &str) -> String {
    let Ok(_file) = File::open(path) else {
        panic!("Failed to open file");
    };

    fs::read_to_string(path).unwrap()
}
