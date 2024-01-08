use std::{env, fs, process::exit, str};

use aho_corasick::AhoCorasick;

fn replace_digits(line: &str) -> String {
    let patterns = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let replace = &[
        "o1e", "t2o", "t3e", "f4r", "f5e", "s6x", "s7n", "e8t", "n9e",
    ];

    let mut result = line.to_string();

    let ac = AhoCorasick::new(patterns).unwrap();

    while let Some(m) = ac.find_overlapping_iter(result.as_str()).next() {
        result.replace_range(m.start()..m.end(), replace[m.pattern().as_usize()]);
    }

    result
}

fn extract_number<S: AsRef<str>>(line: S) -> i32 {
    let digits = line
        .as_ref()
        .chars()
        .filter(|x| x.is_ascii_digit())
        .collect::<Vec<char>>();

    let sum = (digits.first().unwrap().to_string() + digits.last().unwrap().to_string().as_str())
        .parse::<i32>()
        .unwrap();

    sum
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1]).expect("Should have been able to read the file");

    println!(
        "Part 1: {}",
        contents.lines().map(extract_number).sum::<i32>()
    );

    println!(
        "Part 2: {}",
        contents
            .lines()
            .map(replace_digits)
            .map(extract_number)
            .sum::<i32>()
    );
}
