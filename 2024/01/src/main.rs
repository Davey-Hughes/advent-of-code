use std::{collections::HashMap, env, error::Error, fs, process::exit};

fn parse_input(file_string: &str) -> (Vec<u64>, Vec<u64>) {
    file_string
        .lines()
        .map(|s| {
            let mut ids = s.split_whitespace();
            (
                ids.next().unwrap().parse::<u64>().unwrap(),
                ids.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .unzip()
}

fn part_1(lists: &mut (Vec<u64>, Vec<u64>)) -> u64 {
    lists.0.sort_unstable();
    lists.1.sort_unstable();

    lists
        .0
        .iter()
        .zip(lists.1.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<u64>()
}

fn part_2(lists: &mut (Vec<u64>, Vec<u64>)) -> u64 {
    let list_2_frequencies = lists.1.iter().fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|freq| *freq += 1).or_insert(1);
        map
    });

    lists
        .0
        .iter()
        .map(|id| list_2_frequencies.get(id).unwrap_or(&0) * id)
        .sum::<u64>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1])?;
    let mut lists = parse_input(&contents);

    println!("Part 1: {:?}", part_1(&mut lists.clone()));
    println!("Part 2: {:?}", part_2(&mut lists));

    Ok(())
}
