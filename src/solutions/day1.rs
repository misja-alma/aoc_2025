use std::io;
use regex::Regex;
use crate::utils::*;

fn parse_move(s: &String) -> io::Result<i32> {
    let re = Regex::new(r"^([LR])([1-9]\d*)$").unwrap();

    if let Some(caps) = re.captures(s.as_str()) {
        let direction = match caps.get(1).unwrap().as_str() {
            "L" => -1,
            "R" => 1,
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid direction")),
        };

        let number: i32 = caps.get(2).unwrap().as_str()
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid number"))?;


        Ok(number * direction)
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid input: {s}"))
    }
}

pub fn part1() -> std::io::Result<()> {
    let mut dial = 50;
    let mut zeroes = 0;

    for line_result in file_to_lines("day1.txt")? {
        let line = line_result?;
        let dif = parse_move(&line)?;
        dial = (dial + dif) % 100;
        if dial == 0 { zeroes += 1 };
    }

    println!("Solution part 1: {}", zeroes);
    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let mut dial = 50;
    let mut zeroes = 0;

    for line_result in file_to_lines("day1.txt")? {
        let line = line_result?;
        let dif = parse_move(&line)?;
        let turns = (dial + dif) / 100;
        if (dial + dif) <= 0 { zeroes += -turns + (if dial > 0 {1} else {0}) } else { zeroes += turns };
        dial = ((dial + dif) % 100 + 100) % 100;
    }

    println!("Solution part 2: {}", zeroes);
    Ok(())
}
