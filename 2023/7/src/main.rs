use core::panic;
use std::{cmp::Ordering, collections::BTreeMap, env, fs, process::exit};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
enum HandStrength {
    FiveOfAKind(Card) = 7,
    FourOfAKind(Card) = 6,
    FullHouse(Card, Card) = 5,
    ThreeOfAKind(Card) = 4,
    TwoPair(Card, Card) = 3,
    OnePair(Card) = 2,
    HighCard(Card) = 1,
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    strength: HandStrength,
    bid: usize,
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.strength == other.strength
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if std::mem::discriminant(&self.strength) == std::mem::discriminant(&other.strength) {
            for (&a, &b) in self.cards.iter().zip(other.cards.iter()) {
                let ordering = a.cmp(&b);
                if ordering != Ordering::Equal {
                    return ordering;
                }
            }
        }

        self.strength.cmp(&other.strength)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_card_slice(cards: &str) -> [Card; 5] {
    cards
        .chars()
        .map(|c| match c {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("Unexpected card: {}", c),
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn get_hand_strength(cs: &[Card; 5]) -> HandStrength {
    let mut cards = *cs;
    fn frequency_map(cs: &[Card]) -> BTreeMap<Card, usize> {
        let mut map = BTreeMap::new();

        for &c in cs {
            *map.entry(c).or_insert(0) += 1;
        }

        map
    }

    fn frequency_sort(cs: &mut [Card]) {
        let freq_map = frequency_map(cs);

        cs.sort_by(|a, b| b.cmp(a));
        cs.sort_by(|a, b| freq_map.get(b).unwrap().cmp(freq_map.get(a).unwrap()));
    }

    frequency_sort(&mut cards);
    let mut freq_values = frequency_map(&cards).into_values().collect::<Vec<_>>();
    freq_values.sort_by(|a, b| b.cmp(a));

    match freq_values.as_slice() {
        [5] => HandStrength::FiveOfAKind(cards[0]),
        [4, 1] => HandStrength::FourOfAKind(cards[0]),
        [3, 2] => HandStrength::FullHouse(cards[0], cards[3]),
        [3, 1, 1] => HandStrength::ThreeOfAKind(cards[0]),
        [2, 2, 1] => HandStrength::TwoPair(cards[0], cards[2]),
        [2, 1, 1, 1] => HandStrength::OnePair(cards[0]),
        [1, 1, 1, 1, 1] => HandStrength::HighCard(cards[0]),
        _ => panic!("Unexpected frequency values: {:?}", freq_values),
    }
}

fn parse_input<S: AsRef<str>>(file_string: &S) -> impl Iterator<Item = Hand> + '_ {
    file_string
        .as_ref()
        .lines()
        .map(|s| s.split(' '))
        .map(|mut hand| {
            let cards = get_card_slice(hand.next().unwrap());
            let strength = get_hand_strength(&cards);

            Hand {
                cards,
                strength,
                bid: hand.next().unwrap().parse().unwrap(),
            }
        })
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1]).expect("Should have been able to read the file");
    let mut hands = parse_input(&contents).collect::<Vec<_>>();
    hands.sort();

    println!(
        "Part 1: {}",
        hands
            .iter()
            .enumerate()
            .map(|(i, h)| (i + 1) * h.bid)
            .sum::<usize>()
    )
}
