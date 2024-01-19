use std::{collections::BTreeMap, env, fs, process::exit};

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "main.pest"]
struct Day8Parser;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

#[derive(Debug, Default)]
struct Instructions {
    values: Vec<char>,
}

struct InstructionsIterator<'a> {
    instructions: &'a Instructions,
    index: usize,
}

impl Instructions {
    fn new(values: Vec<char>) -> Instructions {
        Instructions { values }
    }

    fn iter(&self) -> InstructionsIterator {
        InstructionsIterator {
            instructions: self,
            index: 0,
        }
    }
}

impl<'a> Iterator for InstructionsIterator<'a> {
    type Item = &'a char;

    fn next(&mut self) -> Option<Self::Item> {
        let val = Some(&self.instructions.values[self.index]);

        match self.index < self.instructions.values.len() - 1 {
            true => self.index += 1,
            false => self.index = 0,
        }

        val
    }
}

#[derive(Debug)]
struct Graph {
    instructions: Instructions,
    map: BTreeMap<String, (String, String)>,
}

impl Graph {
    fn new<S: AsRef<str>>(file_string: &S) -> Self {
        let file = Day8Parser::parse(Rule::file, file_string.as_ref()).expect("cannot read file");

        let mut instructions: Instructions = Instructions::new(vec![]);
        let mut map = BTreeMap::default();

        for item in file {
            match item.as_rule() {
                Rule::instructions => {
                    instructions = Instructions::new(item.as_str().chars().collect())
                }
                Rule::node => {
                    let (mut key, mut left, mut right) = ("", "", "");
                    for part in item.into_inner() {
                        match part.as_rule() {
                            Rule::key => key = part.as_str(),
                            Rule::left => left = part.as_str(),
                            Rule::right => right = part.as_str(),
                            _ => (),
                        }
                    }
                    map.insert(key.to_string(), (left.to_string(), right.to_string()));
                }
                _ => (),
            }
        }

        Graph { instructions, map }
    }

    fn traverse_part1(&self, start: &str, cond: fn(&str) -> bool) -> u64 {
        let mut cur = start;
        let mut num_steps = 0;

        let mut it = self.instructions.iter();
        while cond(cur) {
            num_steps += 1;

            cur = match it.next() {
                Some('L') => self.map.get(cur).unwrap().0.as_str(),
                Some('R') => self.map.get(cur).unwrap().1.as_str(),
                _ => cur,
            };
        }

        num_steps
    }

    fn traverse_part2(&self) -> u64 {
        self.map
            .keys()
            .filter(|s| s.ends_with('A'))
            .map(|s| self.traverse_part1(s.as_str(), |s| !s.ends_with('Z')))
            .reduce(lcm)
            .unwrap()
    }

    #[allow(dead_code)]
    fn traverse_part2_brute_force(&self) -> u64 {
        let mut curs = self
            .map
            .keys()
            .filter(|s| s.ends_with('A'))
            .collect::<Vec<_>>();

        let mut num_steps = 0;

        let mut it = self.instructions.iter();
        while !curs.iter().all(|v| v.ends_with('Z')) {
            num_steps += 1;

            let instruction = it.next();

            for cur in curs.iter_mut() {
                *cur = match instruction {
                    Some('L') => &self.map.get(cur.as_str()).unwrap().0,
                    Some('R') => &self.map.get(cur.as_str()).unwrap().1,
                    _ => cur,
                }
            }
        }

        num_steps
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1]).expect("Should have been able to read the file");

    let graph = Graph::new(&contents);

    println!("Part 1: {:?}", graph.traverse_part1("AAA", |s| s != "ZZZ"));
    println!("Part 2: {:?}", graph.traverse_part2());
}
