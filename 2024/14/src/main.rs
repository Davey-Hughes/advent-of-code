use std::{collections::BTreeSet, env, error::Error, fs};

use regex::Regex;

#[derive(Debug)]
struct Robot {
    pos: (i64, i64),
    velocity: (i64, i64),
}

fn parse_input(input_string: &str) -> Result<Vec<Robot>, Box<dyn Error>> {
    let re_digits = Regex::new(r"-?[\d]+")?;

    input_string
        .lines()
        .map(|l| {
            let robot = re_digits
                .find_iter(l)
                .map(|c| c.as_str().parse::<i64>())
                .collect::<Result<Vec<_>, _>>()?;

            Ok(Robot {
                pos: (robot[0], robot[1]),
                velocity: (robot[2], robot[3]),
            })
        })
        .collect::<Result<Vec<_>, _>>()
}

#[allow(dead_code)]
fn print_grid(positions: &BTreeSet<(i64, i64)>, width: i64, height: i64) -> String {
    let mut res = String::new();
    for y in 0..height {
        for x in 0..width {
            if positions.contains(&(x, y)) {
                res.push('#');
            } else {
                res.push('.');
            }
        }
        res.push('\n');
    }

    res
}

fn part_1(robots: &[Robot]) -> u64 {
    fn update_pos(pos: i64, velocity: i64, steps: i64, modulus: i64) -> i64 {
        (pos + ((velocity * steps) % modulus) + modulus) % modulus
    }

    let width = 101;
    let height = 103;

    let mut quadrant_sums = [0; 4];

    for robot in robots {
        let new_pos = (
            update_pos(robot.pos.0, robot.velocity.0, 100, width),
            update_pos(robot.pos.1, robot.velocity.1, 100, height),
        );

        if new_pos.0 < width / 2 && new_pos.1 < height / 2 {
            quadrant_sums[0] += 1;
        } else if new_pos.0 > width / 2 && new_pos.1 < height / 2 {
            quadrant_sums[1] += 1;
        } else if new_pos.0 < width / 2 && new_pos.1 > height / 2 {
            quadrant_sums[2] += 1;
        } else if new_pos.0 > width / 2 && new_pos.1 > height / 2 {
            quadrant_sums[3] += 1;
        }
    }

    quadrant_sums.iter().product()
}

fn part_2(robots: &[Robot]) -> Option<i64> {
    fn update_pos(pos: i64, velocity: i64, steps: i64, modulus: i64) -> i64 {
        (pos + ((velocity * steps) % modulus) + modulus) % modulus
    }

    let width = 101;
    let height = 103;

    for steps in 0..100_000 {
        let positions = robots
            .iter()
            .map(|robot| {
                (
                    update_pos(robot.pos.0, robot.velocity.0, steps, width),
                    update_pos(robot.pos.1, robot.velocity.1, steps, height),
                )
            })
            .collect::<BTreeSet<(i64, i64)>>();

        for window in positions.iter().collect::<Vec<_>>().windows(10) {
            if window.windows(2).all(|x| x[0].1 + 1 == x[1].1) {
                return Some(steps);
            }
        }
    }

    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;

    let robots = parse_input(&contents)?;

    println!("Part 1: {:?}", part_1(&robots));
    println!("Part 2: {:?}", part_2(&robots).unwrap());

    Ok(())
}
