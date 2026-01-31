use core::fmt;
use std::{env, error::Error, fs::read_to_string, str::FromStr};

#[derive(Debug)]
enum Direction {
    Right,
    Left,
}

#[derive(Debug)]
struct ParseDirectionError;

impl fmt::Display for ParseDirectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid direction")
    }
}

impl Error for ParseDirectionError {}

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            _ => Err(ParseDirectionError),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    amount: i32,
}

fn parse_input(input_string: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
    input_string
        .lines()
        .map(|line: &str| -> Result<Instruction, Box<dyn Error>> {
            let direction = Direction::from_str(line.chars().take(1).collect::<String>().as_str())?;
            let amount = line.chars().skip(1).collect::<String>().parse::<_>()?;

            Ok(Instruction { direction, amount })
        })
        .collect::<Result<Vec<Instruction>, Box<dyn Error>>>()
}

fn part_1(instructions: &[Instruction]) -> usize {
    let mut number: i32 = 50;

    instructions
        .iter()
        .map(|instruction| {
            match &instruction.direction {
                Direction::Right => number = (number + instruction.amount) % 100,
                Direction::Left => number = (number - instruction.amount) % 100,
            }

            number == 0
        })
        .filter(|x| *x)
        .count()
}

fn part_2(instructions: &[Instruction]) -> u32 {
    let mut number: i32 = 50;

    instructions
        .iter()
        .map(|instruction| {
            let prev = number;

            match &instruction.direction {
                Direction::Right => number += instruction.amount,
                Direction::Left => number -= instruction.amount,
            }

            let quotient = number / 100;
            number %= 100;

            let mut ret = 0;

            match number.cmp(&0) {
                std::cmp::Ordering::Equal => {
                    ret += quotient.unsigned_abs();

                    if quotient.unsigned_abs() == 0 || quotient < 0 {
                        ret += 1;
                    }
                }
                std::cmp::Ordering::Less => {
                    if prev != 0 {
                        ret += 1;
                    }
                    number += 100;
                    ret += quotient.unsigned_abs();
                }
                std::cmp::Ordering::Greater => {
                    ret += quotient.unsigned_abs();
                }
            }

            ret
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;
    let instructions = parse_input(&contents)?;

    println!("Part 1: {}", part_1(&instructions));
    println!("Part 2: {}", part_2(&instructions));

    Ok(())
}
