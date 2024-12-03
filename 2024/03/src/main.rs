use std::{env, error::Error, fs, process::exit};

use regex::Regex;

fn part_1(memory: &str) -> Result<u64, Box<dyn Error>> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)")?;

    Ok(re
        .captures_iter(memory)
        .map(|c| c.extract())
        .map(|(_, [left, right])| -> Result<u64, Box<dyn Error>> {
            Ok(left.parse::<u64>()? * right.parse::<u64>()?)
        })
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum::<u64>())
}

fn part_2(memory: &str) -> Result<u64, Box<dyn Error>> {
    let re = Regex::new(r"(mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\))")?;

    let mut sum = 0;
    let mut enabled = true;

    let all_captures = re.captures_iter(memory);
    for captures in all_captures {
        let capture_name = captures.get(0).ok_or("Invalid get(0) on capture")?.as_str();

        if capture_name.starts_with("do()") {
            enabled = true;
        } else if capture_name.starts_with("don't()") {
            enabled = false;
        } else if enabled {
            sum += (2..=3)
                .map(|i| -> Result<u64, Box<dyn Error>> {
                    Ok(captures
                        .get(i)
                        .ok_or("Invalid get(0) on capture")?
                        .as_str()
                        .parse::<u64>()?)
                })
                .collect::<Result<Vec<_>, _>>()?
                .iter()
                .product::<u64>();
        }
    }

    Ok(sum)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1])?;

    println!("Part 1: {:?}", part_1(&contents)?);
    println!("Part 2: {:?}", part_2(&contents)?);

    Ok(())
}
