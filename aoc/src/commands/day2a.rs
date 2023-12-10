use std::{collections::HashMap, path::PathBuf, str::FromStr};

use clap::Parser;

use crate::utils::{slurp_file, ParseError};

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day2a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day2a {
    fn main(&self) -> Result<(), DynError> {
        let games = slurp_file::<_, Game>(&self.input);
        // Which games are possible if a bag contained only 12 red, 13 green, 14 blue cubes?
        // What is the sum of the ids of the possible games?
        let mut sum = 0;
        for game in games {
            let game = game?;
            if game.check_constraints(12, 13, 14) {
                sum += game.id
            }
        }

        eprintln!("Day 2a answer: {:?}", sum);
        Ok(())
    }
}

/// A Game, consisting of the id and the sets of cubes seen.
#[derive(Debug)]
pub struct Game {
    /// The game id
    id: usize,
    /// The sets of cubes that we've seen
    // We technically only need the maxes of the seen red, green, blue cubes, but I'm betting part2 is harder
    cube_sets: Vec<CubeSet>,
}

impl Game {
    /// Check to see if the game would have been possible with the given number of cubes
    fn check_constraints(&self, red: usize, green: usize, blue: usize) -> bool {
        // Find the max of each cube color seen in the cube sets
        let max = self.find_max_seen();
        // We should never see more cubes than the suggested constraint
        max.red <= red && max.green <= green && max.blue <= blue
    }

    /// Create a cube set of max of for each color seen across cube sets in a game
    fn find_max_seen(&self) -> CubeSet {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for set in &self.cube_sets {
            max_red = set.red.max(max_red);
            max_green = set.green.max(max_green);
            max_blue = set.blue.max(max_blue);
        }
        CubeSet { red: max_red, green: max_green, blue: max_blue }
    }

    /// Find the minimal cube set that could been used for a game, which is a set made of the
    pub fn find_minimal_cube_set(&self) -> CubeSet {
        self.find_max_seen()
    }
}

impl FromStr for Game {
    type Err = ParseError;

    /// Create a game from a string that looks like: `Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(": ");
        let game = parts.next().ok_or(ParseError::new(format!("Unable to parse `{}`", s)))?;
        let cube_sets = parts.next().ok_or(ParseError::new(format!("Unable to parse `{}`", s)))?;
        let id = game
            .split_ascii_whitespace()
            .last()
            .ok_or(ParseError::new(format!("Unable to parse game id `{}`", s)))?
            .parse::<usize>()
            .map_err(|_e| ParseError::new(format!("Unable to parse game id `{}`", s)))?;

        // N.B. Right here would actually be the most optimal place to determine the maxes
        // to avoid a second pass over the cube sets later.
        let mut sets = vec![];
        for set in cube_sets.split("; ") {
            sets.push(CubeSet::from_str(set)?);
        }
        Ok(Self { id, cube_sets: sets })
    }
}

/// A set of seen cubes.
#[derive(Debug)]
pub struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl CubeSet {
    /// The "power" of the set, which is just the product of the seen cubes.
    pub fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

impl FromStr for CubeSet {
    type Err = ParseError;

    /// Create a cube set from a string like: `1 red, 2 green, 6 blue`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for cube in s.split(", ") {
            let mut tokens = cube.split_ascii_whitespace();
            let number = tokens
                .next()
                .ok_or(ParseError::new(format!("Unable to parse cube `{}`", s)))?
                .parse::<usize>()
                .map_err(|_e| ParseError::new(format!("Unable to parse cube number `{}`", s)))?;
            let color =
                tokens.next().ok_or(ParseError::new(format!("Unable to parse cube `{}`", s)))?;
            match color {
                "red" => red = number,
                "green" => green = number,
                "blue" => blue = number,
                _ => return Err(ParseError::new(format!("Unable to parse cube set `{}`", s))),
            }
        }

        Ok(Self { red, green, blue })
    }
}
