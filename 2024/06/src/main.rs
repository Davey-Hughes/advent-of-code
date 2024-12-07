use core::fmt;
use std::{char, collections::HashSet, env, error::Error, fs, process::exit};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Guard {
    pos: (usize, usize),
    direction: u8,
}

impl Guard {
    fn turn_right(&mut self) {
        self.direction = self.direction.rotate_right(1);
    }

    fn next_move(&mut self, bounds: (usize, usize)) -> Option<(usize, usize)> {
        match self.direction {
            0b1000_1000 => Some((self.pos.0, self.pos.1.checked_sub(1)?)),
            0b0100_0100 => {
                let next_x = self.pos.0 + 1;

                if next_x >= bounds.0 {
                    return None;
                }

                Some((next_x, self.pos.1))
            }
            0b0010_0010 => {
                let next_y = self.pos.1 + 1;

                if next_y >= bounds.1 {
                    return None;
                }

                Some((self.pos.0, next_y))
            }
            0b0001_0001 => Some((self.pos.0.checked_sub(1)?, self.pos.1)),
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum MapItem {
    Box,
    Empty,
    Walked,
}

impl From<char> for MapItem {
    fn from(val: char) -> Self {
        match val {
            '#' => MapItem::Box,
            '.' => MapItem::Empty,
            '^' => MapItem::Walked,
            _ => panic!("Invalid map item: {val}"),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    field: Vec<Vec<MapItem>>,
    guard: Guard,
    start: (usize, usize),
    visited: HashSet<Guard>,
}

impl Map {
    fn move_guard(&mut self) -> Option<()> {
        if let Some((x, y)) = self
            .guard
            .next_move((self.field[0].len(), self.field.len()))
        {
            if self.visited.contains(&self.guard) {
                return None;
            }

            self.visited.insert(self.guard.clone());

            if self.field[y][x] == MapItem::Box {
                self.guard.turn_right();
            } else {
                self.guard.pos = (x, y);
                self.field[y][x] = MapItem::Walked;
            }

            return Some(());
        }

        self.visited.insert(self.guard.clone());

        None
    }

    fn walk(&mut self) -> &mut Self {
        while self.move_guard().is_some() {}

        self
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.field {
            for item in line {
                let c = match item {
                    MapItem::Box => '#',
                    MapItem::Empty => '.',
                    MapItem::Walked => 'O',
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_input(file_string: &str) -> Map {
    let mut field = vec![];
    let mut guard = Guard {
        pos: (0, 0),
        direction: 0b1000_1000_u8,
    };

    for (i, line) in file_string.lines().enumerate() {
        let mut line_vec: Vec<MapItem> = vec![];

        for (k, c) in line.chars().enumerate() {
            line_vec.push(c.into());

            if c == '^' {
                guard.pos = (k, i);
            }
        }

        field.push(line_vec);
    }

    Map {
        field,
        start: guard.pos,
        guard,
        visited: HashSet::new(),
    }
}

fn part_1(map: &mut Map) -> usize {
    map.walk();

    map.field
        .iter()
        .flatten()
        .filter(|&x| *x == MapItem::Walked)
        .count()
}

fn part_2(map: &mut Map) -> usize {
    let mut visited = HashSet::new();

    let visited_guards = map.clone().walk().visited.clone();

    for guard in visited_guards {
        visited.insert(guard.pos);
    }

    let mut cycles = 0;

    for item in visited {
        let mut cloned_map = map.clone();

        if item == cloned_map.start {
            continue;
        }

        cloned_map.field[item.1][item.0] = MapItem::Box;

        cloned_map.walk();

        if cloned_map
            .guard
            .next_move((cloned_map.field[0].len(), cloned_map.field.len()))
            .is_some()
        {
            cycles += 1;
        }
    }

    cycles
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1])?;
    let mut map = parse_input(&contents);

    println!("Part 1: {:?}", part_1(&mut map.clone()));
    println!("Part 2: {:?}", part_2(&mut map));

    Ok(())
}
