use std::{
    collections::{HashMap, HashSet, VecDeque},
    env,
    error::Error,
    fs,
};

fn parse_input(input_string: &str) -> Vec<&str> {
    input_string.lines().collect()
}

struct Keypad {
    keys: HashMap<char, Vec<(char, char)>>,
    path_cache: HashMap<(char, char), Vec<String>>,
    memo: HashMap<(char, char, usize), usize>,
}

impl Keypad {
    fn new() -> Self {
        Self {
            keys: HashMap::from([
                ('A', vec![('0', '<'), ('3', '^')]),
                ('0', vec![('A', '>'), ('2', '^')]),
                ('1', vec![('2', '>'), ('4', '^')]),
                ('2', vec![('0', 'v'), ('5', '^'), ('1', '<'), ('3', '>')]),
                ('3', vec![('A', 'v'), ('6', '^'), ('2', '<')]),
                ('4', vec![('1', 'v'), ('7', '^'), ('5', '>')]),
                ('5', vec![('2', 'v'), ('8', '^'), ('4', '<'), ('6', '>')]),
                ('6', vec![('3', 'v'), ('9', '^'), ('5', '<')]),
                ('7', vec![('4', 'v'), ('8', '>')]),
                ('8', vec![('5', 'v'), ('7', '<'), ('9', '>')]),
                ('9', vec![('6', 'v'), ('8', '<')]),
            ]),
            path_cache: HashMap::new(),
            memo: HashMap::new(),
        }
    }

    fn new_number_pad() -> Self {
        Keypad::new()
    }

    fn new_arrow_pad() -> Self {
        Self {
            keys: HashMap::from([
                ('A', vec![('^', '<'), ('>', 'v')]),
                ('^', vec![('A', '>'), ('v', 'v')]),
                ('<', vec![('v', '>')]),
                ('v', vec![('^', '^'), ('<', '<'), ('>', '>')]),
                ('>', vec![('v', '<'), ('A', '^')]),
            ]),
            path_cache: HashMap::new(),
            memo: HashMap::new(),
        }
    }

    fn bfs(&mut self, start: char, end: char) -> Vec<String> {
        if let Some(paths) = self.path_cache.get(&(start, end)) {
            return paths.clone();
        }

        let mut queue = VecDeque::from([(start, String::new(), HashSet::new())]);
        let mut paths = vec![];
        let mut shortest_path_len = usize::MAX;

        while let Some((key, path_so_far, mut seen_so_far)) = queue.pop_front() {
            if path_so_far.len() > shortest_path_len {
                break;
            }

            if key == end {
                paths.push((*path_so_far).to_string());
                shortest_path_len = path_so_far.len();
            }

            seen_so_far.insert(key);

            if let Some(neighbor_keys) = self.keys.get(&key) {
                for (next_key, direction) in neighbor_keys {
                    if seen_so_far.contains(next_key) {
                        continue;
                    }

                    let mut new_path = path_so_far.clone();
                    new_path.push(*direction);
                    queue.push_back((*next_key, new_path, seen_so_far.clone()));
                }
            }
        }

        paths = paths.into_iter().map(|p| p + "A").collect::<Vec<_>>();

        self.path_cache.entry((start, end)).or_insert(paths.clone());

        paths
    }

    fn cheapest_arrowpad(&mut self, start: char, end: char, depth: usize) -> usize {
        if let Some(&cost) = self.memo.get(&(start, end, depth)) {
            return cost;
        }

        let mut res = usize::MAX;

        for v in self.bfs(start, end) {
            res = res.min(self.cheapest_robot(&v, depth - 1));
        }

        self.memo.insert((start, end, depth), res);
        res
    }

    fn cheapest_robot(&mut self, path: &str, depth: usize) -> usize {
        if depth == 0 {
            return path.len();
        }

        "A".chars()
            .chain(path.chars())
            .collect::<Vec<_>>()
            .windows(2)
            .map(|c| self.cheapest_arrowpad(c[0], c[1], depth))
            .sum()
    }
}

fn complexities(codes: &Vec<&str>, num_arrowpads: usize) -> Result<usize, Box<dyn Error>> {
    let re = regex::Regex::new(r"[0-9]+")?;

    let mut keypad = Keypad::new_number_pad();
    let mut arrowpad = Keypad::new_arrow_pad();

    Ok(codes
        .iter()
        .map(|&code| {
            let d = re.find(code).ok_or("Can't find digit in code")?;
            let d = d.as_str().parse::<usize>()?;

            let robot0_commands = "A"
                .chars()
                .chain(code.chars())
                .collect::<Vec<_>>()
                .windows(2)
                .map(|c| keypad.bfs(c[0], c[1]))
                .collect::<Vec<_>>();

            let num_presses = robot0_commands
                .into_iter()
                .map(|v| {
                    v.clone()
                        .into_iter()
                        .map(|s| {
                            "A".chars()
                                .chain(s.chars())
                                .collect::<Vec<_>>()
                                .windows(2)
                                .map(|c| arrowpad.cheapest_arrowpad(c[0], c[1], num_arrowpads))
                                .sum::<usize>()
                        })
                        .min()
                        .expect("There must be a min")
                })
                .sum::<usize>();

            Ok(d * num_presses)
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?
        .iter()
        .sum())
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;

    let codes = parse_input(&contents);

    println!("Part 1: {:?}", complexities(&codes, 2)?);
    println!("Part 2: {:?}", complexities(&codes, 25)?);

    Ok(())
}
