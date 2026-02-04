use std::{env, error::Error, fs::read_to_string, ops::RangeInclusive};

#[derive(Debug)]
struct Input {
    ingredient_ids: Vec<RangeInclusive<u64>>,
    available_ids: Vec<u64>,
}

fn parse_input(input: &str) -> Result<Input, Box<dyn Error>> {
    let mut parts = input.split("\n\n");
    let first = parts.next().ok_or("First part not found")?;
    let second = parts.next().ok_or("Second part not found")?;

    let ingredient_ids = first
        .lines()
        .map(|line| -> Result<_, Box<dyn Error>> {
            let mut l = line.split('-');
            let x = l
                .next()
                .ok_or("Error parsing ingredient ids")?
                .parse::<u64>()?;
            let y = l
                .next()
                .ok_or("Error parsing ingredient ids")?
                .parse::<u64>()?;

            Ok(x..=y)
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

    let available_ids = second
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    Ok(Input {
        ingredient_ids,
        available_ids,
    })
}

fn merge_ranges(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> Option<RangeInclusive<u64>> {
    // b is entirely within a
    if a.contains(b.start()) && a.contains(b.end()) {
        return Some(a.clone());
    }

    // a is entirely within b
    if b.contains(a.start()) && b.contains(a.end()) {
        return Some(b.clone());
    }

    // a begins before b but ends in the middle of b
    if a.end() >= b.start() && a.start() < b.start() {
        return Some(*a.start()..=*b.end());
    }

    // b begins before a but ends in the middle of a
    if b.end() >= a.start() && b.start() < a.start() {
        return Some(*b.start()..=*a.end());
    }

    // the ranges don't overlap
    None
}

fn part_1(input: &Input) -> usize {
    input
        .available_ids
        .iter()
        .filter(|a_id| input.ingredient_ids.iter().any(|i_id| i_id.contains(a_id)))
        .count()
}

fn part_2(input: &Input) -> u64 {
    let mut ranges = input.ingredient_ids.clone();
    ranges.sort_by_key(|r| *r.start());

    let mut merged = vec![];

    for range in ranges {
        if let Some(last) = merged.last_mut() {
            if let Some(new_range) = merge_ranges(last, &range) {
                *last = new_range;
            } else {
                merged.push(range);
            }
        } else {
            merged.push(range);
        }
    }

    merged.iter().map(|r| r.end() - r.start() + 1).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;
    let input = parse_input(&contents)?;

    println!("Part 1: {:?}", part_1(&input));
    println!("Part 2: {:?}", part_2(&input));

    Ok(())
}
