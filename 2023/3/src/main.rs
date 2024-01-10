use std::{char, cmp, collections::BTreeSet, env, fs, process::exit};

use ndarray::{Array, Axis, Ix2};

type Engine = Array<char, Ix2>;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct EnginePart {
    val: i64,
    pos: (usize, usize),
    len: usize,
}

impl EnginePart {
    fn is_valid_part(&self, engine: &Engine) -> i64 {
        let nrows = engine.len_of(Axis(0));
        let ncols = engine.len_of(Axis(1));

        // check above
        let mut start_row: usize = cmp::max(0isize, isize::try_from(self.pos.0).unwrap() - 1)
            .try_into()
            .unwrap();
        let start_col: usize = cmp::max(0isize, isize::try_from(self.pos.1).unwrap() - 1)
            .try_into()
            .unwrap();

        if start_row < self.pos.0 {
            for i in start_col..cmp::min(ncols, self.pos.1 + self.len + 1) {
                if let Some(c) = engine.get((start_row, i)) {
                    if !c.is_ascii_digit() && *c != '.' {
                        return self.val;
                    }
                };
            }
        }

        // check sides
        let left: (usize, usize) = (self.pos.0, start_col);
        if left.1 != self.pos.1 {
            if let Some(c) = engine.get(left) {
                if !c.is_ascii_digit() && *c != '.' {
                    return self.val;
                }
            }
        }

        let right: (usize, usize) = (self.pos.0, cmp::min(ncols, self.pos.1 + self.len));
        if right.1 != self.pos.1 + self.len - 1 {
            if let Some(c) = engine.get(right) {
                if !c.is_ascii_digit() && *c != '.' {
                    return self.val;
                }
            }
        }

        // check below
        start_row = cmp::min(nrows, self.pos.0 + 1);

        if start_row <= nrows {
            for i in start_col..cmp::min(ncols, self.pos.1 + self.len + 1) {
                if let Some(c) = engine.get((start_row, i)) {
                    if !c.is_ascii_digit() && *c != '.' {
                        return self.val;
                    }
                };
            }
        }

        0
    }
}

fn read_engine<S: AsRef<str>>(file_string: S) -> Engine {
    let num_lines = file_string.as_ref().lines().collect::<Vec<_>>().len();
    let line_length = file_string.as_ref().lines().next().unwrap().len();

    let arr = Array::from_iter(file_string.as_ref().chars().filter(|c| *c != '\n'));
    arr.into_shape((num_lines, line_length)).unwrap()
}

fn get_all_nums(engine: &Engine) -> Vec<EnginePart> {
    let mut num_vec: Vec<char>;
    let mut all_nums: Vec<EnginePart> = vec![];

    for (row, line) in engine.outer_iter().enumerate() {
        let mut col_outer = 0;
        num_vec = vec![];
        for (col, e) in line.indexed_iter() {
            col_outer = col;
            if e.is_ascii_digit() {
                num_vec.push(*e);
            } else if !num_vec.is_empty() {
                all_nums.push(EnginePart {
                    len: num_vec.len(),
                    pos: (row, col - num_vec.len()),
                    val: num_vec
                        .into_iter()
                        .collect::<String>()
                        .parse::<i64>()
                        .unwrap(),
                });

                num_vec = vec![];
            }
        }

        if !num_vec.is_empty() {
            all_nums.push(EnginePart {
                len: num_vec.len(),
                pos: (row, col_outer - num_vec.len()),
                val: num_vec
                    .into_iter()
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap(),
            });
        }
    }

    all_nums
}

fn sum_parts(engine: &Engine) -> i64 {
    get_all_nums(engine)
        .iter()
        .map(|num| num.is_valid_part(engine))
        .sum::<i64>()
}

fn parse_part(location: (usize, usize), engine: &Engine) -> EnginePart {
    // let ncols = engine.len_of(Axis(1));
    let mut col = location.1;
    let mut num_vec: Vec<char> = vec![];

    // backtrack
    while let Some(c) = engine.get((location.0, col)) {
        if !c.is_ascii_digit() {
            col += 1;
            break;
        }

        if col == 0 {
            break;
        }

        col -= 1;
    }

    let start_col = col;

    // parse part
    while let Some(c) = engine.get((location.0, col)) {
        if !c.is_ascii_digit() {
            break;
        }

        num_vec.push(*c);

        col += 1;
    }

    EnginePart {
        val: num_vec.iter().collect::<String>().parse::<i64>().unwrap(),
        pos: (location.0, start_col),
        len: col - start_col,
    }
}

fn find_gear_numbers(gear: (usize, usize), engine: &Engine) -> BTreeSet<EnginePart> {
    let mut parts = BTreeSet::new();

    let nrows = engine.len_of(Axis(0));
    let ncols = engine.len_of(Axis(1));

    let start_row: usize = cmp::max(0isize, isize::try_from(gear.0).unwrap() - 1)
        .try_into()
        .unwrap();
    let start_col: usize = cmp::max(0isize, isize::try_from(gear.1).unwrap() - 1)
        .try_into()
        .unwrap();

    for k in start_row..cmp::min(nrows, gear.0 + 2) {
        for i in start_col..cmp::min(ncols, gear.1 + 2) {
            if let Some(c) = engine.get((k, i)) {
                if c.is_ascii_digit() {
                    parts.insert(parse_part((k, i), engine));
                }
            };
        }
    }

    parts
}

fn get_gear_ratio(gear: (usize, usize), engine: &Engine) -> i64 {
    let parts = find_gear_numbers(gear, engine);

    if parts.len() == 2 {
        return parts
            .iter()
            .map(|p| p.val)
            .reduce(|acc, v| acc * v)
            .unwrap();
    }

    0
}

fn sum_gear_ratios(engine: &Engine) -> i64 {
    let mut acc = 0;

    for (row, line) in engine.outer_iter().enumerate() {
        for (col, e) in line.indexed_iter() {
            if *e == '*' {
                acc += get_gear_ratio((row, col), engine);
            }
        }
    }

    acc
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1]).expect("Should have been able to read the file");
    let engine = read_engine(contents);
    println!("Part 1: {:?}", sum_parts(&engine));
    println!("Part 2: {:?}", sum_gear_ratios(&engine));
}
