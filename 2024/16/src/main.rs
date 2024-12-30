use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    env, fs,
};

use anyhow::{Context, Result};
use num::Complex;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Reindeer {
    pos: Complex<i64>,
    dir: Complex<i64>,
}

impl PartialOrd for Reindeer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Reindeer {
    fn cmp(&self, _: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

#[derive(Debug)]
struct Maze {
    map: HashSet<Complex<i64>>,
    end: Complex<i64>,
    reindeer: Reindeer,
}

impl Maze {
    fn neighbors(
        &self,
        pos: Complex<i64>,
        direction: Complex<i64>,
    ) -> impl Iterator<Item = (Complex<i64>, Complex<i64>, i64)> + Clone + '_ {
        [
            (pos + direction, direction, 1),
            (
                pos + (direction * Complex::i()),
                (direction * Complex::i()),
                1001,
            ),
            (
                pos + (direction * -Complex::i()),
                (direction * -Complex::i()),
                1001,
            ),
        ]
        .into_iter()
        .filter(|(c, _, _)| self.map.contains(c))
    }

    #[allow(dead_code)]
    fn djikstra_simple(&self) -> Option<i64> {
        let mut heap: BinaryHeap<(i64, Reindeer)> = BinaryHeap::from([(0, self.reindeer)]);
        let mut seen = HashSet::new();

        while let Some((score, reindeer)) = heap.pop() {
            if seen.insert(reindeer.pos) {
                if reindeer.pos == self.end {
                    return Some(-score);
                }

                for (pos, direction, s) in self.neighbors(reindeer.pos, reindeer.dir) {
                    heap.push((
                        score - s,
                        Reindeer {
                            pos,
                            dir: direction,
                        },
                    ));
                }
            }
        }

        None
    }

    fn djikstra(&self) -> Option<(i64, usize)> {
        let mut heap: BinaryHeap<(i64, Reindeer, Vec<Reindeer>)> =
            BinaryHeap::from([(0, self.reindeer, vec![self.reindeer])]);

        let mut paths = vec![];
        let mut best_score = None;

        // dynamic programming
        let mut dist = HashMap::new();

        while let Some((score, reindeer, path)) = heap.pop() {
            let e = dist.entry(reindeer).or_insert(i64::MIN);
            if *e > score {
                continue;
            }
            *e = score;

            if reindeer.pos == self.end {
                best_score = Some(score);
                paths.push(path.clone());
            }

            for (pos, direction, s) in self.neighbors(reindeer.pos, reindeer.dir) {
                let next = Reindeer {
                    pos,
                    dir: direction,
                };

                let mut path = path.clone();
                path.push(next);

                heap.push((score - s, next, path.clone()));
            }
        }

        Some((
            -best_score?,
            paths
                .iter()
                .flat_map(|p| p.iter().map(|r| r.pos))
                .collect::<HashSet<_>>()
                .len(),
        ))
    }
}

fn parse_input(input_string: &str) -> Result<Maze> {
    let mut pos = None;
    let mut end = None;
    let mut map: HashSet<Complex<i64>> = HashSet::new();

    for (y, row) in input_string.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            match c {
                'S' => {
                    pos = Some(Complex::new(i64::try_from(x)?, i64::try_from(y)?));
                }
                'E' => {
                    end = Some(Complex::new(i64::try_from(x)?, i64::try_from(y)?));
                    map.insert(Complex::new(i64::try_from(x)?, i64::try_from(y)?));
                }
                '.' => {
                    map.insert(Complex::new(i64::try_from(x)?, i64::try_from(y)?));
                }
                _ => {}
            }
        }
    }

    Ok(Maze {
        map,
        end: end.context("End not found in maze")?,
        reindeer: Reindeer {
            pos: pos.context("Start not found in maze")?,
            dir: Complex::new(1, 0),
        },
    })
}

fn main() -> Result<()> {
    let contents =
        fs::read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;

    let maze = parse_input(&contents)?;
    let solve = maze.djikstra().context("No path to end found")?;

    println!("Part 1: {:?}", solve.0);
    println!("Part 2: {:?}", solve.1);

    Ok(())
}
