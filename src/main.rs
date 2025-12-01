mod solutions;
mod utils;

use anyhow::Result;
use solutions::day1::*;

fn main() -> Result<()> {
    part1()?;

    part2()
}