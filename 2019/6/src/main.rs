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
            let first = x.next()?;
            let second = x.next()?;
            map.entry(first).or_default().push(second);
            map.entry(second).or_default().push(first);

            Some(())
        })
        .collect::<Option<_>>()
        .ok_or("Could not parse input")?;

    Ok(map)
}

fn part_1(orbits: &BTreeMap<&str, Vec<&str>>, start: &str) -> usize {
    fn dfs<'a>(
        orbits: &BTreeMap<&str, Vec<&'a str>>,
        start: &'a str,
        seen: &mut BTreeSet<&'a str>,
        depth: usize,
    ) -> usize {
        seen.insert(start);

        orbits.get(start).map_or(depth, |nodes| {
            nodes
                .iter()
                .map(|node| {
                    if seen.contains(node) {
                        depth
                    } else {
                        dfs(orbits, node, seen, depth + 1)
                    }
                })
                .sum::<usize>()
        })
    }

    dfs(orbits, start, &mut BTreeSet::new(), 0)
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
