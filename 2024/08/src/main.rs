use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    env,
    error::Error,
    fs,
    process::exit,
};

type Antennae = HashMap<char, Vec<(isize, isize)>>;

struct Map {
    antennae: Antennae,
    bounds: (isize, isize),
}

fn parse_input(input_string: &str) -> Result<Map, Box<dyn Error>> {
    let mut antennae: Antennae = HashMap::new();

    let lines = input_string.lines().collect::<Vec<_>>();

    let bounds = (
        isize::try_from(lines.len())?,
        isize::try_from(lines[0].len())?,
    );

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennae
                    .entry(c)
                    .or_default()
                    .push((isize::try_from(x)?, isize::try_from(y)?));
            }
        }
    }

    Ok(Map { antennae, bounds })
}

fn part_1(map: &Map) -> usize {
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for coords in map.antennae.values() {
        for (left, right) in coords
            .iter()
            .cartesian_product(coords)
            .filter(|(first, second)| first != second)
        {
            let antinode = (right.0 - (left.0 - right.0), (right.1 - (left.1 - right.1)));

            if (0..map.bounds.0).contains(&antinode.0) && (0..map.bounds.1).contains(&antinode.1) {
                antinodes.insert(antinode);
            }
        }
    }

    antinodes.len()
}

fn part_2(map: &Map) -> usize {
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for coords in map.antennae.values() {
        for (left, right) in coords.iter().tuple_combinations() {
            antinodes.insert(*left);

            let slope = (left.0 - right.0, left.1 - right.1);

            for n in 1..map.bounds.0 {
                let antinode = (left.0 + slope.0 * n, left.1 + slope.1 * n);

                if !(0..map.bounds.0).contains(&antinode.0)
                    || !(0..map.bounds.1).contains(&antinode.1)
                {
                    break;
                }

                antinodes.insert(antinode);
            }

            for n in 1..map.bounds.0 {
                let antinode = (left.0 - slope.0 * n, left.1 - slope.1 * n);

                if !(0..map.bounds.0).contains(&antinode.0)
                    || !(0..map.bounds.1).contains(&antinode.1)
                {
                    break;
                }

                antinodes.insert(antinode);
            }
        }
    }

    antinodes.len()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1])?;
    let antennae = parse_input(&contents)?;

    println!("Part 1: {:?}", part_1(&antennae));
    println!("Part 2: {:?}", part_2(&antennae));

    Ok(())
}
