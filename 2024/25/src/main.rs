use std::{env, error::Error, fs};

fn parse_input(input_string: &str) -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
    let mut locks = vec![];
    let mut keys = vec![];

    for section in input_string.split("\n\n") {
        let x = section
            .lines()
            .flat_map(|line| line.bytes().enumerate().filter(|(_, b)| *b == b'#'))
            .fold(vec![-1; 5], |mut acc: Vec<i64>, (i, _)| {
                acc[i] += 1;
                acc
            });

        if section.bytes().next() == Some(b'#') {
            locks.push(x);
        } else {
            keys.push(x);
        }
    }

    (locks, keys)
}

fn part_1(locks: &[Vec<i64>], keys: &[Vec<i64>]) -> i64 {
    keys.iter()
        .flat_map(|key| {
            locks
                .iter()
                .map(|lock| i64::from(key.iter().zip(lock.iter()).all(|(a, b)| a + b <= 5)))
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;

    let (locks, keys) = parse_input(&contents);

    println!("Part 1: {:?}", part_1(&locks, &keys));

    Ok(())
}
