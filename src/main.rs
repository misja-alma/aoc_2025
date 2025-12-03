mod solutions;
mod utils;

use std::time::Instant;
use anyhow::Result;

use solutions::day3::*;

macro_rules! timeit {
    ($expr:expr) => {{
        let start = Instant::now();
        let result = $expr;
        let duration = start.elapsed();
        println!("Time elapsed: {:?}", duration);
        result
    }};
}

fn main() -> Result<()> {
    println!("Part 1:");
    let result1 = timeit!(part1()?);
    println!("Solution: {}", result1);

    println!("Part 2:");
    let result2 = timeit!(part2()?);
    println!("Solution: {}", result2);

    Ok(())
}