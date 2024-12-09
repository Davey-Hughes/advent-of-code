use itertools::Itertools;
use std::{collections::BTreeSet, env, error::Error, fs, process::exit};

fn parse_input(input_string: &str) -> Vec<(usize, usize)> {
    input_string
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.bytes()
                .enumerate()
                .filter(|(_, p)| p == &b'#')
                .map(move |(x, _)| (x, y))
        })
        .collect::<Vec<_>>()
}

fn empty_spaces(galaxies: &[(usize, usize)]) -> Result<Vec<Vec<usize>>, Box<dyn Error>> {
    let mut seen: [BTreeSet<usize>; 2] = [const { BTreeSet::new() }; 2];

    for (x, y) in galaxies {
        seen[0].insert(*x);
        seen[1].insert(*y);
    }

    seen.iter()
        .map(|s| {
            let all = (0..*s.last().ok_or("Empty galaxies")?).collect::<BTreeSet<usize>>();
            Ok(all
                .difference(s)
                .copied()
                .collect::<BTreeSet<usize>>()
                .into_iter()
                .collect::<Vec<_>>())
        })
        .collect::<Result<Vec<_>, _>>()
}

fn distances(galaxies: &mut [(usize, usize)], empty: &[Vec<usize>], expansion: usize) -> usize {
    for (x, y) in &mut *galaxies {
        *x += expansion * empty[0].binary_search(x).unwrap_err();
        *y += expansion * empty[1].binary_search(y).unwrap_err();
    }

    galaxies
        .iter()
        .tuple_combinations()
        .map(|(first, second)| first.0.abs_diff(second.0) + first.1.abs_diff(second.1))
        .sum::<usize>()
}

fn part_1(galaxies: &mut [(usize, usize)], empty: &[Vec<usize>]) -> usize {
    distances(galaxies, empty, 1)
}

fn part_2(galaxies: &mut [(usize, usize)], empty: &[Vec<usize>]) -> usize {
    distances(galaxies, empty, 999_999)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1])?;
    let mut galaxies = parse_input(&contents);
    let empty = empty_spaces(&galaxies)?;

    println!("Part 1: {:?}", part_1(&mut galaxies.clone(), &empty));
    println!("Part 2: {:?}", part_2(&mut galaxies, &empty));

    Ok(())
}
