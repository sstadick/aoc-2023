use std::path::PathBuf;

use clap::Parser;

use crate::{commands::day2a::Game, utils::slurp_file};

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day2b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day2b {
    fn main(&self) -> Result<(), DynError> {
        let games = slurp_file::<_, Game>(&self.input);

        // Find the minimal cube set needed for each game
        // Sum the power of each set
        let mut sum = 0;
        for game in games {
            let game = game?;
            sum += game.find_minimal_cube_set().power();
        }
        eprintln!("Day 2b answer: {:?}", sum);
        Ok(())
    }
}
