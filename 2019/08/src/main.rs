use std::{env, error::Error, fs, process::exit};

fn parse_input(file_string: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    Ok(file_string
        .lines()
        .next()
        .ok_or("No line in input file")?
        .chars()
        .map(|x| x.to_digit(10))
        .collect::<Option<Vec<_>>>()
        .ok_or("Could not parse input digits")?)
}

fn part_1(input: &[u32], width: usize, height: usize) -> usize {
    let mut min_zeroes = width * height;
    let mut res = 0;
    for chunk in input.chunks(width * height) {
        let zeroes = chunk.iter().filter(|&&x| x == 0).count();
        if zeroes < min_zeroes {
            min_zeroes = zeroes;
            let ones = chunk.iter().filter(|&&x| x == 1).count();
            let twos = chunk.iter().filter(|&&x| x == 2).count();
            res = ones * twos;
        }
    }

    res
}

fn part_2(input: &[u32], width: usize, height: usize) {
    let mut image = vec![2; width * height];
    for chunk in input.chunks(width * height) {
        for (i, &x) in chunk.iter().enumerate() {
            if image[i] == 2 {
                image[i] = x;
            }
        }
    }

    for chunk in image.chunks(width) {
        for &x in chunk {
            print!("{}", if x == 0 { ' ' } else { '█' });
        }
        println!();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1])?;
    let input = parse_input(&contents)?;

    println!("Part 1: {:?}", part_1(&input, 25, 6));

    println!("Part 2:");
    part_2(&input, 25, 6);

    Ok(())
}
