use std::{cmp::Ordering, path::PathBuf, str::FromStr};

use clap::Parser;

use crate::utils::{slurp_bytes, ParseError};

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day7b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day7b {
    fn main(&self) -> Result<(), DynError> {
        let buffer = slurp_bytes(&self.input)?;
        let mut hands = parse_hands(&buffer);
        hands.sort_unstable();
        // println!("hands: {:?}", hands);
        let sum: usize = hands.iter().enumerate().map(|(index, hand)| hand.bet * (index + 1)).sum();
        println!("Day7b Answer: {}", sum);
        Ok(())
    }
}

// Mapping of ascii values for the 2, 3, 4, 5, 6, 7, 8, 9, T, J, Q, K, A cards to the index into the ordered array for them
// ASCII - MINUS 50 = Lookup index -> original value, index in stored array
// 74 - 24 -> J index 0
// 50 - 0 -> 2 index 1
// 51 - 1 -> 3 index 2
// 52 - 2 -> 4 index 3
// 53 - 3 -> 5 index 4
// 54 - 4 -> 6 index 5
// 55 - 5 -> 7 index 6
// 56 - 6 -> 8 index 7
// 57 - 7 -> 9 index 8
// 84 - 34 -> T index 9
// 81 - 31 -> Q index 10
// 75 - 25 -> K index 11
// 65 - 15 -> A index 12
const CARD_TO_INDEX: &[usize; 35] = &[
    1, 2, 3, 4, 5, 6, 7, 8, 0, 0, // 0-9
    0, 0, 0, 0, 0, 12, 0, 0, 0, 0, // 10-19
    0, 0, 0, 0, 0, 11, 0, 0, 0, 0, // 20-29
    0, 10, 0, 0, 9, // 30-34
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
        let jokers = hand[0];
        // Check for 5s
        if hand.iter().skip(1).any(|v| v + jokers >= 5) {
            HandKind::FiveOfAKind
        } else if hand.iter().skip(1).any(|v| v + jokers >= 4) {
            HandKind::FourOfAKind
        } else if Self::is_full_house(hand) {
            HandKind::FullHouse
        } else if hand.iter().skip(1).any(|v| v + jokers >= 3) {
            HandKind::ThreeOfAKind
        } else if Self::is_two_pair(hand) {
            HandKind::TwoPair
        } else if hand.iter().skip(1).any(|v| v + jokers >= 2) {
            HandKind::OnePair
        } else {
            HandKind::HighCard
        }
    }

    #[inline]
    pub fn is_full_house(hand: &[u8; 13]) -> bool {
        // TODO: I think the account isn't really needed, since if we had 2 jokers and a pair, that would become 4 of a kind, not a full house
        let mut jokers = hand[0];
        // Is there a 3 of a kind, and how many jokers does that use?
        if let Some((index, count)) =
            hand.iter().enumerate().skip(1).find(|(index, value)| *value + jokers >= 3)
        {
            jokers = (*count + jokers) - 3;
            // Now look for two of a kind
            hand.iter()
                .enumerate()
                .skip(1)
                .filter(|(i, c)| *i != index)
                .any(|(_, c)| c + jokers >= 2)
        } else {
            false
        }
    }

    #[inline]
    pub fn is_two_pair(hand: &[u8; 13]) -> bool {
        let mut jokers = hand[0];
        if let Some((index, count)) =
            hand.iter().enumerate().skip(1).find(|(index, value)| *value + jokers >= 2)
        {
            jokers = (*count + jokers) - 2;
            hand.iter()
                .enumerate()
                .skip(1)
                .filter(|(i, c)| *i != index)
                .any(|(_, c)| c + jokers >= 2)
        } else {
            false
        }
    }
}

// 249106961 too low
// 249776650
// 249853776 too high

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
