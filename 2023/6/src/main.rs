use std::{env, fs, process::exit};

#[derive(Debug)]
struct Trial {
    time: u64,
    distance: u64,
}

fn parse_input_part1<S: AsRef<str>>(file_string: S) -> Vec<Trial> {
    let lines = file_string.as_ref().lines().collect::<Vec<_>>();
    fn parse_line(line: &str) -> Vec<u64> {
        line.split(':')
            .nth(1)
            .unwrap()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
    }
    let times = parse_line(lines[0]);
    let distances = parse_line(lines[1]);

    times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| Trial { time, distance })
        .collect::<Vec<Trial>>()
}

fn parse_input_part2<S: AsRef<str>>(file_string: S) -> Trial {
    let lines = file_string.as_ref().lines().collect::<Vec<_>>();
    fn parse_line(line: &str) -> u64 {
        line.split(':')
            .nth(1)
            .unwrap()
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .concat()
            .parse::<u64>()
            .unwrap()
    }

    Trial {
        time: parse_line(lines[0]),
        distance: parse_line(lines[1]),
    }
}

#[allow(dead_code)]
fn ways_to_win_dumb(trial: &Trial) -> usize {
    (0..trial.time + 1)
        .map(|speed| speed * (trial.time - speed))
        .filter(|&distance| distance > trial.distance)
        .count()
}

fn ways_to_win(trial: &Trial) -> usize {
    let mut forward: usize = 0;
    let mut backward: usize = 0;

    for speed in 0..trial.time + 1 {
        let distance = speed * (trial.time - speed);
        if distance > trial.distance {
            forward = speed.try_into().unwrap();
            break;
        }
    }

    for speed in (0..trial.time + 1).rev() {
        let distance = speed * (trial.time - speed);
        if distance > trial.distance {
            backward = speed.try_into().unwrap();
            break;
        }
    }

    backward + 1 - forward
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1]).expect("Should have been able to read the file");

    let trials_part1 = parse_input_part1(&contents);

    println!(
        "Part 1: {}",
        trials_part1.iter().map(ways_to_win).product::<usize>()
    );

    let trials_part2 = parse_input_part2(&contents);

    println!("Part 2: {}", ways_to_win(&trials_part2));
}
