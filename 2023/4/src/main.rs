use std::{collections::BTreeSet, convert::identity, env, fs, process::exit};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "main.pest"]
struct Day4Parser;

fn card_num_matches(card: Pair<'_, Rule>) -> Option<(usize, Option<u32>)> {
    if card.as_rule() == Rule::EOI {
        return None;
    }

    let mut card_ident = 0;
    let mut winning_numbers: BTreeSet<i64> = BTreeSet::new();
    let mut num_winners = 0;

    for inner_card in card.into_inner() {
        match inner_card.as_rule() {
            Rule::winning_number => {
                winning_numbers.insert(inner_card.as_str().parse().unwrap());
            }
            Rule::play_number => {
                if winning_numbers
                    .get(&inner_card.as_str().parse().unwrap())
                    .is_some()
                {
                    num_winners += 1;
                }
            }
            Rule::game_prefix => {
                for inner_prefix in inner_card.into_inner() {
                    if inner_prefix.as_rule() == Rule::card_ident {
                        card_ident = inner_prefix.as_str().parse().unwrap();
                    }
                }
            }
            _ => (),
        }
    }

    if num_winners > 0 {
        return Some((card_ident, Some(num_winners)));
    }

    Some((card_ident, None))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1]).expect("Should have been able to read the file");

    let cards = Day4Parser::parse(Rule::file, &contents).expect("cannot read file");
    let winning_matches = cards.map(card_num_matches);

    println!(
        "Part 1: {}",
        winning_matches
            .clone()
            .map(|m| match m {
                Some((_, Some(v))) => 2i64.pow(v - 1),
                Some((_, None)) => 0,
                None => 0,
            })
            .sum::<i64>()
    );

    let mut copies_vec = vec![1; winning_matches.clone().flatten().count()];

    for m in winning_matches {
        let (card_ident, value) = match m {
            Some((c, Some(v))) => (c, v),
            Some((c, None)) => (c, 0),
            None => continue,
        };

        let copies = copies_vec[card_ident - 1];

        for i in 0..value {
            copies_vec[card_ident + usize::try_from(i).unwrap()] += copies;
        }
    }

    println!("Part 2: {:?}", copies_vec.iter().sum::<usize>());
}
