use std::{path::PathBuf, str::FromStr};

use clap::Parser;

use crate::{
    commands::day6a::{find_ways, Distance, Time},
    utils::slurp_file,
};

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day6b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day6b {
    fn main(&self) -> Result<(), DynError> {
        let lines: Result<Vec<String>, _> = slurp_file::<_, String>(&self.input).collect();
        let lines = lines?;
        let time = Time::from_str(&lines[0])?;
        let distance = Distance::from_str(&lines[1])?;

        let ways = find_ways(time.time, distance.distance);

        println!("Day6b Answer: {}", ways);
        Ok(())
    }
}
