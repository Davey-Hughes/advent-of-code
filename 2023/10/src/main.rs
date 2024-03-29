use core::fmt;
use std::{env, error::Error, fs::File, process::exit};

use memmap2::Mmap;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Index {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone)]
struct Dimensions {
    width: usize,
    height: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Parity {
    Clockwise = 1,
    Counterclockwise = -1,
}

// since the mmap is a newline delimited file, each line is 1 character longer than the width
// dimension
#[derive(Debug)]
pub struct Grid {
    mmap: Mmap,
    dimensions: Dimensions,
    start: Index,
    parity: Option<Parity>,
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

    /// # Panics
    ///
    /// Panics if pos is larger than the length of the file
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

    // positive parity means clockwise
    // negative parity means counterclockwise
    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn get_parity(&mut self) -> Parity {
        if let Some(parity) = self.parity {
            return parity;
        }

        let mut parity = 0;
        for (_, _, p) in self.iter() {
            parity = p;
        }

        let parity = if parity.is_positive() {
            Parity::Clockwise
        } else {
            Parity::Counterclockwise
        };

        self.parity = Some(parity);
        parity
    }

    fn valid_start_directions(&self) -> Vec<(Direction, Index)> {
        let to_check = &[
            (
                Direction::North,
                Some(self.start.x),
                self.start.y.checked_sub(1),
            ),
            (
                Direction::East,
                self.start.x.checked_add(1),
                Some(self.start.y),
            ),
            (
                Direction::South,
                Some(self.start.x),
                self.start.y.checked_add(1),
            ),
            (
                Direction::West,
                self.start.x.checked_sub(1),
                Some(self.start.y),
            ),
        ];

        let valid = to_check
            .iter()
            .filter_map(|v| match v {
                (d, Some(x), Some(y)) => Some((d, Index { x: *x, y: *y })),
                (_, None, _) | (_, _, None) => None,
            })
            .filter_map(|(d, index)| match (d, self.char_at_index(&index)) {
                (Direction::North, Some('|' | '7' | 'F'))
                | (Direction::East, Some('-' | 'J' | '7'))
                | (Direction::South, Some('|' | 'L' | 'J'))
                | (Direction::West, Some('-' | 'L' | 'F')) => Some((*d, index)),
                _ => None,
            })
            .collect::<Vec<_>>();

        valid
    }

    fn next_for_s(&self) -> Option<(Direction, Index, i64)> {
        let valid = self.valid_start_directions();

        let parity = match valid.iter().map(|v| v.0).collect::<Vec<_>>().as_slice() {
            [Direction::North, Direction::South] | [Direction::East, Direction::West] => 0,
            _ => 1,
        };

        Some((valid.first()?.0, valid.first()?.1, parity))
    }

    fn num_enclosed_point_in_polygon(&mut self) -> usize {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[allow(dead_code)]
        enum Seen {
            Unknown,
            Pipe,
            Inside,
            Outside,
        }

        let s_pipe = match self
            .valid_start_directions()
            .iter()
            .map(|(d, _)| *d)
            .collect::<Vec<_>>()
            .as_slice()
        {
            [Direction::North, Direction::East] => 'L',
            [Direction::North, Direction::South] => '|',
            [Direction::North, Direction::West] => 'J',
            [Direction::East, Direction::South] => 'F',
            [Direction::East, Direction::West] => '-',
            [Direction::South, Direction::West] => '7',
            _ => panic!("Invalid start directions"),
        };

        let mut seen = vec![Seen::Unknown; self.mmap.len()];

        for (_, i, _) in self.iter() {
            let pos = self.get_pos(&i).unwrap();
            seen[pos] = Seen::Pipe;
        }

        let mut ret = 0;

        for y in 0..self.dimensions.height {
            let mut inside = false;
            for x in 0..self.dimensions.width {
                let pos = self.get_pos(&Index { x, y }).unwrap();
                match seen[pos] {
                    Seen::Unknown => {
                        if inside {
                            ret += 1;
                        }
                    }

                    Seen::Inside => panic!("Unexpected inside"),
                    Seen::Outside => (),
                    Seen::Pipe => {
                        let mut c = self.char_at_pos(pos);
                        if c == 'S' {
                            c = s_pipe;
                        }
                        match (self.get_parity(), c) {
                            (Parity::Clockwise, 'F' | '7')
                            | (Parity::Counterclockwise, 'L' | 'J')
                            | (_, '|') => inside = !inside,

                            _ => (),
                        }
                    }
                };
            }
        }

        ret
    }

