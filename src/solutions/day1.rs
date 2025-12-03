use std::fmt::Display;
use regex::Regex;
use anyhow::{Result, anyhow};
use crate::utils::*;

fn parse_move(s: &String) -> Result<i32> {
    let re = Regex::new(r"^([LR])([1-9]\d*)$").unwrap();

    if let Some(caps) = re.captures(s) {
        let direction = match caps.get(1).unwrap().as_str() {
            "L" => -1,
            "R" => 1,
            _ => return Err(anyhow!("Invalid direction")),
        };

        let number: i32 = caps.get(2).unwrap().as_str()
            .parse()?;

        Ok(number * direction)
    } else {
        Err(anyhow!("Invalid input: {s}"))
    }
}

pub fn part1() -> Result<impl Display> {
    let mut dial = 50;
    let mut zeroes = 0;

    for line in file_to_lines("day1.txt")? {
        let line = line?;
        let dif = parse_move(&line)?;
        dial = (dial + dif) % 100;
        if dial == 0 { zeroes += 1 };
    }
    
    Ok(zeroes)
}

pub fn part2() -> Result<impl Display>  {
    let mut dial = 50;
    let mut zeroes = 0;

    for line in file_to_lines("day1.txt")? {
        let line = line?;
        let dif = parse_move(&line)?;
        let turns = (dial + dif) / 100;
        if (dial + dif) <= 0 { zeroes += -turns + (if dial > 0 {1} else {0}) } else { zeroes += turns };
        dial = ((dial + dif) % 100 + 100) % 100;
    }
    
    Ok(zeroes)
}
