use std::{env, error::Error, fs, process::exit};

fn parse_input<S: AsRef<str>>(file_string: &S) -> impl Iterator<Item = u64> + Clone + '_ {
    file_string
        .as_ref()
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
}

const fn get_fuel_part1(mass: u64) -> u64 {
    (mass / 3).saturating_sub(2)
}

fn get_fuel_part2(mass: u64) -> u64 {
    if mass == 0 {
        return 0;
    }

    let fuel = (mass / 3).saturating_sub(2);

    fuel + get_fuel_part2(fuel)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1])?;

    let lines = parse_input(&contents);

    println!(
        "Part 1: {:?}",
        lines.clone().map(get_fuel_part1).sum::<u64>()
    );

    println!("Part 2: {:?}", lines.map(get_fuel_part2).sum::<u64>());

    Ok(())
}
