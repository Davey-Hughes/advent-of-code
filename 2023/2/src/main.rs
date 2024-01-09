use std::{cmp, env, fs, ops::Index, ops::IndexMut, process::exit};

use aho_corasick::AhoCorasick;
use itertools::Itertools;

#[derive(Debug, Default)]
struct Game {
    red: i64,
    green: i64,
    blue: i64,
}

impl Game {
    fn possible(&self, other: &Game) -> bool {
        if other.red > self.red || other.green > self.green || other.blue > self.blue {
            return false;
        }

        true
    }

    fn power(&self) -> i64 {
        self.red * self.blue * self.green
    }
}

impl Index<&'_ str> for Game {
    type Output = i64;
    fn index(&self, s: &str) -> &i64 {
        match s {
            "red" => &self.red,
            "green" => &self.green,
            "blue" => &self.blue,
            _ => panic!("unknown field: {}", s),
        }
    }
}

impl IndexMut<&'_ str> for Game {
    fn index_mut(&mut self, s: &str) -> &mut i64 {
        match s {
            "red" => &mut self.red,
            "green" => &mut self.green,
            "blue" => &mut self.blue,
            _ => panic!("unknown field: {}", s),
        }
    }
}

static ACTUAL: Game = Game {
    red: 12,
    green: 13,
    blue: 14,
};

fn check_game<S: AsRef<str>>(line: S) -> i64 {
    let (game_prefix, games) = line.as_ref().split(':').collect_tuple().unwrap();

    let patterns = &["red", "green", "blue"];
    let ac = AhoCorasick::new(patterns).unwrap();

    for trial in games.split(';') {
        let mut trial_game = Game::default();
        for ball in trial.split(',') {
            if let Some(m) = ac.find(ball) {
                let color = patterns[m.pattern().as_usize()];
                let num = ball
                    .trim()
                    .split(' ')
                    .next()
                    .unwrap()
                    .parse::<i64>()
                    .unwrap();

                trial_game[color] = num
            }
        }

        if !ACTUAL.possible(&trial_game) {
            return 0;
        }
    }

    return game_prefix
        .split(' ')
        .nth(1)
        .unwrap()
        .parse::<i64>()
        .unwrap();
}

fn fewest_cubes<S: AsRef<str>>(line: S) -> i64 {
    let (_, games) = line.as_ref().split(':').collect_tuple().unwrap();

    let patterns = &["red", "green", "blue"];
    let ac = AhoCorasick::new(patterns).unwrap();

    let mut min_game = Game::default();

    for ball in games.split([';', ',']) {
        if let Some(m) = ac.find(ball) {
            let color = patterns[m.pattern().as_usize()];
            let num = ball
                .trim()
                .split(' ')
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap();

            min_game[color] = cmp::max(min_game[color], num);
        }
    }

    min_game.power()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1]).expect("Should have been able to read the file");

    println!("Part 1: {}", contents.lines().map(check_game).sum::<i64>());
    println!(
        "Part 2: {}",
        contents.lines().map(fewest_cubes).sum::<i64>()
    );
}
