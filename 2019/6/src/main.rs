use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    env,
    error::Error,
    fs,
    process::exit,
    rc::Rc,
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
        curr: &'a str,
        seen: &mut BTreeSet<&'a str>,
        depth: usize,
    ) -> usize {
        seen.insert(curr);

        orbits.get(curr).map_or(depth, |nodes| {
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

// adapted from: https://stackoverflow.com/a/71190546
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchPath<N>(N, Option<Rc<SearchPath<N>>>);

impl<N> SearchPath<N> {
    fn len(&self) -> usize {
        self.1.as_ref().map_or(1, |path| 1 + path.len())
    }
}

fn part_2(orbits: &BTreeMap<&str, Vec<&str>>, start: &str, end: &str) -> Option<usize> {
    let mut visited: BTreeSet<&str> = BTreeSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(SearchPath(start, None));
    while let Some(SearchPath(node, path)) = queue.pop_front() {
        if node == end {
            return Some(SearchPath(node, path).len() - 3);
        }

        let path = Rc::new(SearchPath(node, path.clone()));

        for edge in orbits.get(node).unwrap_or(&vec![]) {
            if !visited.contains(edge) {
                visited.insert(edge);
                queue.push_back(SearchPath(edge, Some(path.clone())));
            }
        }
    }

    None
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
    println!(
        "Part 2: {:?}",
        part_2(&orbits, "YOU", "SAN").ok_or("No path found from 'YOU' to 'SAN'")?
    );

    Ok(())
}
