use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    env,
    error::Error,
    fs,
};

use regex::Regex;

type Wires = BTreeMap<String, u8>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Op {
    AND,
    OR,
    XOR,
}

impl TryFrom<&str> for Op {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "AND" => Ok(Self::AND),
            "OR" => Ok(Self::OR),
            "XOR" => Ok(Self::XOR),
            _ => Err("Invalid operator"),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Instruction {
    inputs: (String, String),
    output: String,
    op: Op,
}

#[allow(clippy::complexity)]
fn parse_input(input_string: &str) -> Result<(Wires, VecDeque<Instruction>), Box<dyn Error>> {
    let re = Regex::new(r"(\S+) (AND|OR|XOR) (\S+) -> (\S+)")?;

    let (input, wires) = input_string
        .split_once("\n\n")
        .ok_or("Two sections expected in input")?;

    let map = input
        .lines()
        .map(|line| -> Result<_, Box<dyn Error>> {
            let (name, value) = line.split_once(": ").ok_or("Invalid input")?;
            Ok((name.to_string(), value.parse()?))
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .fold(Wires::new(), |mut acc, (name, value)| {
            acc.insert(name, value);
            acc
        });

    let instructions = re
        .captures_iter(wires)
        .map(|c| c.extract())
        .map(|(_, [input0, op, input1, output])| Instruction {
            inputs: (input0.to_string(), input1.to_string()),
            output: output.to_string(),
            op: Op::try_from(op).unwrap(),
        })
        .collect::<VecDeque<_>>();

    Ok((map, instructions))
}

fn part_1(wires: &mut Wires, instructions: &mut VecDeque<Instruction>) -> u64 {
    while let Some(ins) = instructions.pop_front() {
        if let Some(val0) = wires.get(&ins.inputs.0) {
            if let Some(val1) = wires.get(&ins.inputs.1) {
                let result = match ins.op {
                    Op::AND => val0 & val1,
                    Op::OR => val0 | val1,
                    Op::XOR => val0 ^ val1,
                };
                wires.insert(ins.output, result);
                continue;
            }
        }

        instructions.push_back(ins);
    }

    wires
        .iter()
        .filter(|(k, _)| k.starts_with('z'))
        .rev()
        .fold(0, |acc: u64, (_, &v)| (acc << 1) + u64::from(v))
}

fn part_2(instructions: &VecDeque<Instruction>) -> String {
    let instruction_map: HashMap<&str, HashSet<&Instruction>> =
        instructions.iter().fold(HashMap::new(), |mut acc, ins| {
            acc.entry(ins.output.as_str()).or_default().insert(ins);
            acc.entry(ins.inputs.0.as_str()).or_default().insert(ins);
            acc.entry(ins.inputs.1.as_str()).or_default().insert(ins);

            acc
        });

    let mut res = BTreeSet::new();

    let max_z = instructions
        .iter()
        .filter(|&e| e.output.starts_with('z'))
        .fold(String::new(), |acc, e| {
            if e.output > acc {
                e.output.clone()
            } else {
                acc
            }
        });

    for ins in instructions {
        // all outputs must be preceded by XOR
        if ins.output.starts_with('z') && ins.output != max_z && ins.op != Op::XOR {
            res.insert(ins.output.clone());
        }

        if ins.op == Op::XOR {
            let re = Regex::new(r"[xyz]\d+").unwrap();

            // all XORs must be adjacent to an x, y, or z wire
            if [&ins.inputs.0, &ins.inputs.1, &ins.output]
                .iter()
                .all(|s| re.find(s).is_none())
            {
                res.insert(ins.output.clone());
            }

            // an XOR must not be followed by an OR
            if let Some(subopts) = instruction_map.get(ins.output.as_str()) {
                for subopt in subopts {
                    if (ins.output == subopt.inputs.0 || ins.output == subopt.inputs.1)
                        && subopt.op == Op::OR
                    {
                        res.insert(ins.output.clone());
                    }
                }
            }
        }

        // an AND must be followed by an OR (except for the first half-adder)
        if ins.op == Op::AND && ins.inputs.0 != "x00" && ins.inputs.1 != "x00" {
            if let Some(subopts) = instruction_map.get(ins.output.as_str()) {
                for subopt in subopts {
                    if (ins.output == subopt.inputs.0 || ins.output == subopt.inputs.1)
                        && subopt.op != Op::OR
                    {
                        res.insert(ins.output.clone());
                    }
                }
            }
        }
    }

    res.into_iter().collect::<Vec<_>>().join(",")
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;

    let (mut wires, mut instructions) = parse_input(&contents)?;

    let part2 = part_2(&instructions);

    println!("Part 1: {:?}", part_1(&mut wires, &mut instructions));
    println!("Part 2: {part2}");

    Ok(())
}
