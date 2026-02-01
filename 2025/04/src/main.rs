use std::{collections::HashSet, env, error::Error, fs::read_to_string};

fn parse_input(input: &str) -> Result<HashSet<(isize, isize)>, Box<dyn Error>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().filter(|(_, c)| *c == '@').map(
                move |(k, _)| -> Result<(isize, isize), Box<dyn Error>> {
                    Ok((isize::try_from(i)?, isize::try_from(k)?))
                },
            )
        })
        .collect::<Result<HashSet<_>, Box<dyn Error>>>()
}

fn check_adjacent(input: &HashSet<(isize, isize)>, i: isize, k: isize) -> bool {
    let count = (-1..=1)
        .flat_map(|x| (-1..=1).map(move |y| (x, y)))
        .filter_map(|(x, y)| {
            let coords = (i.checked_sub(x)?, k.checked_sub(y)?);
            if input.contains(&coords) {
                return Some(coords);
            }

            None
        })
        .count();

    count <= 4
}

fn part_1(input: &HashSet<(isize, isize)>) -> usize {
    input
        .iter()
        .filter(|(i, k)| check_adjacent(input, *i, *k))
        .count()
}

fn part_2(input: &mut HashSet<(isize, isize)>) -> usize {
    std::iter::from_fn(|| {
        let to_remove: Vec<_> = input
            .iter()
            .copied()
            .filter(|(i, k)| check_adjacent(input, *i, *k))
            .collect();

        for pos in &to_remove {
            input.remove(pos);
        }

        (!to_remove.is_empty()).then_some(to_remove.len())
    })
    .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;

    let mut rolls = parse_input(&contents)?;

    println!("Part 1: {}", part_1(&rolls));
    println!("Part 2: {}", part_2(&mut rolls));

    Ok(())
}
