use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    error::Error,
    fs,
    process::exit,
};

fn parse_input(file_string: &str) -> Result<BTreeMap<&str, Vec<&str>>, Box<dyn Error>> {
    let mut map: BTreeMap<&str, Vec<&str>> = BTreeMap::new();

    file_string
        .lines()
        .map(|s| s.split(')'))
        .map(|mut x| -> Option<()> {
            map.entry(x.next()?).or_default().push(x.next()?);
            Some(())
        })
        .collect::<Option<_>>()
        .ok_or("Could not parse input")?;

    Ok(map)
}

fn part_1(orbits: &BTreeMap<&str, Vec<&str>>, start: &str) -> usize {
    fn traverse(orbits: &BTreeMap<&str, Vec<&str>>, start: &str, depth: usize) -> usize {
        orbits.get(start).map_or(depth, |x| {
            x.iter()
                .map(|y| traverse(orbits, y, depth + 1))
                .sum::<usize>()
                + depth
        })
    }

    traverse(orbits, start, 0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1])?;
    let orbits = parse_input(&contents)?;

    println!("Part 1: {:?}", part_1(&orbits, "COM"));

    Ok(())
}
