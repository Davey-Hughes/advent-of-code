use itertools::Itertools;
use std::{env, error::Error, fs, process::exit};

#[derive(Debug)]
struct Equation {
    lhs: u64,
    rhs: Vec<u64>,
}

fn parse_input(input_string: &str) -> Result<Vec<Equation>, Box<dyn Error>> {
    input_string
        .lines()
        .map(|l| -> Result<Equation, Box<dyn Error>> {
            let mut sides = l.split(':');

            let lhs = sides.next().ok_or("Error parsing input")?.parse::<u64>()?;
            let rhs = sides
                .next()
                .ok_or("Error parsing input")?
                .split_whitespace()
                .map(str::parse::<u64>)
                .collect::<Result<Vec<_>, _>>()?;

            Ok(Equation { lhs, rhs })
        })
        .collect::<Result<Vec<_>, _>>()
}

trait U64Ext {
    unsafe fn concat(self, other: u64) -> u64;
}

impl U64Ext for u64 {
    unsafe fn concat(self, n: u64) -> u64 {
        let mut m = 1;

        while m <= n {
            m *= 10;
        }

        m * self + n
    }
}

#[allow(clippy::type_complexity)]
fn compute_equations(operators: &[unsafe fn(u64, u64) -> u64], equations: &[Equation]) -> u64 {
    equations
        .iter()
        .map(|equation| {
            for ops in (0..equation.rhs.len() - 1)
                .map(|_| operators)
                .multi_cartesian_product()
            {
                if equation.lhs
                    == equation.rhs[1..]
                        .iter()
                        .enumerate()
                        .fold(equation.rhs[0], |acc, (i, e)| unsafe { ops[i](acc, *e) })
                {
                    return equation.lhs;
                }
            }

            0
        })
        .sum::<u64>()
}

fn part_1(equations: &[Equation]) -> u64 {
    let operators = [u64::unchecked_add, u64::unchecked_mul];

    compute_equations(&operators, equations)
}

fn part_2(equations: &[Equation]) -> u64 {
    let operators = [u64::unchecked_add, u64::unchecked_mul, u64::concat];

    compute_equations(&operators, equations)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1])?;
    let equations = parse_input(&contents)?;

    println!("Part 1: {:?}", part_1(&equations));
    println!("Part 2: {:?}", part_2(&equations));

    Ok(())
}
