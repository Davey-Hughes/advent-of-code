use std::{env, error::Error, fs};

mod part1;
mod part2;

fn main() -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;

    println!("Part 1: {:?}", part1::part_1(&contents)?);
    println!("Part 2: {:?}", part2::part_2(&contents)?);

    Ok(())
}
