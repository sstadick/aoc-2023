use std::path::PathBuf;

use clap::Parser;

use crate::{commands::day3a::Schematic, utils::slurp_bytes};

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day3b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day3b {
    fn main(&self) -> Result<(), DynError> {
        let bytes = slurp_bytes(&self.input)?;
        let scheme = Schematic::from_bytes(&bytes);
        println!("Day3b Answer: {:?}", scheme.get_sum_of_gear_ratios());
        Ok(())
    }
}
