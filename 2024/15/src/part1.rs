use core::fmt;
use std::error::Error;

use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
enum Direction {
    Up = b'^',
    Down = b'v',
    Left = b'<',
    Right = b'>',
}

impl Direction {
    fn apply(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Up => Some((pos.0, pos.1.checked_sub(1)?)),
            Direction::Down => Some((pos.0, pos.1.checked_add(1)?)),
            Direction::Left => Some((pos.0.checked_sub(1)?, pos.1)),
            Direction::Right => Some((pos.0.checked_add(1)?, pos.1)),
        }
    }
}

#[derive(Debug)]
struct Robot {
    position: (usize, usize),
    moves: Vec<Direction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u8)]
enum WarehouseTile {
    #[default]
    Empty = b'.',
    Wall = b'#',
    Box = b'O',
    Robot = b'@',
}

#[derive(Debug)]
struct Warehouse {
    map: Vec<Vec<WarehouseTile>>,
    robot: Robot,
}

impl Warehouse {
    fn try_move(&mut self, pos: (usize, usize), direction: Direction) -> Option<()> {
        let (x, y) = direction.apply(pos)?;

        match self.map.get(pos.1)?.get(pos.0)? {
            WarehouseTile::Empty => Some(()),
            WarehouseTile::Wall => None,
            WarehouseTile::Robot => panic!("Should never encounter robot in try_move"),
            WarehouseTile::Box => {
                if self.try_move((x, y), direction).is_some() {
                    self.map[y][x] = WarehouseTile::Box;
                    self.map[pos.1][pos.0] = WarehouseTile::Empty;
                    return Some(());
                }

                None
            }
        }
    }

    fn move_robot(&mut self) -> usize {
        for direction in self.robot.moves.clone() {
            let (x, y) = direction.apply(self.robot.position).unwrap();

            if self.try_move((x, y), direction).is_some() {
                self.map[y][x] = WarehouseTile::Robot;
                self.map[self.robot.position.1][self.robot.position.0] = WarehouseTile::Empty;
                self.robot.position = (x, y);
            }
        }

        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, xs)| xs.iter().enumerate().map(move |(x, &tile)| (x, y, tile)))
            .filter(|(_, _, tile)| *tile == WarehouseTile::Box)
            .map(|(x, y, _)| x + y * 100)
            .sum()
    }
}

impl fmt::Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for xs in &self.map {
            for &tile in xs {
                write!(f, "{}", tile as u8 as char)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn parse_input(input_string: &str) -> Result<Warehouse, Box<dyn Error>> {
    let (map, moves) = input_string
        .split_once("\n\n")
        .ok_or("Malformed input. Expected blank line between warehouse and moves")?;

    let mut robot = Robot {
        position: (0, 0),
        moves: moves
            .bytes()
            .filter(|&b| b != b'\n')
            .map(Direction::try_from)
            .collect::<Result<Vec<_>, _>>()?,
    };

    let map = map
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(|(x, c)| {
                    if c == b'@' {
                        robot.position = (x, y);
                    }

                    WarehouseTile::from(c)
                })
                .collect()
        })
        .collect();

    Ok(Warehouse { map, robot })
}

pub fn part_1(file_string: &str) -> Result<usize, Box<dyn Error>> {
    Ok(parse_input(file_string)?.move_robot())
}
