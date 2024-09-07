use std::{cmp::min, env, error::Error, fs, process::exit};

fn parse_input<S: AsRef<str>>(file_string: &S) -> Result<Vec<usize>, Box<dyn Error>> {
    Ok(file_string
        .as_ref()
        .lines()
        .next()
        .ok_or("Error reading line from input file")?
        .split(',')
        .map(str::parse::<usize>)
        .collect::<Result<Vec<_>, _>>()?)
}

fn calc(ints: &mut [usize]) -> &[usize] {
    let mut i = 0;
    while i < ints.len() {
        let pos1 = ints[i + 1];
        let pos2 = ints[i + 2];
        let pos3 = ints[i + 3];

        match ints.get(i) {
            Some(1) => {
                ints[pos3] = ints[pos1] + ints[pos2];
                i += 4;
            }
            Some(2) => {
                ints[pos3] = ints[pos1] * ints[pos2];
                i += 4;
            }
            Some(99) => break,
            _ => {
                println!("Error: unexpected opcode {}", ints[i]);
                break;
            }
        }
    }

    ints
}

#[allow(dead_code)]
fn print_ints(ints: &[usize], cur_line: usize) {
    let mut i = 0;
    while i < ints.len() {
        let arrow = if i / 4 == cur_line { "<-" } else { "" };
        let line = if ints[i] == 99 {
            let ret = ints.get(i..=i).unwrap();
            i += 1;
            ret
        } else {
            let ret = ints.get(i..min(i + 4, ints.len())).unwrap();
            i += 4;
            ret
        };

        println!("{:2}|{:3}: {:3?} {}", i / 4, i, line, arrow);
    }

    println!();
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1])?;

    let mut ints = parse_input(&contents)?;

    ints[1] = 12;
    ints[2] = 2;

    println!(
        "Part 1: {:?}",
        calc(&mut ints)
            .first()
            .ok_or("Couldn't get position 0 after calculation")?
    );

    Ok(())
}
