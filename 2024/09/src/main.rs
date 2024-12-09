use std::{env, error::Error, fs, process::exit};

fn parse_input(file_string: &str) -> Vec<(usize, Option<usize>)> {
    file_string
        .chars()
        .filter(|x| x != &'\n')
        .enumerate()
        .map(|(i, x)| {
            let len = x.to_digit(10).expect("Illegal non-digit in input") as usize;
            if i % 2 == 0 {
                (len, Some(i / 2))
            } else {
                (len, None)
            }
        })
        .collect::<Vec<_>>()
}

fn part_1(spans: &[(usize, Option<usize>)]) -> usize {
    let mut disk = spans
        .iter()
        .flat_map(|(len, c)| (0..*len).map(move |_| c))
        .collect::<Vec<_>>();

    let mut first = 0;
    let mut last = disk.len() - 1;

    while first < last {
        if disk[first].is_some() {
            first += 1;
        } else if disk[last].is_none() {
            last -= 1;
        } else {
            disk.swap(first, last);
        }
    }

    disk.into_iter()
        .filter_map(|&x| x)
        .enumerate()
        .map(|(i, x)| i * x)
        .sum::<usize>()
}

fn part_2(spans: &mut Vec<(usize, Option<usize>)>) -> usize {
    for i in 0..spans.len() {
        if spans[i].1.is_some() {
            continue;
        }

        // search for the latest file span that will fit in the empty blocks
        for k in (i..spans.len()).rev() {
            if spans[k].1.is_none() {
                continue;
            }

            // end file exactly fits so just swap them
            if spans[i].0 == spans[k].0 {
                spans.swap(i, k);

                break;
            }

            // end file is smaller than the space, so swap and then create a span indicating number
            // of empty blocks
            //
            // the empty span that is moved to the end of the list should be merged with neighbor
            // empty blocks, but it isn't necessary for this problem
            if spans[i].0 > spans[k].0 {
                let diff = spans[i].0 - spans[k].0;

                spans[i].0 -= diff;
                spans.swap(i, k);
                spans.insert(i + 1, (diff, None));

                break;
            }
        }
    }

    spans
        .iter()
        .flat_map(|(len, c)| (0..*len).map(move |_| c))
        .enumerate()
        .fold(0, |acc, (i, x)| acc + i * x.unwrap_or(0))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1])?;

    let mut spans = parse_input(&contents);

    println!("Part 1: {:?}", part_1(&spans));
    println!("Part 2: {:?}", part_2(&mut spans));

    Ok(())
}
