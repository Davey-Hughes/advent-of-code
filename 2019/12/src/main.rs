use std::{
    cell::RefCell,
    collections::BTreeMap,
    env, fs,
    ops::{Deref, DerefMut, Index, IndexMut},
    process::exit,
};

use anyhow::Result;
use itertools::Itertools;
use num::Integer;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MoonVec(Vec<i64>);

#[allow(dead_code)]
impl MoonVec {
    fn abs_sum(&self) -> i64 {
        self.0.iter().map(|x| x.abs()).sum()
    }

    fn add_assign(&mut self, other: &Self) {
        for (a, b) in self.iter_mut().zip(other.iter()) {
            *a += b;
        }
    }
}

impl Deref for MoonVec {
    type Target = Vec<i64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MoonVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Index<usize> for MoonVec {
    type Output = i64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for MoonVec {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Moon {
    pos: MoonVec,
    vel: MoonVec,
}

#[allow(dead_code)]
impl Moon {
    fn gravity_axies<I: Iterator<Item = usize>>(&mut self, other: &Self, axies: I) {
        for axis in axies {
            self.vel[axis] += (other.pos[axis] - self.pos[axis]).signum();
        }
    }

    fn gravity(&mut self, other: &Self) {
        self.gravity_axies(other, 0..self.pos.len());
    }

    fn apply_velocity_axies<I: Iterator<Item = usize>>(&mut self, axies: I) {
        for axis in axies {
            self.pos[axis] += self.vel[axis];
        }
    }

    fn apply_velocity(&mut self) {
        self.apply_velocity_axies(0..self.pos.len());
    }

    fn energy(&self) -> i64 {
        self.pos.abs_sum() * self.vel.abs_sum()
    }
}

fn parse_input(file_string: &str) -> Result<Vec<RefCell<Moon>>> {
    let re = Regex::new(r"([xyz])=(-?[0-9]+)")?;

    file_string
        .lines()
        .map(|l| {
            let mut map = BTreeMap::new();
            for (_, [axis, val]) in re.captures_iter(l).map(|c| c.extract()) {
                assert!(map.insert(axis, val).is_none(), "duplicate axis");
            }

            Ok(RefCell::new(Moon {
                vel: MoonVec(vec![0; map.len()]),
                pos: MoonVec(
                    map.into_values()
                        .map(|v| -> Result<i64> { Ok(v.parse::<i64>()?) })
                        .collect::<Result<Vec<i64>>>()?,
                ),
            }))
        })
        .collect::<Result<Vec<_>, _>>()
}

fn simulate_step<I>(moons: &[RefCell<Moon>], axies: &I)
where
    I: Iterator<Item = usize> + Clone,
{
    for pair in moons.iter().permutations(2) {
        (*pair[0].borrow_mut()).gravity_axies(&pair[1].borrow(), axies.clone());
    }

    for moon in moons {
        moon.borrow_mut().apply_velocity_axies(axies.clone());
    }
}

fn part_1(moons: &[RefCell<Moon>]) -> i64 {
    let axies = 0..moons[0].borrow().pos.len();
    for _ in 0..1000 {
        simulate_step(moons, &axies);
    }

    moons.iter().map(|m| m.borrow().energy()).sum()
}

fn part_2(moons: &[RefCell<Moon>]) -> i64 {
    let axies = 0..moons[0].borrow().pos.len();

    axies
        .map(|axis| {
            let mut period = 0;
            let moon_copies = moons
                .iter()
                .map(|m| {
                    RefCell::new(Moon {
                        vel: MoonVec(vec![0]),
                        pos: MoonVec(vec![m.borrow().pos[axis]]),
                    })
                })
                .collect::<Vec<_>>();

            loop {
                period += 1;
                simulate_step(&moon_copies, &(0..1));
                if moon_copies.iter().all(|m| m.borrow().vel[0] == 0) {
                    return period * 2;
                }
            }
        })
        .fold(1, |acc, x| acc.lcm(&x))
}

#[allow(clippy::redundant_clone)]
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1])?;
    let input = parse_input(&contents)?;

    println!("Part 1: {:?}", part_1(&input.clone()));
    println!("Part 2: {:?}", part_2(&input));

    Ok(())
}
