use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Lines};

pub fn file_to_lines(file: &str) -> io::Result<Lines<BufReader<File>>> {
    let full_path = format!("{}{}", "/home/almam/RustroverProjects/aoc_2025/resources/", file);
    let file = File::open(full_path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines())
}

pub fn file_as_string(file: &str) -> io::Result<String> {
    let full_path = format!("{}{}", "/home/almam/RustroverProjects/aoc_2025/resources/", file);
    fs::read_to_string(full_path)
}