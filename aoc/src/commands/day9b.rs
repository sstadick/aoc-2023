use std::path::PathBuf;

use clap::Parser;

use crate::{commands::day9a::OasisValues, utils::slurp_file};

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day9b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day9b {
    fn main(&self) -> Result<(), DynError> {
        let mut sum = 0;
        for value in slurp_file::<_, OasisValues>(&self.input) {
            let mut value = value?;
            // Just flip the inputs to get the equivalent of extrapolating the starting values
            value.readings.reverse();
            let x = value.extrapolate();
            sum += x;
        }
        println!("Day9b Answer: {}", sum);
        Ok(())
    }
}
