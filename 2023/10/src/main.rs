use core::fmt;
use std::{env, error::Error, fs::File, process::exit};

use memmap2::Mmap;

#[derive(Debug, Copy, Clone)]
pub struct Index {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone)]
struct Dimensions {
    width: usize,
    height: usize,
}

// since the mmap is a newline delimited file, each line is 1 character longer than the width
// dimension
#[derive(Debug)]
pub struct Grid {
    mmap: Mmap,
    dimensions: Dimensions,
    start: Index,
}

impl Grid {
    fn _get_index(pos: usize, dimensions: &Dimensions) -> Index {
        assert!(pos < (dimensions.width + 1) * dimensions.height);
        Index {
            x: pos % (dimensions.width + 1),
            y: pos / (dimensions.width + 1),
        }
    }

    #[must_use]
    pub fn get_index(&self, pos: usize) -> Index {
        Self::_get_index(pos, &self.dimensions)
    }

    const fn get_pos(&self, index: &Index) -> Option<usize> {
        if index.x > self.dimensions.width || index.y > self.dimensions.height {
            return None;
        }

        Some(index.x + (index.y * (self.dimensions.width + 1)))
    }

    #[must_use]
    pub fn char_at_pos(&self, pos: usize) -> char {
        assert!(pos < self.mmap.len());
        self.mmap[pos] as char
    }

    fn char_at_index(&self, index: &Index) -> Option<char> {
        self.get_pos(index).map(|pos| self.mmap[pos] as char)
    }

    fn find_dimensions(mmap: &Mmap) -> Result<Dimensions, Box<dyn Error>> {
        let mut height = 0;

        let mut lines = mmap.split(|c| *c as char == '\n').peekable();
        let width = lines.peek().ok_or("Empty file")?.len();

        for line in lines.filter(|l| !l.is_empty()) {
            if line.len() != width {
                return Err("Inconsistent line length".into());
            }

            height += 1;
        }

        Ok(Dimensions { width, height })
    }
    #[must_use]
    pub fn iter(&self) -> GridIterator {
        self.into_iter()
    }

    #[must_use]
    pub fn loop_distance(&self) -> usize {
        self.iter().count() / 2
    }

    pub fn new<S: AsRef<str>>(file_name: &S) -> Result<Self, Box<dyn Error>> {
        let file = File::open(file_name.as_ref()).unwrap();
        let mmap = unsafe { Mmap::map(&file)? };

        let dimensions = Self::find_dimensions(&mmap)?;
        let pos = mmap
            .iter()
            .position(|&r| r as char == 'S')
            .ok_or("No start found")?;

        let start = Self::_get_index(pos, &dimensions);

        Ok(Self {
            mmap,
            dimensions,
            start,
        })
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

const fn calc_next_direction(prev_direction: Direction, pipe: char) -> Option<Direction> {
    match (prev_direction, pipe) {
        (Direction::North, '|') | (Direction::East, 'J') | (Direction::West, 'L') => {
            Some(Direction::North)
        }
        (Direction::North, 'F') | (Direction::East, '-') | (Direction::South, 'L') => {
            Some(Direction::East)
        }
        (Direction::North, '7') | (Direction::South, 'J') | (Direction::West, '-') => {
            Some(Direction::West)
        }
        (Direction::East, '7') | (Direction::South, '|') | (Direction::West, 'F') => {
            Some(Direction::South)
        }
        _ => None,
    }
}

#[derive(Debug)]
pub struct GridIterator<'a> {
    grid: &'a Grid,
    index: Index,
    next_index: Option<Index>,
    next_direction: Option<Direction>,
}

fn next_for_s(grid: &Grid) -> Option<(Direction, Index)> {
    let to_check = &[
        (
            Direction::North,
            Some(grid.start.x),
            grid.start.y.checked_sub(1),
        ),
        (
            Direction::East,
            grid.start.x.checked_add(1),
            Some(grid.start.y),
        ),
        (
            Direction::South,
            Some(grid.start.x),
            grid.start.y.checked_add(1),
        ),
        (
            Direction::West,
            grid.start.x.checked_sub(1),
            Some(grid.start.y),
        ),
    ];

    let mut valid = to_check
        .iter()
        .filter_map(|v| match v {
            (d, Some(x), Some(y)) => Some((d, Index { x: *x, y: *y })),
            (_, None, _) | (_, _, None) => None,
        })
        .filter_map(|(d, index)| match (d, grid.char_at_index(&index)) {
            (Direction::North, Some('|' | '7' | 'F'))
            | (Direction::East, Some('-' | 'J' | '7'))
            | (Direction::South, Some('|' | 'L' | 'J'))
            | (Direction::West, Some('-' | 'L' | 'F')) => Some((*d, index)),
            _ => None,
        });

    valid.next()
}

impl<'a> IntoIterator for &'a Grid {
    type Item = char;
    type IntoIter = GridIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let (next_direction, next_index) = match next_for_s(self) {
            Some((d, index)) => (Some(d), Some(index)),
            None => (None, None),
        };

        GridIterator {
            grid: self,
            index: self.start,
            next_index,
            next_direction,
        }
    }
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let ret = self.grid.char_at_index(&self.index);

        let next_index = self.next_index?;
        let next_direction = self.next_direction?;
        let next_char = self.grid.char_at_index(&next_index)?;

        self.next_index = match (next_direction, next_char) {
            (Direction::North, '|') => Some(Index {
                x: next_index.x,
                y: next_index.y.checked_sub(1)?,
            }),

            (Direction::North, 'F') => Some(Index {
                x: next_index.x.checked_add(1)?,
                y: next_index.y,
            }),

            (Direction::North, '7') => Some(Index {
                x: next_index.x.checked_sub(1)?,
                y: next_index.y,
            }),

            (Direction::East, '-') => Some(Index {
                x: next_index.x.checked_add(1)?,
                y: next_index.y,
            }),

            (Direction::East, '7') => Some(Index {
                x: next_index.x,
                y: next_index.y.checked_add(1)?,
            }),

            (Direction::East, 'J') => Some(Index {
                x: next_index.x,
                y: next_index.y.checked_sub(1)?,
            }),

            (Direction::South, '|') => Some(Index {
                x: next_index.x,
                y: next_index.y.checked_add(1)?,
            }),

            (Direction::South, 'L') => Some(Index {
                x: next_index.x.checked_add(1)?,
                y: next_index.y,
            }),

            (Direction::South, 'J') => Some(Index {
                x: next_index.x.checked_sub(1)?,
                y: next_index.y,
            }),

            (Direction::West, '-') => Some(Index {
                x: next_index.x.checked_sub(1)?,
                y: next_index.y,
            }),

            (Direction::West, 'F') => Some(Index {
                x: next_index.x,
                y: next_index.y.checked_add(1)?,
            }),

            (Direction::West, 'L') => Some(Index {
                x: next_index.x,
                y: next_index.y.checked_sub(1)?,
            }),

            (_, _) => None,
        };

        self.next_direction = calc_next_direction(self.next_direction?, next_char);
        self.index = next_index;

        ret
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self.mmap.iter() {
            write!(f, "{}", *c as char)?;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let grid = Grid::new(&args[1])?;
    // println!("{grid}");

    grid.into_iter();
    println!("Part 1: {}", grid.loop_distance());

    Ok(())
}
