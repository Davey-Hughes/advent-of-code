use std::{char, env, error::Error, fs, process::exit};

fn parse_input(file_string: &str) -> Vec<Vec<char>> {
    file_string
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn part_1(words: &[Vec<char>]) -> u64 {
    fn find_xmas(words: &[Vec<char>], i: usize, k: usize) -> u64 {
        let xmas = "XMAS";
        let mut res = 0;

        // right
        if k + 3 < words[i].len() {
            res += u64::from(
                xmas.chars()
                    .eq((0..4).map(|n| words[i][k + n]).collect::<Vec<_>>()),
            );
        }

        // left
        if k.checked_sub(3).is_some() {
            res += u64::from(
                xmas.chars()
                    .eq((0..4).map(|n| words[i][k - n]).collect::<Vec<_>>()),
            );
        }

        // down
        if i + 3 < words.len() {
            res += u64::from(
                xmas.chars()
                    .eq((0..4).map(|n| words[i + n][k]).collect::<Vec<_>>()),
            );
        }

        // up
        if i.checked_sub(3).is_some() {
            res += u64::from(
                xmas.chars()
                    .eq((0..4).map(|n| words[i - n][k]).collect::<Vec<_>>()),
            );
        }

        // down-right
        if i + 3 < words.len() && k + 3 < words[i].len() {
            res += u64::from(
                xmas.chars()
                    .eq((0..4).map(|n| words[i + n][k + n]).collect::<Vec<_>>()),
            );
        }

        // down-left
        if i + 3 < words.len() && k.checked_sub(3).is_some() {
            res += u64::from(
                xmas.chars()
                    .eq((0..4).map(|n| words[i + n][k - n]).collect::<Vec<_>>()),
            );
        }

        // up-right
        if i.checked_sub(3).is_some() && k + 3 < words[i].len() {
            res += u64::from(
                xmas.chars()
                    .eq((0..4).map(|n| words[i - n][k + n]).collect::<Vec<_>>()),
            );
        }

        // up-left
        if i.checked_sub(3).is_some() && k.checked_sub(3).is_some() {
            res += u64::from(
                xmas.chars()
                    .eq((0..4).map(|n| words[i - n][k - n]).collect::<Vec<_>>()),
            );
        }

        res
    }

    let mut res = 0;

    for (i, row) in words.iter().enumerate() {
        for (k, c) in row.iter().enumerate() {
            if *c == 'X' {
                res += find_xmas(words, i, k);
            }
        }
    }

    res
}
fn part_2(words: &[Vec<char>]) -> u64 {
    fn find_x_mas(words: &[Vec<char>], i: usize, k: usize) -> u64 {
        if i.checked_sub(1).is_some()
            && i + 1 < words.len()
            && k.checked_sub(1).is_some()
            && k + 1 < words[i].len()
        {
            let diag_0 = [words[i - 1][k - 1], words[i + 1][k + 1]];
            let diag_1 = [words[i - 1][k + 1], words[i + 1][k - 1]];

            if (diag_0 == ['M', 'S'] || diag_0 == ['S', 'M'])
                && (diag_1 == ['M', 'S'] || diag_1 == ['S', 'M'])
            {
                return 1;
            }
        }

        0
    }

    let mut res = 0;

    for (i, row) in words.iter().enumerate() {
        for (k, c) in row.iter().enumerate() {
            if *c == 'A' {
                res += find_x_mas(words, i, k);
            }
        }
    }

    res
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1])?;
    let words = parse_input(&contents);

    println!("Part 1: {:?}", part_1(&words));
    println!("Part 2: {:?}", part_2(&words));

    Ok(())
}
