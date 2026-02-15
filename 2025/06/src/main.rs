use std::{env, error::Error, fs::read_to_string};

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

impl TryFrom<char> for Operation {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '+' => Ok(Operation::Add),
            '*' => Ok(Operation::Multiply),
            _ => Err(format!("Invalid operation character: {}", c)),
        }
    }
}

impl Operation {
    fn apply(&self, numbers: &[i64]) -> i64 {
        match self {
            Operation::Add => numbers.iter().sum(),
            Operation::Multiply => numbers.iter().product(),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Column {
    numbers: Vec<i64>,
    operation: Operation,
}

fn parse_input_part_1(input: &str) -> Vec<Column> {
    let all_lines: Vec<&str> = input.lines().collect();
    let mut line_idx = 0;

    // Parse number rows until we hit a blank line or non-number line
    let mut number_rows: Vec<Vec<i64>> = Vec::new();
    while line_idx < all_lines.len() {
        let line = all_lines[line_idx];

        // Stop if we hit a blank line or a line that doesn't start with a digit
        if line.is_empty()
            || !line
                .split_whitespace()
                .next()
                .and_then(|s| s.chars().next())
                .is_some_and(|c| c.is_ascii_digit())
        {
            break;
        }

        let row: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        number_rows.push(row);
        line_idx += 1;
    }

    // Skip blank lines to find operations row
    while line_idx < all_lines.len() && all_lines[line_idx].is_empty() {
        line_idx += 1;
    }

    // Parse the operations row
    let operations: Vec<char> = all_lines[line_idx]
        .split_whitespace()
        .map(|op| op.chars().next().unwrap())
        .collect();

    // Transpose: convert each row into an iterator, then collect columns
    let mut row_iters: Vec<_> = number_rows
        .into_iter()
        .map(std::iter::IntoIterator::into_iter)
        .collect();

    operations
        .into_iter()
        .map(|op| {
            let numbers: Vec<i64> = row_iters
                .iter_mut()
                .map(|iter| iter.next().unwrap())
                .collect();
            let operation = Operation::try_from(op).unwrap();
            Column { numbers, operation }
        })
        .collect()
}

fn parse_input_part_2(input: &str) -> Vec<Column> {
    let all_lines: Vec<&str> = input.lines().collect();
    let mut line_idx = 0;

    // Parse number rows preserving character positions
    let mut number_rows: Vec<&str> = Vec::new();
    while line_idx < all_lines.len() {
        let line = all_lines[line_idx];

        // Stop if we hit a blank line or a line that doesn't start with a digit
        if line.is_empty()
            || !line
                .chars()
                .find(|c| !c.is_whitespace())
                .is_some_and(|c| c.is_ascii_digit())
        {
            break;
        }

        number_rows.push(line);
        line_idx += 1;
    }

    // Skip blank lines to find operations row
    while line_idx < all_lines.len() && all_lines[line_idx].is_empty() {
        line_idx += 1;
    }

    // Find operations by character position
    let operations: Vec<(usize, char)> = all_lines[line_idx]
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '+' || *c == '*')
        .collect();

    // For each operation, collect numbers from character positions until the next operation
    operations
        .windows(2)
        .map(|window| {
            let (start_pos, op) = window[0];
            let (end_pos, _) = window[1];

            // Collect numbers from start_pos to end_pos (exclusive)
            let mut numbers: Vec<i64> = Vec::new();
            for char_pos in start_pos..end_pos {
                let mut current_num = String::new();
                for row in &number_rows {
                    if let Some(c) = row.chars().nth(char_pos)
                        && c.is_ascii_digit()
                    {
                        current_num.push(c);
                    }
                }
                if !current_num.is_empty() {
                    numbers.push(current_num.parse().unwrap());
                }
            }

            let operation = Operation::try_from(op).unwrap();
            Column { numbers, operation }
        })
        .chain(std::iter::once({
            // Handle the last operation
            let (start_pos, op) = operations[operations.len() - 1];
            let end_pos = all_lines[line_idx].len();

            let mut numbers: Vec<i64> = Vec::new();
            for char_pos in start_pos..end_pos {
                let mut current_num = String::new();
                for row in &number_rows {
                    if let Some(c) = row.chars().nth(char_pos)
                        && c.is_ascii_digit()
                    {
                        current_num.push(c);
                    }
                }
                if !current_num.is_empty() {
                    numbers.push(current_num.parse().unwrap());
                }
            }

            let operation = Operation::try_from(op).unwrap();
            Column { numbers, operation }
        }))
        .collect()
}

fn solve(columns: &[Column]) -> i64 {
    columns
        .iter()
        .map(|col| col.operation.apply(&col.numbers))
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;

    let columns = parse_input_part_1(&contents);
    println!("Part 1: {}", solve(&columns));

    let columns_part_2 = parse_input_part_2(&contents);
    println!("Part 2: {}", solve(&columns_part_2));

    Ok(())
}
