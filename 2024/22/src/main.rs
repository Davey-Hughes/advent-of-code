use std::{
    collections::{HashMap, HashSet},
    env,
    error::Error,
    fs,
};

fn parse_input(input_string: &str) -> Result<Vec<i64>, Box<dyn Error>> {
    Ok(input_string
        .lines()
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?)
}

fn part_1(secrets: &[i64], n: usize) -> i64 {
    secrets
        .iter()
        .map(|s| {
            let mut secret = *s;

            for _ in 0..n {
                for f in [|s: i64| s * 64, |s: i64| s / 32, |s: i64| s * 2048] {
                    secret = (f(secret) ^ secret) % 16_777_216;
                }
            }

            secret
        })
        .sum()
}

fn part_2(secrets: &[i64], n: usize) -> i64 {
    let mut sell_prices = HashMap::new();

    for s in secrets {
        let mut seen = HashSet::new();

        let mut secret = *s;
        let mut prev_price = secret % 10;

        (0..n)
            .map(|_| {
                for f in [|s: i64| s * 64, |s: i64| s / 32, |s: i64| s * 2048] {
                    secret = (f(secret) ^ secret) % 16_777_216;
                }

                let price = secret % 10;
                let diff = price - prev_price;
                prev_price = price;

                (diff, price)
            })
            .collect::<Vec<_>>()
            .windows(4)
            .for_each(|w| {
                let key = (w[0].0, w[1].0, w[2].0, w[3].0);
                if seen.insert(key) {
                    sell_prices
                        .entry(key)
                        .and_modify(|e| *e += w[3].1)
                        .or_insert(w[3].1);
                }
            });
    }

    sell_prices.into_values().max().expect("No sell prices")
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;

    let secrets = parse_input(&contents)?;

    println!("Part 1: {:?}", part_1(&secrets, 2000));
    println!("Part 2: {:?}", part_2(&secrets, 2000));

    Ok(())
}
