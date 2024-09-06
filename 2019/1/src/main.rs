use std::{env, error::Error, fs, process::exit};

fn parse_input<S: AsRef<str>>(file_string: &S) -> impl Iterator<Item = i64> + Clone + '_ {
    file_string
        .as_ref()
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
}

const fn get_fuel(mass: i64) -> i64 {
    mass / 3 - 2
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1])?;

    let lines = parse_input(&contents);

    println!("Part 1: {:?}", lines.map(get_fuel).sum::<i64>());

    Ok(())
}
