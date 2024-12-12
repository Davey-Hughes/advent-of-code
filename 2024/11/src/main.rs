use std::{collections::HashMap, env, error::Error, fs, process::exit};

fn parse_input(input_string: &str) -> Result<Vec<u64>, Box<dyn Error>> {
    Ok(input_string
        .split_whitespace()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()?)
}

fn blink_stone(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }

    let len = stone.ilog10() + 1;
    if len % 2 == 0 {
        let half = 10u64.pow(len / 2);
        return vec![stone / half, stone % half];
    }

    vec![stone * 2024]
}

fn num_stones(stones: &[u64], blinks: u64) -> usize {
    let mut stones_map = stones
        .iter()
        .map(|&x| (x, 1))
        .collect::<HashMap<u64, usize>>();

    for _ in 0..blinks {
        stones_map = stones_map
            .iter()
            .flat_map(|stone| blink_stone(*stone.0).into_iter().map(|val| (val, *stone.1)))
            .fold(HashMap::new(), |mut map, (k, v)| {
                *map.entry(k).or_insert(0) += v;
                map
            });
    }

    stones_map.into_values().sum::<usize>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1])?;
    let stones = parse_input(&contents)?;

    println!("Part 1: {:?}", num_stones(&stones, 25));
    println!("Part 2: {:?}", num_stones(&stones, 75));

    Ok(())
}
