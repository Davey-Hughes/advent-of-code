use std::{env, error::Error, fs::read_to_string, ops::RangeInclusive};

use itertools::Itertools;

fn parse_input(input: &str) -> Result<Vec<RangeInclusive<u64>>, Box<dyn Error>> {
    fn parse_int<'a, I, T>(iter: &mut I) -> Result<T, Box<dyn Error>>
    where
        I: Iterator<Item = &'a str>,
        T: std::str::FromStr,
        T::Err: Error + 'static,
    {
        iter.next()
            .ok_or("Invalid input")?
            .trim()
            .parse::<T>()
            .map_err(Into::into)
    }

    input
        .split(',')
        .map(|range| -> Result<RangeInclusive<u64>, Box<dyn Error>> {
            let mut id_range = range.split('-');
            let first = parse_int(&mut id_range)?;
            let second = parse_int(&mut id_range)?;
            Ok(first..=second)
        })
        .collect::<Result<Vec<RangeInclusive<_>>, Box<dyn Error>>>()
}

fn part_1(ranges: &mut [RangeInclusive<u64>]) -> u64 {
    fn invalid_id<T>(x: &T) -> bool
    where
        T: ToString,
    {
        let input_str = x.to_string();
        let (begin, end) = input_str.split_at(input_str.len() / 2);
        begin == end
    }

    ranges
        .iter_mut()
        .map(|range| range.filter(invalid_id).sum::<u64>())
        .sum()
}

fn part_2(ranges: &mut [RangeInclusive<u64>]) -> u64 {
    fn divisors(x: usize) -> impl Iterator<Item = usize> {
        (1..x).filter(move |i| x.is_multiple_of(*i))
    }

    fn invalid_id(x: u64) -> bool {
        let input_str = x.to_string();

        divisors(input_str.len())
            .find(|&d| {
                input_str
                    .chars()
                    .chunks(d)
                    .into_iter()
                    .map(std::iter::Iterator::collect::<String>)
                    .all_equal()
            })
            .is_some()
    }

    ranges
        .iter_mut()
        .map(|range| range.filter(|&i| invalid_id(i)).sum::<u64>())
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;
    let mut ranges = parse_input(&contents)?;

    println!("Part 1: {}", part_1(&mut ranges.clone()));
    println!("Part 2: {}", part_2(&mut ranges));

    Ok(())
}
