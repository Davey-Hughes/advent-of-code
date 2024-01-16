use rayon::prelude::*;
use std::{collections::HashMap, env, fs, ops::Range, process::exit};

use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "main.pest"]
struct Day5Parser;

#[derive(Debug)]
struct AlmanacMapping {
    mapping: HashMap<Range<usize>, (usize, usize)>,
}

fn parse_map_block(block: Pairs<'_, Rule>) -> AlmanacMapping {
    let mut mapping = HashMap::new();
    for rule in block {
        if rule.as_rule() == Rule::mapping {
            let mut values: Vec<usize> = vec![];
            for value in rule.into_inner() {
                values.push(value.as_str().parse().unwrap());
            }

            mapping.insert(values[1]..values[1] + values[2], (values[0], values[1]));
        }
    }

    AlmanacMapping { mapping }
}

fn parse_seed_mappings<S: AsRef<str>>(file_string: S) -> (Vec<usize>, Vec<AlmanacMapping>) {
    let file = Day5Parser::parse(Rule::file, file_string.as_ref()).expect("cannot read file");

    let mut seeds: Vec<usize> = vec![];
    let mut mappings: Vec<AlmanacMapping> = vec![];

    for rule in file {
        match rule.as_rule() {
            Rule::seed_line => {
                for seed in rule.into_inner() {
                    if seed.as_rule() == Rule::seed {
                        seeds.push(seed.as_str().parse().unwrap())
                    }
                }
            }
            Rule::map_block => {
                let mapping = parse_map_block(rule.into_inner());
                mappings.push(mapping);
            }
            _ => (),
        }
    }

    (seeds, mappings)
}

fn find_location(seed: usize, almanac: &[AlmanacMapping]) -> usize {
    let mut source = seed;

    for almanac_mapping in almanac.iter() {
        for (k, v) in almanac_mapping.mapping.iter() {
            if k.contains(&source) {
                source = v.0 + (source - v.1);
                break;
            }
        }
    }

    source
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1]).expect("Should have been able to read the file");

    let (seeds, almanac) = parse_seed_mappings(contents);

    println!(
        "Part 1: {}",
        seeds
            .iter()
            .map(|s| find_location(*s, &almanac))
            .min()
            .unwrap()
    );

    let mut seed_ranges = vec![];

    for i in (0..seeds.len()).step_by(2) {
        seed_ranges.push(seeds[i]..seeds[i] + seeds[i + 1]);
    }

    // brute-force with parallelism by rayon
    println!(
        "Part 2: {}",
        seed_ranges
            .par_iter()
            .flat_map(|it| it.clone())
            .map(|s| find_location(s, &almanac))
            .min()
            .unwrap()
    );
}
