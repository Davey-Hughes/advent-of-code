use std::{env, error::Error, fs::read_to_string};

fn parse_input(input: &str) -> Option<Vec<Vec<u64>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| -> Option<u64> { Some(u64::from(c.to_digit(10)?)) })
                .collect::<Option<Vec<_>>>()
        })
        .collect::<Option<Vec<_>>>()
}

fn argmax<'a, T>(bank: T) -> (usize, u64)
where
    T: Iterator<Item = &'a u64>,
{
    bank.enumerate().fold((0, 0), |mut acc, (i, &x)| {
        if x > acc.1 {
            acc = (i, x);
        }
        acc
    })
}

fn joltage(bank: &[u64], n: usize) -> u64 {
    (0..n)
        .fold((0, vec![]), |mut acc, i| {
            let cur = argmax(bank.iter().take(bank.len() - (n - i) + 1).skip(acc.0));
            acc.0 = acc.0 + cur.0 + 1;
            acc.1.push(cur.1);

            acc
        })
        .1
        .iter()
        .rev()
        .fold((1, 0), |acc, &x| (acc.0 * 10, acc.1 + acc.0 * x))
        .1
}

fn part_1(banks: &[Vec<u64>]) -> u64 {
    banks.iter().map(|bank| joltage(bank, 2)).sum()
}

fn part_2(banks: &[Vec<u64>]) -> u64 {
    banks.iter().map(|bank| joltage(bank, 12)).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;

    let banks = parse_input(&contents).ok_or("Invalid input")?;
    println!("Part 1: {:?}", part_1(&banks));
    println!("Part 2: {:?}", part_2(&banks));

    Ok(())
}
