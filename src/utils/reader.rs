use std::fs::{self};

pub fn read_file_line_by_line(filepath: &str) -> Vec<String> {
    let input = fs::read_to_string(filepath).expect("Failed to read file");
    input.lines().map(|line| line.to_string()).collect()
}
