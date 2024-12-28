use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    env,
    error::Error,
    fs,
};

fn parse_input(input_string: &str) -> Result<HashMap<&str, Vec<&str>>, Box<dyn Error>> {
    let re = Regex::new(r"(\w+)-(\w+)")?;

    Ok(re
        .captures_iter(input_string)
        .map(|c| c.extract())
        .map(|(_, [first, second])| (first, second))
        .fold(HashMap::new(), |mut acc, s| {
            acc.entry(s.0).or_default().push(s.1);
            acc.entry(s.1).or_default().push(s.0);
            acc
        }))
}

fn part_1(network: &HashMap<&str, Vec<&str>>) -> usize {
    fn bfs<'a>(
        network: &'a HashMap<&'a str, Vec<&'a str>>,
        start: &'a str,
    ) -> BTreeSet<Vec<&'a str>> {
        let mut queue = VecDeque::from([(start, vec![])]);

        let mut res = BTreeSet::new();

        while let Some((node, path)) = queue.pop_front() {
            if path.len() > 3 {
                break;
            }

            if node == start && path.len() == 3 {
                let mut path = path.clone();
                path.sort_unstable();
                res.insert(path);
            }

            if let Some(neighbors) = network.get(node) {
                for neighbor in neighbors {
                    let mut path = path.clone();
                    path.push(node);
                    queue.push_back((neighbor, path));
                }
            }
        }

        res
    }

    network
        .into_par_iter()
        .flat_map(|(k, _)| bfs(network, k))
        .filter(|n| n.iter().any(|x| x.starts_with('t')))
        .fold(BTreeSet::new, |mut acc, x| {
            acc.insert(x);
            acc
        })
        .reduce(BTreeSet::new, |mut acc, x| {
            acc.par_extend(x);
            acc
        })
        .len()
}

fn part_2(network: &HashMap<&str, Vec<&str>>) -> String {
    let network = network
        .iter()
        .map(|(&k, v)| (k, BTreeSet::from_iter([&[k], v.as_slice()].concat())))
        .fold(HashMap::new(), |mut acc, x| {
            acc.insert(x.0, x.1);
            acc
        });

    let mut longest = BTreeSet::new();
    let mut seen = HashSet::new();

    // greedy algorithm for maximum clique
    for &v in network.keys() {
        if seen.insert(v) {
            let mut clique = BTreeSet::from([v]);

            for &n in network.keys() {
                if clique
                    .iter()
                    .map(|&x| network.get(x).unwrap())
                    .all(|s| s.contains(n))
                {
                    clique.insert(n);
                    seen.insert(n);
                }
            }

            if clique.len() > longest.len() {
                longest = clique;
            }
        }
    }

    longest.into_iter().join(",")
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;

    let network = parse_input(&contents)?;

    println!("Part 1: {:?}", part_1(&network));
    println!("Part 2: {}", part_2(&network));

    Ok(())
}
