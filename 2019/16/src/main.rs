use std::{env, fs, iter, ops::Rem};

use anyhow::{Context, Result};
use itertools::Itertools;

fn parse_input(input_string: &str) -> Option<Vec<i64>> {
    Some(
        input_string
            .trim()
            .chars()
            .map(|c| c.to_digit(10))
            .collect::<Option<Vec<_>>>()?
            .iter()
            .map(|&d| i64::from(d))
            .collect::<Vec<_>>(),
    )
}

fn phase(input: &[i64]) -> Vec<i64> {
    (1..=input.len())
        .map(|i| {
            let mut base = iter::repeat(
                [0, 1, 0, -1]
                    .into_iter()
                    .flat_map(|n| std::iter::repeat(n).take(i))
                    .collect::<Vec<_>>(),
            )
            .flatten()
            .skip(1);

            input
                .iter()
                .map(|digit| digit * base.next().unwrap())
                .sum::<i64>()
                .rem(10)
                .abs()
        })
        .collect::<Vec<_>>()
}

fn part_1(mut input: Vec<i64>) -> String {
    for _ in 0..100 {
        input = phase(&input);
    }

    input[..8].iter().join("")
}

fn part_2(mut input: Vec<i64>) -> String {
    let offset = usize::try_from(input[..7].iter().fold(0, |acc, e| acc * 10 + e)).unwrap();
    input = iter::repeat(input.iter().copied())
        .take(10000)
        .flatten()
        .collect::<Vec<_>>();

    for _ in 0..100 {
        let mut partial_sum = input[offset..].iter().sum::<i64>();

        input[offset..].iter_mut().for_each(|k| {
            let temp = partial_sum;
            partial_sum -= *k;

            *k = temp.rem(10).abs();
        });
    }

    input[offset..offset + 8].iter().join("")
}

fn main() -> Result<()> {
    let contents =
        fs::read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;

    let input = parse_input(&contents).context("Failed to parse input")?;

    println!("Part 1: {}", part_1(input.clone()));
    println!("Part 2: {}", part_2(input));

    Ok(())
}
