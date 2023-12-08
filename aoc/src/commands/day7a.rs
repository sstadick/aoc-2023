use std::{cmp::Ordering, path::PathBuf, str::FromStr};

use clap::Parser;

use crate::utils::{slurp_bytes, ParseError};

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day7a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day7a {
    fn main(&self) -> Result<(), DynError> {
        let buffer = slurp_bytes(&self.input)?;
        let mut hands = parse_hands(&buffer);
        hands.sort_unstable();
        // println!("hands: {:?}", hands);
        let sum: usize = hands.iter().enumerate().map(|(index, hand)| hand.bet * (index + 1)).sum();
        println!("Day7a Answer: {}", sum);
        Ok(())
    }
}

// Mapping of ascii values for the 2, 3, 4, 5, 6, 7, 8, 9, T, J, Q, K, A cards to the index into the ordered array for them
// ASCII - MINUS 50 = Lookup index -> original value, index in stored array
// 50 - 0 -> 2 index 0
// 51 - 1 -> 3 index 1
// 52 - 2 -> 4 index 2
// 53 - 3 -> 5 index 3
// 54 - 4 -> 6 index 4
// 55 - 5 -> 7 index 5
// 56 - 6 -> 8 index 6
// 57 - 7 -> 9 index 7
// 84 - 34 -> T index 8
// 74 - 24 -> J index 9
// 81 - 31 -> Q index 10
// 75 - 25 -> K index 11
// 65 - 15 -> A index 12
const CARD_TO_INDEX: &[usize; 35] = &[
    0, 1, 2, 3, 4, 5, 6, 7, 0, 0, // 0-9
    0, 0, 0, 0, 0, 12, 0, 0, 0, 0, // 10-19
    0, 0, 0, 0, 9, 11, 0, 0, 0, 0, // 20-29
    0, 10, 0, 0, 8, // 30-34
];

// const INDEX_TO_VALUE: &[usize; 13] = []

#[derive(Debug, Eq, PartialEq)]
pub struct Hand<'a> {
    kind: HandKind,
    hand: &'a [u8],
    bet: usize,
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                for (left, right) in self.hand.iter().zip(other.hand.iter()) {
                    match CARD_TO_INDEX[(*left as usize) - 50]
                        .cmp(&CARD_TO_INDEX[(*right as usize) - 50])
                    {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => (),
                        Ordering::Greater => return Ordering::Greater,
                    }
                }
                Ordering::Equal
            }
            Ordering::Greater => Ordering::Greater,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandKind {
    pub fn get_kind(hand: &[u8; 13]) -> Self {
        let mut is_three_of_kind = false;
        let mut is_one_pair = false;

        for count in hand.iter() {
            if *count == 5 {
                return Self::FiveOfAKind;
            } else if *count == 4 {
                return Self::FourOfAKind;
            }

            if *count == 3 && is_one_pair {
                return Self::FullHouse;
            } else if *count == 3 {
                is_three_of_kind = true;
                continue;
            }

            if *count == 2 && is_three_of_kind {
                return Self::FullHouse;
            } else if *count == 2 && is_one_pair {
                return Self::TwoPair;
            } else if *count == 2 {
                is_one_pair = true;
                continue;
            }
        }

        if is_three_of_kind {
            Self::ThreeOfAKind
        } else if is_one_pair {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

pub fn parse_hands<'a>(buffer: &'a [u8]) -> Vec<Hand<'a>> {
    let mut hands = vec![];
    let mut index = 0;
    while index < buffer.len() {
        let mut cards: [u8; 13] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut line_length = 0;
        // Look at hand
        for i in 0..5 {
            let card = buffer[index + i];
            cards[CARD_TO_INDEX[(card - 50) as usize]] += 1;
        }
        let hand = &buffer[index..index + 5];
        line_length += 5;

        // Skip space
        line_length += 1;

        // Read the bid
        let mut bid = 0;
        while index + line_length < buffer.len() && buffer[index + line_length] != b'\n' {
            bid = (bid * 10) as usize + (buffer[index + line_length] - 48) as usize;
            line_length += 1;
        }
        // Consume newline
        line_length += 1;
        let kind = HandKind::get_kind(&cards);
        hands.push(Hand { kind, hand, bet: bid });

        // Move index to next line
        index += line_length;
    }
    hands
}
