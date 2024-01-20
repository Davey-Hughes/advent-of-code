#![feature(array_windows)]
use std::{env, error::Error, fs, process::exit};

fn parse_input<S: AsRef<str>>(file_string: &S) -> impl Iterator<Item = Vec<i64>> + Clone + '_ {
    file_string
        .as_ref()
        .lines()
        .map(|s| s.split(' '))
        .map(|x| x.map(|x| x.parse::<i64>().unwrap()).collect())
}

fn predict_next(history: &[i64]) -> i64 {
    let next_vec = history
        .array_windows()
        .map(|[x, y]| y - x)
        .collect::<Vec<_>>();

    // base case
    if next_vec.iter().all(|&x| x == 0) {
        return *history.last().unwrap();
    }

    history.last().unwrap() + predict_next(&next_vec)
}

fn predict_prev(history: &[i64]) -> i64 {
    let next_vec = history
        .array_windows()
        .map(|[x, y]| y - x)
        .collect::<Vec<_>>();

    if next_vec.iter().all(|&x| x == 0) {
        return history[0];
    }

    history[0] - predict_prev(&next_vec)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1])?;

    let lines = parse_input(&contents);
    println!(
        "Part 1: {}",
        lines.clone().map(|v| predict_next(&v)).sum::<i64>()
    );

    println!("Part 1: {}", lines.map(|v| predict_prev(&v)).sum::<i64>());

    Ok(())
}
