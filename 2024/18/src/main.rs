use std::{
    collections::{HashSet, VecDeque},
    env, fs,
};

use anyhow::{Context, Result};

enum SearchAlgorithm {
    BFS,
    DFS,
}

fn parse_input(input_string: &str) -> Result<Vec<(u64, u64)>> {
    input_string
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(',').context("Invalid input")?;

            Ok((left.parse::<u64>()?, right.parse::<u64>()?))
        })
        .collect::<Result<Vec<_>>>()
}

fn neighbors(coords: (u64, u64), max: (u64, u64)) -> impl Iterator<Item = (u64, u64)> + Clone {
    [
        (coords.0.checked_sub(1), Some(coords.1)),
        (coords.0.checked_add(1), Some(coords.1)),
        (Some(coords.0), coords.1.checked_sub(1)),
        (Some(coords.0), coords.1.checked_add(1)),
    ]
    .into_iter()
    .filter_map(|(x, y)| Some((x?, y?)))
    .filter(move |(x, y)| *x <= max.0 && *y <= max.1)
}

fn search(
    byte_set: &HashSet<&(u64, u64)>,
    start: (u64, u64),
    end: (u64, u64),
    algorithm: &SearchAlgorithm,
) -> Option<usize> {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from([(start, 0)]);

    loop {
        let pop = match algorithm {
            SearchAlgorithm::BFS => queue.pop_front(),
            SearchAlgorithm::DFS => queue.pop_back(),
        };

        if let Some((current, steps)) = pop {
            if current == end {
                return Some(steps);
            }
            for next in neighbors(current, end) {
                if !byte_set.contains(&next) && !seen.contains(&next) {
                    seen.insert(next);
                    queue.push_back((next, steps + 1));
                }
            }
        } else {
            break;
        }
    }

    None
}

fn part_1(falling_bytes: &[(u64, u64)], num_fallen: usize) -> usize {
    let byte_set = falling_bytes[..num_fallen]
        .iter()
        .fold(HashSet::new(), |mut acc, x| {
            acc.insert(x);
            acc
        });

    let max = falling_bytes
        .iter()
        .fold((0, 0), |acc, x| (acc.0.max(x.0), acc.1.max(x.1)));

    search(&byte_set, (0, 0), max, &SearchAlgorithm::BFS).expect("End not found")
}

fn part_2(falling_bytes: &[(u64, u64)]) -> Option<String> {
    let mut byte_set = HashSet::new();

    let max = falling_bytes
        .iter()
        .fold((0, 0), |acc, x| (acc.0.max(x.0), acc.1.max(x.1)));

    for byte in falling_bytes {
        byte_set.insert(byte);

        if search(&byte_set, (0, 0), max, &SearchAlgorithm::DFS).is_none() {
            return Some(format!("{},{}", byte.0, byte.1).to_string());
        }
    }

    None
}

fn main() -> Result<()> {
    let contents =
        fs::read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;

    let falling_bytes = parse_input(&contents)?;

    println!("Part 1: {:?}", part_1(&falling_bytes, 1024));
    println!("Part 2: {}", part_2(&falling_bytes).unwrap());

    Ok(())
}