    pub fn num_enclosed(&mut self) -> usize {
        self.num_enclosed_point_in_polygon()
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
            parity: None,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
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
    parity: i64,
}

impl<'a> IntoIterator for &'a Grid {
    type Item = (char, Index, i64);
    type IntoIter = GridIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let (next_direction, next_index, parity) = match self.next_for_s() {
            Some((d, index, p)) => (Some(d), Some(index), p),
            None => (None, None, 0),
        };

        GridIterator {
            grid: self,
            index: self.start,
            next_index,
            next_direction,
            parity,
        }
    }
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = (char, Index, i64);

    fn next(&mut self) -> Option<(char, Index, i64)> {
        let ret_char = self.grid.char_at_index(&self.index);

        let cur_index = self.next_index?;
        let cur_direction = self.next_direction?;
        let cur_char = self.grid.char_at_index(&cur_index)?;

        self.next_index = match (cur_direction, cur_char) {
            (Direction::North, '|') => Some(Index {
                x: cur_index.x,
                y: cur_index.y.checked_sub(1)?,
            }),

            (Direction::North, 'F') => {
                self.parity += 1;

                Some(Index {
                    x: cur_index.x.checked_add(1)?,
                    y: cur_index.y,
                })
            }

            (Direction::North, '7') => {
                self.parity -= 1;

                Some(Index {
                    x: cur_index.x.checked_sub(1)?,
                    y: cur_index.y,
                })
            }

            (Direction::East, '-') => Some(Index {
                x: cur_index.x.checked_add(1)?,
                y: cur_index.y,
            }),

            (Direction::East, '7') => {
                self.parity += 1;

                Some(Index {
                    x: cur_index.x,
                    y: cur_index.y.checked_add(1)?,
                })
            }

            (Direction::East, 'J') => {
                self.parity -= 1;

                Some(Index {
                    x: cur_index.x,
                    y: cur_index.y.checked_sub(1)?,
                })
            }

            (Direction::South, '|') => Some(Index {
                x: cur_index.x,
                y: cur_index.y.checked_add(1)?,
            }),

            (Direction::South, 'J') => {
                self.parity += 1;

                Some(Index {
                    x: cur_index.x.checked_sub(1)?,
                    y: cur_index.y,
                })
            }

            (Direction::South, 'L') => {
                self.parity -= 1;

                Some(Index {
                    x: cur_index.x.checked_add(1)?,
                    y: cur_index.y,
                })
            }

            (Direction::West, '-') => Some(Index {
                x: cur_index.x.checked_sub(1)?,
                y: cur_index.y,
            }),

            (Direction::West, 'L') => {
                self.parity += 1;

                Some(Index {
                    x: cur_index.x,
                    y: cur_index.y.checked_sub(1)?,
                })
            }

            (Direction::West, 'F') => {
                self.parity -= 1;

                Some(Index {
                    x: cur_index.x,
                    y: cur_index.y.checked_add(1)?,
                })
            }

            (_, _) => None,
        };

        self.next_direction = calc_next_direction(self.next_direction?, cur_char);
        self.index = cur_index;

        Some((ret_char?, cur_index, self.parity))
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

    let mut grid = Grid::new(&args[1])?;
    println!("Part 1: {}", grid.loop_distance());
    println!("Part 2: {}", grid.num_enclosed());

    Ok(())
}
