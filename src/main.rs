mod solutions;
mod utils;

use anyhow::Result;
use solutions::day2::*;

fn main() -> Result<()> {
    part1()?;

    part2()
}