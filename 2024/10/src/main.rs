use std::{collections::HashSet, env, error::Error, fs, process::exit};

struct Map {
    map: Vec<Vec<u8>>,
    trailheads: Vec<(usize, usize)>,
}

impl Map {
    fn neighbors(
        &self,
        coords: (usize, usize),
    ) -> impl Iterator<Item = (u8, (usize, usize))> + Clone + '_ {
        [
            (coords.0.checked_sub(1), Some(coords.1)),
            (coords.0.checked_add(1), Some(coords.1)),
            (Some(coords.0), coords.1.checked_sub(1)),
            (Some(coords.0), coords.1.checked_add(1)),
        ]
        .into_iter()
        .filter_map(|(x, y)| Some((x?, y?)))
        .filter(|(x, y)| *x < self.map[0].len() && *y < self.map.len())
        .map(|(x, y)| (self.map[y][x], (x, y)))
    }
}

fn num_trails(map: &Map, pos: (u8, (usize, usize))) -> usize {
    map.neighbors(pos.1)
        .map(|neighbor| {
            if neighbor.0 == pos.0 + 1 {
                if neighbor.0 == b'9' {
                    1
                } else {
                    num_trails(map, neighbor)
                }
            } else {
                0
            }
        })
        .sum()
}

fn reachable_peaks(map: &Map, pos: (u8, (usize, usize))) -> Vec<(usize, usize)> {
    map.neighbors(pos.1)
        .map(|neighbor| {
            if neighbor.0 == pos.0 + 1 {
                if neighbor.0 == b'9' {
                    vec![neighbor.1]
                } else {
                    reachable_peaks(map, neighbor)
                }
            } else {
                vec![]
            }
        })
        .fold(vec![], |mut acc, res| {
            acc.extend(res);
            acc
        })
}

fn parse_input(input_string: &str) -> Map {
    let map = input_string
        .lines()
        .map(|l| l.bytes().map(Into::into).collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let trailheads = map
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter(|(_, &p)| p == b'0')
                .map(move |(x, _)| (x, y))
        })
        .collect::<Vec<(usize, usize)>>();

    Map { map, trailheads }
}

fn part_1(map: &Map) -> usize {
    map.trailheads
        .iter()
        .map(|(x, y)| {
            HashSet::<(usize, usize)>::from_iter(reachable_peaks(map, (b'0', (*x, *y)))).len()
        })
        .sum()
}

fn part_2(map: &Map) -> usize {
    map.trailheads
        .iter()
        .map(|(x, y)| num_trails(map, (b'0', (*x, *y))))
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1])?;
    let map = parse_input(&contents);

    println!("Part 1: {:?}", part_1(&map));
    println!("Part 2: {:?}", part_2(&map));

    Ok(())
}
