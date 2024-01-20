use std::{env, error::Error, fs, process::exit};

fn parse_input<S: AsRef<str>>(
    file_string: &S,
) -> impl Iterator<Item = impl Iterator<Item = i64> + '_> + '_ {
    file_string
        .as_ref()
        .lines()
        .map(|s| s.split(' '))
        .map(|x| x.map(|x| x.parse::<i64>().unwrap()))
}

fn predict_next<I>(history: I) -> i64
where
    I: Iterator<Item = i64>,
{
    let history_vec = history.collect::<Vec<_>>();
    let mut sum = *history_vec.last().unwrap();
    let mut cur_vec = history_vec;

    loop {
        let mut next_vec = vec![];
        let mut should_break = true;

        for i in 0..cur_vec.len() - 1 {
            let k = cur_vec[i + 1] - cur_vec[i];

            if k != 0 {
                should_break = false;
            }

            next_vec.push(k);
        }

        sum += next_vec.last().unwrap();

        if should_break {
            break;
        }

        cur_vec = next_vec;
    }

    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1])?;
    let lines = parse_input(&contents);

    println!("Part 1: {}", lines.map(predict_next).sum::<i64>());

    Ok(())
}
