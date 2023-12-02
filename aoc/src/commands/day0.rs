use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day0 {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day0 {
    fn main(&self) -> Result<(), DynError> {
        println!("EX: {:?}", self.input);
        Ok(())
    }
}
