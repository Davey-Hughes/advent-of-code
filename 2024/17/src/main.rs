use itertools::Itertools;
use regex::Regex;
use std::{env, error::Error, fs};

#[derive(Debug, Clone)]
struct Process {
    a: i64,
    b: i64,
    c: i64,
    pc: usize,

    instructions: Vec<i64>,
}

fn parse_input(input_string: &str) -> Result<Process, Box<dyn Error>> {
    let re = Regex::new(r"[0-9]+")?;

    let (registers, instructions) = input_string
        .split_once("\n\n")
        .ok_or("Malformed input. Expected blank line between registers and instructions")?;

    let registers = registers
        .lines()
        .map(|l| re.find(l))
        .collect::<Option<Vec<_>>>()
        .ok_or("Malformed registers in input")?
        .iter()
        .map(|r| r.as_str().parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let instructions = instructions
        .split_whitespace()
        .nth(1)
        .ok_or("Expected space between `Program:` and instruction list")?
        .split(',')
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Process {
        a: registers[0],
        b: registers[1],
        c: registers[2],
        pc: 0,

        instructions,
    })
}

fn compute(process: &mut Process) -> Result<Vec<i64>, Box<dyn Error>> {
    let mut res = vec![];

    while process.pc < process.instructions.len() - 1 {
        let opcode = process.instructions[process.pc];
        let operand = process.instructions[process.pc + 1];

        let combo_operand = match operand {
            0..=3 => operand,
            4 => process.a,
            5 => process.b,
            6 => process.c,
            _ => panic!("Invalid operand"),
        };

        match opcode {
            0 => process.a >>= combo_operand,
            1 => process.b ^= operand,
            2 => process.b = combo_operand % 8,
            3 => {
                if process.a != 0 {
                    process.pc = usize::try_from(operand)?;
                    continue;
                }
            }
            4 => process.b ^= process.c,
            5 => res.push(combo_operand % 8),
            6 => process.b = process.a >> combo_operand,
            7 => process.c = process.a >> combo_operand,
            _ => panic!("Invalid opcode"),
        }

        process.pc += 2;
    }

    Ok(res)
}

fn part_1(process: &mut Process) -> Result<String, Box<dyn Error>> {
    Ok(compute(process)?.into_iter().join(","))
}

fn part_2(process: &mut Process) -> Result<i64, Box<dyn Error>> {
    let end = &process.instructions;
    let mut cur = vec![0];

    for k in 0..end.len() {
        let mut iter_res = vec![];

        for val in &cur {
            let min_i = val << 3;
            let max_i = (min_i | 0b111) + 1;

            for i in min_i..max_i {
                let mut p = process.clone();
                p.a = i;

                if compute(&mut p)? == end[end.len() - k - 1..] {
                    iter_res.push(i);
                }
            }
        }

        cur = iter_res;
    }

    Ok(cur[0])
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;

    let mut process = parse_input(&contents)?;

    println!("Part 1: {}", part_1(&mut process.clone())?);
    println!("Part 2: {}", part_2(&mut process)?);

    Ok(())
}
