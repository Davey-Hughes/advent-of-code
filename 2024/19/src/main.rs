use std::{
    collections::{HashMap, HashSet, VecDeque},
    env, fs,
};

use anyhow::Result;

struct Onsen<'a> {
    towels: Vec<&'a str>,
    patterns: Vec<&'a str>,
}

fn parse_input(input_string: &str) -> Result<Onsen> {
    let Some((towels, patterns)) = input_string.split_once("\n\n") else {
        anyhow::bail!("Expected two sections separated by two newlines");
    };

    Ok(Onsen {
        towels: towels.split(',').map(str::trim).collect::<Vec<_>>(),
        patterns: patterns.lines().collect::<Vec<_>>(),
    })
}

fn part_1(onsen: &Onsen) -> usize {
    fn check_pattern(towels: &[&str], pattern: &str) -> bool {
        let mut queue = VecDeque::from([pattern]);
        let mut seen = HashSet::new();

        while let Some(p) = queue.pop_back() {
            if seen.insert(p) {
                if p.is_empty() {
                    return true;
                }

                for t in towels {
                    if let Some(end) = p.strip_prefix(t) {
                        queue.push_back(end);
                    }
                }
            }
        }

        false
    }

    onsen
        .patterns
        .iter()
        .map(|p| check_pattern(&onsen.towels, p))
        .filter(|b| *b)
        .count()
}

fn part_2(onsen: &Onsen) -> usize {
    fn count<'a>(cache: &mut HashMap<&'a str, usize>, towels: &[&str], pattern: &'a str) -> usize {
        if pattern.is_empty() {
            return 1;
        }

        if let Some(item) = cache.get(pattern) {
            return *item;
        }

        towels
            .iter()
            .map(|t| {
                if let Some(end) = pattern.strip_prefix(t) {
                    let res = count(cache, towels, end);
                    cache.insert(end, res);
                    res
                } else {
                    0
                }
            })
            .sum()
    }

    let mut cache = HashMap::new();

    onsen
        .patterns
        .iter()
        .map(|p| count(&mut cache, &onsen.towels, p))
        .sum()
}

fn main() -> Result<()> {
    let contents =
        fs::read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;

    let onsen = parse_input(&contents)?;

    println!("Part 1: {:?}", part_1(&onsen));
    println!("Part 2: {:?}", part_2(&onsen));

    Ok(())
}
