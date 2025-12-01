use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

pub fn file_to_lines(file: &str) -> io::Result<Lines<BufReader<File>>> {
    let full_path = format!("{}{}", "/Users/misja/rust_projects/aoc_2025/resources/", file);
    let file = File::open(full_path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines())
}