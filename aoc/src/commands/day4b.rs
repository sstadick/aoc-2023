use std::path::PathBuf;

use clap::Parser;

use crate::utils::slurp_file;

use super::{day4a::Card, CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day4b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day4b {
    fn main(&self) -> Result<(), DynError> {
        let mut scored_cards = vec![];
        for card in slurp_file::<_, Card>(&self.input) {
            let card = card?;
            let score = card.count_winning_numbers();
            scored_cards.push(score);
        }
        let answer = Card::score_part_b(scored_cards);
        println!("Day4b Answer: {}", answer);
        Ok(())
    }
}
