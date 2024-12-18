use core::fmt;
use std::error::Error;

use num_enum::{IntoPrimitive, TryFromPrimitive};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum WarehouseTile {
    Empty = b'.',
    Wall = b'#',
    BoxLeft = b'[',
    BoxRight = b']',
    Robot = b'@',
}

impl WarehouseTile {
    fn try_from(value: u8) -> Result<Vec<Self>, Box<dyn Error>> {
        match value {
            b'.' => Ok(vec![WarehouseTile::Empty, WarehouseTile::Empty]),
            b'#' => Ok(vec![WarehouseTile::Wall, WarehouseTile::Wall]),
            b'O' => Ok(vec![WarehouseTile::BoxLeft, WarehouseTile::BoxRight]),
            b'@' => Ok(vec![WarehouseTile::Robot, WarehouseTile::Empty]),
            _ => Err("Invalid warehouse tile in input")?,
        }
    }
}

#[derive(Debug)]
struct Warehouse {
    map: Vec<Vec<WarehouseTile>>,
    robot: Robot,
}

impl Warehouse {
    fn try_move(
        &mut self,
        pos: (usize, usize),
        direction: Direction,
    ) -> Option<Vec<(WarehouseTile, (usize, usize))>> {
        let (x, y) = direction.apply(pos)?;

        let c = *self.map.get(pos.1)?.get(pos.0)?;
        match (c, direction) {
            (WarehouseTile::Empty, _) => Some(vec![]),
            (WarehouseTile::Wall, _) => None,
            (WarehouseTile::Robot, _) => panic!("Should never encounter robot in try_move"),
            (WarehouseTile::BoxLeft, Direction::Up | Direction::Down) => {
                let pair = (x + 1, y);

                if let Some(a) = self.try_move((x, y), direction) {
                    if let Some(b) = self.try_move(pair, direction) {
                        let mut res = vec![
                            (WarehouseTile::BoxLeft, (x, y)),
                            (WarehouseTile::BoxRight, (x + 1, y)),
                            (WarehouseTile::Empty, (pos.0, pos.1)),
                            (WarehouseTile::Empty, (pos.0 + 1, pos.1)),
                        ];

                        res.extend(a);
                        res.extend(b);

                        return Some(res);
                    }
                }
                None
            }
            (WarehouseTile::BoxRight, Direction::Up | Direction::Down) => {
                let pair = (x - 1, y);

                if let Some(a) = self.try_move((x, y), direction) {
                    if let Some(b) = self.try_move(pair, direction) {
                        let mut res = vec![
                            (WarehouseTile::BoxRight, (x, y)),
                            (WarehouseTile::BoxLeft, (x - 1, y)),
                            (WarehouseTile::Empty, (pos.0, pos.1)),
                            (WarehouseTile::Empty, (pos.0 - 1, pos.1)),
                        ];

                        res.extend(a);
                        res.extend(b);

                        return Some(res);
                    }
                }
                None
            }
            (
                WarehouseTile::BoxLeft | WarehouseTile::BoxRight,
                Direction::Left | Direction::Right,
            ) => {
                if let Some(a) = self.try_move((x, y), direction) {
                    let mut res = vec![(c, (x, y)), (WarehouseTile::Empty, (pos.0, pos.1))];

                    res.extend(a);
                    return Some(res);
                }
                None
            }
        }
    }

    fn move_robot(&mut self) -> usize {
        for direction in self.robot.moves.clone() {
            let (x, y) = direction.apply(self.robot.position).unwrap();

            if let Some(mut moves) = self.try_move((x, y), direction) {
                moves.sort_unstable();
                // term.write_line(format!("{:?}", moves).as_str()).unwrap();

                for m in moves {
                    self.map[m.1 .1][m.1 .0] = m.0;
                }

                self.map[y][x] = WarehouseTile::Robot;
                self.map[self.robot.position.1][self.robot.position.0] = WarehouseTile::Empty;
                self.robot.position = (x, y);
            }
        }

        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, xs)| xs.iter().enumerate().map(move |(x, &tile)| (x, y, tile)))
            .filter(|(_, _, tile)| *tile == WarehouseTile::BoxLeft)
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
    let (input_map, moves) = input_string
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

    let mut map = vec![];

    for (y, line) in input_map.lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.bytes().enumerate() {
            if c == b'@' {
                robot.position = (x * 2, y);
            }

            row.append(&mut WarehouseTile::try_from(c)?);
        }

        map.push(row);
    }

    Ok(Warehouse { map, robot })
}

pub fn part_2(file_string: &str) -> Result<usize, Box<dyn Error>> {
    Ok(parse_input(file_string)?.move_robot())
}
