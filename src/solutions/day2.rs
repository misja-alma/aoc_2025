use anyhow::Result;
use std::fmt::Display;
use crate::utils::*;

fn int_to_digits(x: u64) -> Vec<u32> {
    let mut digits = Vec::new();
    let mut n = x;

    if n == 0 {
        digits.push(0);
    } else {
        while n > 0 {
            digits.push((n % 10) as u32);
            n /= 10;
        }
        digits.reverse();
    }

    digits
}

fn first_palindrome_of_even_length(length: usize) -> u64 {
    10_u64.pow((length-1) as u32) + 10_u64.pow(length as u32 / 2 - 1)
}

fn find_palindrome_ge(target: u64) -> u64 {
    // if nr digits odd; take first next even (i.e. 111..)
    // if digit even; take half or nr
    let digits = int_to_digits(target);
    if digits.len() % 2 != 0 {
        first_palindrome_of_even_length(digits.len() + 1)
    } else {
        let lower_half = digits_to_int(&digits[0..digits.len()/2]);
        let upper_half = digits_to_int(&digits[digits.len()/2..]);
        if upper_half <= lower_half {
            let new_digits = digits[0..digits.len()/2].iter().as_slice();
            let new_result = [new_digits, new_digits].concat();
            digits_to_int(&new_result)
        } else {
            next_palindrome(target)
        }
    }
}

fn next_palindrome(from: u64) -> u64 {
    let digits = int_to_digits(from);
    if digits.len() % 2 != 0 {
        first_palindrome_of_even_length(digits.len() / 2 + 1)
    } else {
        let current = digits_to_int(&digits[0..digits.len()/2]);
        let maybe_result = current + 1;
        let new_digits = int_to_digits(maybe_result);
        if new_digits.len() * 2 > digits.len() {
            first_palindrome_of_even_length(digits.len() + 2)
        } else {
            let new_result = [new_digits.as_slice(), new_digits.as_slice()].concat();
            digits_to_int(&new_result)
        }
    }
}

fn invalid_sum(lower: u64, upper: u64) -> u64 {
    let mut invalid = find_palindrome_ge(lower);
    let mut total = 0;
    while invalid <= upper {
        total += invalid;
        invalid = next_palindrome(invalid);
    }
    total
}

pub fn part1() ->  Result<impl Display> {
    let line = file_as_string("day2.txt")?;
    let mut total = 0;

    for range in line.trim().split(',') {
        let bounds: Vec<&str> = range.split('-').collect();
        let lower = bounds[0].parse()?;
        let higher = bounds[1].parse()?;
        total += invalid_sum(lower, higher);
    }

    Ok(total)
}

fn is_repeating(n: u64) -> bool {
    let digits = int_to_digits(n);
    if digits.len() < 2 {
        return false;
    }

    (1..=digits.len()/2)
        .filter(|d| digits.len() % d == 0)
        .find(|d| {
            let mut chunks = digits.chunks(*d);
            let chunk1 = chunks.next().unwrap();
            chunks.all(|chunk| chunk == chunk1)
        })
        .is_some()
}

fn all_invalid_sum(lower: u64, upper: u64) -> u64 {
    (lower..=upper).filter(|&n| is_repeating(n)).sum()
}

pub fn part2() -> Result<impl Display> {
    let line = file_as_string("day2.txt")?;
    let mut total = 0;

    // Just brute force this time, turned out to be fast enough
    for range in line.trim().split(',') {
        let bounds: Vec<&str> = range.split('-').collect();
        let lower = bounds[0].parse()?;
        let higher = bounds[1].parse()?;
        total += all_invalid_sum(lower, higher);
    }

    Ok(total)
}
