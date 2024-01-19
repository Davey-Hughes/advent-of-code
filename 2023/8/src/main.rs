use std::{collections::BTreeMap, env, fs, process::exit, slice::Iter, thread, time::Duration};

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "main.pest"]
struct Day8Parser;

#[derive(Debug)]
struct Instructions {
    values: Vec<char>,
    index: usize,
}

impl Instructions {
    fn new(values: Vec<char>) -> Self {
        Instructions { values, index: 0 }
    }
}

impl Iterator for Instructions {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let val = Some(self.values[self.index]);

        match self.index < self.values.len() - 1 {
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

    fn traverse_part1(&mut self) -> u64 {
        let mut cur = "AAA";
        let mut num_steps = 0;

        while cur != "ZZZ" {
            num_steps += 1;

            cur = match self.instructions.next() {
                Some('L') => self.map.get(cur).unwrap().0.as_str(),
                Some('R') => self.map.get(cur).unwrap().1.as_str(),
                _ => cur,
            };
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

    let mut graph = Graph::new(&contents);
    println!("{:?}", graph.traverse_part1());
}
