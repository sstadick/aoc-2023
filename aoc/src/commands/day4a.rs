use std::{
    collections::{HashSet, VecDeque},
    ops::Range,
    path::PathBuf,
    str::FromStr,
};

use clap::Parser;

use crate::utils::{slurp_file, ParseError};

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day4a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day4a {
    fn main(&self) -> Result<(), DynError> {
        let mut score = 0;
        for card in slurp_file::<_, Card>(&self.input) {
            let card = card?;
            score += card.score_part_a();
        }
        println!("Day4a Answer: {}", score);
        Ok(())
    }
}

pub struct CardScore {
    /// 0-based index to cards, id - 1
    id: usize,
    /// The number of matches this card had
    score: usize,
}

impl CardScore {
    fn get_range(&self) -> Range<usize> {
        let start = self.id + 1;
        let end = self.id + 1 + self.score;
        start..end
    }
}

#[derive(Debug)]
pub struct Card {
    id: usize,
    winning_numbers: HashSet<usize>,
    numbers: HashSet<usize>,
}

impl Card {
    pub fn count_winning_numbers(&self) -> usize {
        // self.winning_numbers.iter().filter(|w| self.numbers.contains(w)).count()
        self.winning_numbers.intersection(&self.numbers).count()
    }

    pub fn score_part_a(&self) -> usize {
        let matches = self.count_winning_numbers();
        if matches > 0 {
            2_usize.pow(matches as u32 - 1)
        } else {
            0
        }
    }

    pub fn score_part_b(scored_cards: Vec<usize>) -> usize {
        // Read the cards and create a vec of <number of matching numbers>
        // Create a running sum
        // Create a queue of all current card copies as (index, num matches)
        // pop one off the queue, add back referencing the original list

        let mut total_cards = 0;
        let mut queue = VecDeque::from_iter(
            scored_cards.iter().copied().enumerate().map(|(id, score)| CardScore { id, score }),
        );

        while let Some(card_score) = queue.pop_front() {
            for card_index in card_score.get_range() {
                queue.push_front(CardScore { id: card_index, score: scored_cards[card_index] });
            }
            total_cards += 1;
        }
        total_cards
    }
}

impl FromStr for Card {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();
        let id = tokens
            .nth(1)
            .ok_or(ParseError::new(format!("Failed to read game id: `{}`", s)))?
            .strip_suffix(':')
            .ok_or(ParseError::new(format!("Failed to read game id: `{}`", s)))?
            .parse::<usize>()
            .map_err(|_e| ParseError::new(format!("Failed to read game id: `{}`", s)))?;

        let mut winning_numbers = HashSet::new();
        for t in tokens.by_ref().take_while(|t| *t != "|") {
            winning_numbers.insert(t.parse::<usize>().map_err(|_e| {
                ParseError::new(format!("Failed to read winning number: `{}`", s))
            })?);
        }
        let mut numbers = HashSet::new();
        for t in tokens {
            numbers.insert(t.parse::<usize>().map_err(|_e| {
                ParseError::new(format!("Failed to read winning number: `{}`", s))
            })?);
        }

        Ok(Self { id, winning_numbers, numbers })
    }
}
