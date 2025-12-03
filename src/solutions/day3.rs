use std::fmt::Display;
use anyhow::Result;
use crate::utils::*;

fn highest_combo(s: &str) -> u32 {
    let chars = s.as_bytes();

    let mut index = 0;
    let mut highest = chars[0];
    let mut i = 1;
    while i < chars.len() - 1 && highest < b'9' {
        if chars[i] > highest {
            highest = chars[i];
            index = i;
        }
        i += 1;
    }

    let mut second_highest = chars[index + 1];
    let mut i = index + 1;
    while i < chars.len() && second_highest < b'9' {
        if chars[i] > second_highest {
            second_highest = chars[i];
        }
        i += 1;
    }

    10 * (highest - b'0') as u32 + (second_highest - b'0') as u32
}

pub fn part1() -> Result<impl Display> {
    let mut result = 0;

    for line in file_to_lines("day3.txt")? {
        result += highest_combo(line?.as_str())
    }

    Ok(result)
}

fn highest_combo_n(s: &str, length: u32) -> u64 {
    let chars = s.as_bytes();

    let mut result: Vec<u8> = Vec::with_capacity(length as usize);
    let mut next_index = 0;

    for n in (1..=length).rev() {
        let mut index = next_index;
        let mut highest = chars[index];
        let mut i = index + 1;
        while i <= chars.len() - n as usize && highest < b'9' {
            if chars[i] > highest {
                highest = chars[i];
                index = i;
            }
            i += 1;
        }

        next_index = index + 1;
        result.push(highest - b'0');
    }

    digits_to_int(&result)
}

pub fn part2() -> Result<impl Display>  {
    let mut result = 0;

    for line in file_to_lines("day3.txt")? {
        result += highest_combo_n(line?.as_str(), 12) 
    }

    Ok(result)
}
