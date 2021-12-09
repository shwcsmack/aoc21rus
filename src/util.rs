use std::fs;

pub fn read_file(file: &str) -> String {
    fs::read_to_string(file).expect("Couldnt Read File")
}