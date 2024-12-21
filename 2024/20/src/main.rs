use core::fmt;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    env,
    error::Error,
    fs,
    ops::{Add, Mul, Sub},
    usize,
};

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coords {
    x: isize,
    y: isize,
}

impl Sub for Coords {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Coords {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul for Coords {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
enum RacetrackTile {
    Wall = b'#',
    Track = b'.',
    Start = b'S',
    End = b'E',
}

impl fmt::Display for RacetrackTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<u8>::into(*self) as char)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Racetrack {
    track: Vec<Vec<RacetrackTile>>,

    start: Coords,
    end: Coords,

    path: HashMap<Coords, usize>,
}

impl Racetrack {
    #[allow(clippy::cast_sign_loss)]
    fn neighbors(
        &self,
        coords: Coords,
    ) -> impl Iterator<Item = (RacetrackTile, Coords)> + Clone + '_ {
        [
            (coords.x - 1, coords.y),
            (coords.x + 1, coords.y),
            (coords.x, coords.y - 1),
            (coords.x, coords.y + 1),
        ]
        .into_iter()
        .filter(|(x, y)| {
            *x >= 0
                && *y >= 0
                && (*x as usize) < self.track[0].len()
                && (*y as usize) < self.track.len()
        })
        .map(|(x, y)| (self.track[y as usize][x as usize], Coords { x, y }))
    }

    fn next_tile(
        &self,
        current: Coords,
        seen: &HashMap<Coords, usize>,
    ) -> Option<(RacetrackTile, Coords)> {
        self.neighbors(current)
            .filter(|(tile, _)| tile == &RacetrackTile::Track || tile == &RacetrackTile::End)
            .find(|(_, pos)| !seen.contains_key(pos))
    }

    fn init_distances(&mut self) -> &mut Self {
        self.path.insert(self.start, 0);
        let mut current = self.start;

        let mut i = 1;
        while let Some((tile, pos)) = self.next_tile(current, &self.path) {
            self.path.insert(pos, i);
            current = pos;

            if tile == RacetrackTile::End {
                break;
            }

            i += 1;
        }

        self
    }
}

impl fmt::Display for Racetrack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lines = vec![];

        for row in &self.track {
            let mut line = String::new();
            for tile in row {
                line.push_str(format!("{tile}").as_str());
            }

            lines.push(line);
        }

        write!(f, "{}", lines.join("\n"))
    }
}

fn parse_input(input_string: &str) -> Result<Racetrack, Box<dyn Error>> {
    let mut start = None;
    let mut end = None;

    let track = input_string
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(|(x, tile)| -> Result<RacetrackTile, Box<dyn Error>> {
                    let tile = RacetrackTile::try_from(tile)?;
                    match tile {
                        RacetrackTile::Start => {
                            start = Some(Coords {
                                x: isize::try_from(x)?,
                                y: isize::try_from(y)?,
                            });
                        }
                        RacetrackTile::End => {
                            end = Some(Coords {
                                x: isize::try_from(x)?,
                                y: isize::try_from(y)?,
                            });
                        }
                        _ => {}
                    }

                    Ok(tile)
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Racetrack {
        track,
        start: start.ok_or("No start tile found")?,
        end: end.ok_or("No end tile found")?,
        path: HashMap::new(),
    })
}

#[allow(clippy::cast_sign_loss)]
fn part_1(racetrack: &Racetrack) -> usize {
    let mut cheats = vec![];

    for (pos, dist) in &racetrack.path {
        for wall_neighbor in racetrack
            .neighbors(*pos)
            .filter(|n| n.0 == RacetrackTile::Wall)
        {
            let cheat_track = *pos + ((wall_neighbor.1 - *pos) * Coords { x: 2, y: 2 });
            if cheat_track.x >= 0
                && cheat_track.y >= 0
                && (cheat_track.x as usize) < racetrack.track[0].len()
                && (cheat_track.y as usize) < racetrack.track.len()
            {
                if let Some(skip_dist) = racetrack.path.get(&cheat_track) {
                    if *skip_dist > *dist {
                        cheats.push(skip_dist - *dist - 2);
                    }
                }
            }
        }
    }

    cheats.iter().filter(|c| **c >= 100).count()
}

#[allow(clippy::cast_sign_loss)]
fn part_2(racetrack: &Racetrack) -> usize {
    fn bfs(
        racetrack: &Racetrack,
        start: Coords,
        start_dist: usize,
        max_dist: usize,
    ) -> HashMap<(Coords, Coords), usize> {
        let mut seen = HashSet::new();

        let mut queue = VecDeque::new();
        queue.push_back((start, 0));

        let mut cheats: HashMap<(Coords, Coords), usize> = HashMap::new();

        while let Some((current, cheat_dist)) = queue.pop_front() {
            if !seen.contains(&current) {
                seen.insert(current);

                let tile = racetrack.track[current.y as usize][current.x as usize];

                if tile == RacetrackTile::Track || tile == RacetrackTile::End {
                    if let Some(current_dist) = racetrack.path.get(&current) {
                        if *current_dist > start_dist && current_dist - start_dist > cheat_dist {
                            let entry = cheats.entry((start, current)).or_insert(usize::MAX);
                            *entry = (*entry).min(current_dist - start_dist - cheat_dist);
                        }
                    }
                }

                if cheat_dist < max_dist {
                    queue.extend(
                        racetrack
                            .neighbors(current)
                            .map(|(_, c)| (c, cheat_dist + 1)),
                    );
                }
            }
        }

        cheats
    }

    let cheats = racetrack
        .path
        .iter()
        .map(|(pos, dist)| bfs(racetrack, *pos, *dist, 20))
        .fold(HashMap::new(), |mut acc, map| {
            acc.extend(map);
            acc
        });

    cheats.values().filter(|c| **c >= 100).count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;

    let mut racetrack = parse_input(&contents)?;
    let racetrack = racetrack.init_distances();

    println!("Part 1: {:?}", part_1(racetrack));
    println!("Part 2: {:?}", part_2(racetrack));

    Ok(())
}
