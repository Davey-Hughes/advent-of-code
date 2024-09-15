use std::{env, error::Error, fs, process::exit};

#[derive(Debug)]
struct PasswordCracker {
    start: Vec<u32>,
    end: Vec<u32>,
}

impl PasswordCracker {
    fn from_strs(start: &str, end: &str) -> Option<Self> {
        Some(Self {
            start: start
                .to_string()
                .chars()
                .map(|c| c.to_digit(10))
                .collect::<Option<Vec<_>>>()?,
            end: end
                .to_string()
                .chars()
                .map(|c| c.to_digit(10))
                .collect::<Option<Vec<_>>>()?,
        })
    }

    fn inc(&mut self) {
        for d in self.start.iter_mut().rev() {
            if *d != 9 {
                *d += 1;
                break;
            }

            *d = 0;
        }
    }

    fn is_increasing(&self) -> bool {
        self.start.windows(2).all(|w| w[0] <= w[1])
    }

    fn has_double(&self) -> bool {
        self.start.windows(2).any(|w| w[0] == w[1])
    }

    fn has_only_double(&self) -> bool {
        let mut counts = [0; 10];
        for d in &self.start {
            counts[*d as usize] += 1;
        }

        counts.iter().any(|c| *c == 2)
    }

    fn crack(&mut self) -> (u32, u32) {
        let mut part1 = 0;
        let mut part2 = 0;

        // goes up to but not including end
        while self.start.ne(&self.end) {
            if self.is_increasing() {
                if self.has_double() {
                    part1 += 1;
                }

                if self.has_only_double() {
                    part2 += 1;
                }
            }

            self.inc();
        }

        (part1, part2)
    }
}

fn parse_input(file_string: &str) -> Result<(&str, &str), Box<dyn Error>> {
    let v = file_string
        .lines()
        .next()
        .ok_or("Error reading line from input file")?
        .split('-')
        .collect::<Vec<_>>();

    Ok((v[0], v[1]))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1])?;
    let input = parse_input(&contents)?;

    let (part1, part2) = PasswordCracker::from_strs(input.0, input.1)
        .ok_or("Error parsing input range")?
        .crack();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}
