use std::{env, error::Error};

use intcode::Interpreter;

fn neighbors(
    coords: (usize, usize),
    max: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> + Clone {
    [
        (coords.0.checked_sub(1), Some(coords.1)),
        (coords.0.checked_add(1), Some(coords.1)),
        (Some(coords.0), coords.1.checked_sub(1)),
        (Some(coords.0), coords.1.checked_add(1)),
    ]
    .into_iter()
    .filter_map(|(x, y)| Some((x?, y?)))
    .filter(move |(x, y)| *x < max.0 && *y < max.1)
}

async fn part_1(program_file: &str) -> Result<usize, Box<dyn Error + Send + Sync>> {
    let mut interpreter = Interpreter::from_file(&program_file, vec![]).await?;
    interpreter.exec().await?;

    let map = interpreter
        .output_history()
        .iter()
        .map(|&x| -> Result<char, Box<dyn Error + Send + Sync>> {
            Ok(char::from(u8::try_from(x)?))
        })
        .collect::<Result<String, _>>()?
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Ok(map
        .iter()
        .enumerate()
        .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, c)| (x, y, *c)))
        .filter(|(_, _, c)| *c == '#')
        .filter(|(x, y, _)| {
            neighbors((*x, *y), (map[0].len(), map.len())).all(|(x, y)| map[y][x] == '#')
        })
        .map(|(x, y, _)| x * y)
        .sum())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let program_file = env::args().nth(1).expect("Input file expected as argument");

    println!("Part 1: {:?}", part_1(&program_file).await?);
    // println!("Part 2: {:?}", part_2(&mut robot).await?);

    Ok(())
}
