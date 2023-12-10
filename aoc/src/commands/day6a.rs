use std::{path::PathBuf, str::FromStr};

use clap::Parser;

use crate::utils::{slurp_file, ParseError};

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day6a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day6a {
    fn main(&self) -> Result<(), DynError> {
        let lines: Result<Vec<String>, _> = slurp_file::<_, String>(&self.input).collect();
        let lines = lines?;
        let times = Times::from_str(&lines[0])?;
        let distances = Distances::from_str(&lines[1])?;

        let mut product = 1;
        for (time, distance) in times.times.iter().zip(distances.distances.iter()) {
            let ways = find_ways(*time, *distance);
            product *= ways;
        }

        println!("Day6a Answer: {}", product);
        Ok(())
    }
}

/// The naive straight line approach
#[inline]
pub fn find_ways(time: usize, distance: usize) -> usize {
    let ways = 0;
    let mut wait_time = 1;
    // Find where checks go from failing to passing
    while wait_time <= time && !check(time - wait_time, distance, wait_time) {
        wait_time += 1;
    }
    let start = wait_time;

    // Find where checks go from passing to failing
    while wait_time <= time && check(time - wait_time, distance, wait_time) {
        wait_time += 1
    }
    // N.B. we are techinically over by one on wait time for the stop, but it works out since the range is exclusive.
    let stop = wait_time;
    stop - start
}

/// Check if a given time/distance/speed combo produces goes as far as the given distance
#[inline]
pub fn check(time: usize, distance: usize, speed: usize) -> bool {
    speed * time > distance
}

#[derive(Debug)]
pub struct Times {
    times: Vec<usize>,
}

impl FromStr for Times {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Result<Vec<usize>, Self::Err> = s
            .split_ascii_whitespace()
            .skip(1)
            .map(|value| {
                value
                    .parse::<usize>()
                    .map_err(|_e| ParseError::new(format!("Failed to parse `{}`", value)))
            })
            .collect();
        Ok(Self { times: values? })
    }
}

#[derive(Debug)]
pub struct Distances {
    distances: Vec<usize>,
}

impl FromStr for Distances {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Result<Vec<usize>, Self::Err> = s
            .split_ascii_whitespace()
            .skip(1)
            .map(|value| {
                value
                    .parse::<usize>()
                    .map_err(|_e| ParseError::new(format!("Failed to parse `{}`", value)))
            })
            .collect();
        Ok(Self { distances: values? })
    }
}

#[derive(Debug)]
pub struct Time {
    pub time: usize,
}

impl FromStr for Time {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let time = s
            .strip_prefix("Time:")
            .unwrap()
            .replace(" ", "")
            .parse::<usize>()
            .map_err(|_e| ParseError::new(format!("Failed to parse `{}`", s)))?;
        Ok(Self { time })
    }
}

#[derive(Debug)]
pub struct Distance {
    pub distance: usize,
}

impl FromStr for Distance {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let distance = s
            .strip_prefix("Distance:")
            .unwrap()
            .replace(" ", "")
            .parse::<usize>()
            .map_err(|_e| ParseError::new(format!("Failed to parse `{}`", s)))?;
        Ok(Self { distance })
    }
}
