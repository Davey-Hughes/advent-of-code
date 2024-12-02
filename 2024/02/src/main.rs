use std::{env, error::Error, fs, process::exit};

fn parse_input(file_string: &str) -> Result<Vec<Vec<u64>>, Box<dyn Error>> {
    Ok(file_string
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(str::parse::<u64>)
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?)
}

fn part_1(reports: &[Vec<u64>]) -> u64 {
    fn safety(report: &[u64]) -> bool {
        let increasing = report[0] < report[1];

        for levels in report.windows(2) {
            let diff = levels[0].abs_diff(levels[1]);

            if !(1..=3).contains(&diff) {
                return false;
            }

            if (increasing && levels[0] > levels[1]) || (!increasing && levels[0] < levels[1]) {
                return false;
            }
        }

        true
    }

    reports
        .iter()
        .map(|x| u64::from(safety(x)))
        .reduce(|acc, e| e + acc)
        .unwrap()
}

fn part_2(reports: &[Vec<u64>]) -> u64 {
    fn check_level(report: &[u64]) -> bool {
        let increasing = report[0] < report[1];

        for levels in report.windows(2) {
            let diff = levels[0].abs_diff(levels[1]);

            if !(1..=3).contains(&diff)
                || ((increasing && levels[0] > levels[1]) || (!increasing && levels[0] < levels[1]))
            {
                return false;
            }
        }

        true
    }

    fn safety(report: &[u64]) -> bool {
        for trial in 0..report.len() {
            let mut report = report.to_vec();
            report.remove(trial);

            if check_level(&report) {
                return true;
            }
        }

        false
    }

    reports
        .iter()
        .map(|x| u64::from(safety(x)))
        .reduce(|acc, e| e + acc)
        .unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1])?;
    let reports = parse_input(&contents)?;

    println!("Part 1: {:?}", part_1(&reports));
    println!("Part 2: {:?}", part_2(&reports));

    Ok(())
}
