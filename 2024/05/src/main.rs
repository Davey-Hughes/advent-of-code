use std::{collections::HashMap, env, error::Error, fs, process::exit};

type Rules = HashMap<u64, Vec<u64>>;
type Updates = Vec<Vec<u64>>;

fn parse_input(file_string: &str) -> Result<(Rules, Updates), Box<dyn Error>> {
    let mut rules: Rules = HashMap::new();

    let (rules_raw, updates_raw) = file_string
        .split_once("\n\n")
        .ok_or("Invalid input file format")?;

    for line in rules_raw.lines() {
        let rule = line
            .split("|")
            .map(str::parse::<u64>)
            .collect::<Result<Vec<_>, _>>()?;

        rules.entry(rule[0]).or_default().push(rule[1]);
    }

    let updates = updates_raw
        .lines()
        .map(|l| {
            l.split(',')
                .map(str::parse::<u64>)
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok((rules, updates))
}

fn validate_line(rules: &Rules, line: &[u64]) -> bool {
    for (i, num) in line.iter().enumerate() {
        if let Some(after) = rules.get(num) {
            for n in &line[..i] {
                if after.contains(n) {
                    return false;
                }
            }
        }
    }

    true
}

fn part_1(rules: &Rules, updates: &Updates) -> u64 {
    updates
        .iter()
        .filter(|l| validate_line(rules, l))
        .map(|l| l[l.len() / 2])
        .sum::<u64>()
}

fn part_2(rules: &Rules, updates: &Updates) -> u64 {
    updates
        .iter()
        .filter(|l| !validate_line(rules, l))
        .map(|l| {
            let mut temp = l.clone();

            temp.sort_by(|a, b| {
                if let Some(v) = rules.get(a) {
                    if v.contains(b) {
                        return std::cmp::Ordering::Less;
                    }
                } else if let Some(v) = rules.get(b) {
                    if v.contains(a) {
                        return std::cmp::Ordering::Greater;
                    }
                }

                std::cmp::Ordering::Equal
            });

            temp
        })
        .map(|l| l[l.len() / 2])
        .sum::<u64>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1])?;
    let (rules, updates) = parse_input(&contents)?;

    println!("Part 1: {:?}", part_1(&rules, &updates));
    println!("Part 2: {:?}", part_2(&rules, &updates));

    Ok(())
}
