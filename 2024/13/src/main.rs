use std::{env, error::Error, fs};

use regex::Regex;

type F = fraction::Fraction;

#[derive(Debug)]
struct Machine {
    a: (F, F),
    b: (F, F),
    prize: (F, F),
}

fn parse_input(input_string: &str) -> Result<Vec<Machine>, Box<dyn Error>> {
    let re_digits = Regex::new(r"[\d]+")?;

    input_string
        .split("\n\n")
        .map(|machine| -> Result<Machine, Box<dyn Error>> {
            let m = machine
                .lines()
                .map(|l| {
                    re_digits
                        .find_iter(l)
                        .map(|c| c.as_str().parse::<F>())
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<_>, _>>()?;

            Ok(Machine {
                a: (m[0][0], m[0][1]),
                b: (m[1][0], m[1][1]),
                prize: (m[2][0], m[2][1]),
            })
        })
        .collect::<Result<Vec<_>, _>>()
}

#[allow(clippy::many_single_char_names)]
fn solve(machines: &[Machine], prize_extra: u64) -> u64 {
    machines
        .iter()
        .filter_map(|machine| {
            let prize = (machine.prize.0 + prize_extra, machine.prize.1 + prize_extra);
            let x = machine.a.0 - machine.a.1;
            let y = machine.b.1 - machine.b.0;
            let z = prize.1 - prize.0;

            let a_coeff = machine.a.0 + ((machine.b.0 * x) / y);
            let c = prize.0 - ((machine.b.0 * z) / y);

            let a = c / a_coeff;
            let b = (prize.0 - (machine.a.0 * a)) / machine.b.0;

            if a.numer()? % a.denom()? != 0u64 || b.numer()? % b.denom()? != 0u64 {
                return None;
            }

            Some((*a.numer()?, *b.numer()?))
        })
        .map(|(a, b)| a * 3 + b)
        .sum::<u64>()
}

fn part_1(machines: &[Machine]) -> u64 {
    solve(machines, 0)
}

#[allow(clippy::many_single_char_names)]
fn part_2(machines: &[Machine]) -> u64 {
    solve(machines, 10_000_000_000_000_u64)
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;

    let machines = parse_input(&contents)?;

    println!("Part 1: {:?}", part_1(&machines));
    println!("Part 2: {:?}", part_2(&machines));

    Ok(())
}
