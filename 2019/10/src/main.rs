use itertools::Itertools;
use std::{env, error::Error, f64::consts::PI, fs, process::exit};

mod gcd;

#[derive(Debug, Clone, Copy)]
struct Asteroid {
    theta: f64,
    pos: (i64, i64),
    relative_pos: (i64, i64),
    seen: bool,
}

#[allow(clippy::cast_precision_loss)]
fn dist((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> f64 {
    (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt()
}

fn reduce_fraction(num: i64, den: i64) -> (i64, i64) {
    let g: i64 = gcd::stein(num.unsigned_abs(), den.unsigned_abs())
        .try_into()
        .expect("Divisor too large to fit into i64");

    (num / g, den / g)
}

fn parse_input(file_string: &str) -> Result<Vec<(i64, i64)>, Box<dyn Error>> {
    file_string
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.bytes()
                .enumerate()
                .filter(|(_, p)| p == &b'#')
                .map(move |(x, _)| -> Result<_, _> { Ok((x.try_into()?, y.try_into()?)) })
        })
        .collect::<Result<Vec<_>, _>>()
}

fn best_station(asteroids: &[(i64, i64)]) -> Result<(usize, (i64, i64)), Box<dyn Error>> {
    Ok(asteroids
        .iter()
        .map(|(x, y)| {
            (
                asteroids
                    .iter()
                    .filter(|(xx, yy)| xx != x || yy != y)
                    .map(|(xx, yy)| reduce_fraction(*x - *xx, *y - *yy))
                    .unique()
                    .count(),
                (*x, *y),
            )
        })
        .max()
        .ok_or("No asteroids in input")?)
}

#[allow(clippy::cast_precision_loss)]
fn part_2(asteroids: &[(i64, i64)], start: (i64, i64)) -> Result<i64, Box<dyn Error>> {
    let x = start.0;
    let y = start.1;

    let mut angles = asteroids
        .iter()
        .filter(|(xx, yy)| *xx != x || *yy != y)
        .map(|(xx, yy)| Asteroid {
            theta: ((-(x as f64 - *xx as f64)).atan2(-(y as f64 - *yy as f64)) - PI) % (2.0 * PI),
            pos: (*xx, *yy),
            relative_pos: (x - *xx, y - *yy),
            seen: false,
        })
        .collect::<Vec<_>>();

    angles.sort_by(|a, b| b.theta.total_cmp(&a.theta));

    let mut grouped_angles =
        angles
            .into_iter()
            .fold(vec![], |mut acc: Vec<Vec<Asteroid>>, e: Asteroid| {
                if let Some(prev) = acc.last_mut() {
                    if (prev[0].theta - e.theta).abs() < 0.0001 {
                        prev.push(e);
                        return acc;
                    }
                }

                acc.push(vec![e]);
                acc
            });

    for group in &mut grouped_angles {
        group.sort_by(|a, b| dist(b.relative_pos, start).total_cmp(&dist(a.relative_pos, start)));
    }

    let nth_asteroid = |n, groups: &mut Vec<Vec<Asteroid>>| -> Option<Asteroid> {
        let mut i = 0;

        while i < n {
            for group in groups.iter_mut() {
                for asteroid in group.iter_mut() {
                    if asteroid.seen {
                        continue;
                    }

                    asteroid.seen = true;
                    i += 1;

                    if i == n {
                        return Some(*asteroid);
                    }

                    break;
                }
            }
        }

        None
    };

    let asteroid_200 = nth_asteroid(200, &mut grouped_angles).ok_or("No 200th asteroid")?;
    Ok(asteroid_200.pos.0 * 100 + asteroid_200.pos.1)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1])?;
    let asteroids = parse_input(&contents)?;

    let station = best_station(&asteroids)?;
    println!("Part 1: {:?}", station.0);
    println!("Part 2: {:?}", part_2(&asteroids, station.1)?);

    Ok(())
}
