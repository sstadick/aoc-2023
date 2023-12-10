use std::path::PathBuf;

use clap::Parser;
use rayon::prelude::*;

use crate::{
    commands::day8a::{Direction, Nodes},
    utils::slurp_file,
};

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day8b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day8b {
    fn main(&self) -> Result<(), DynError> {
        let mut lines = slurp_file::<_, String>(&self.input).map(|line| line.unwrap());
        let directions = Direction::get_directions(lines.next().unwrap());
        let _ = lines.next(); // skip empty line
        let nodes = Nodes::from_lines(lines);
        let mut positions = nodes.get_starts();

        // This is very AOC. Apparently the input is structured such that if you find the number of steps
        // to the first Z for each input node, you can then find the LCM of those lengths, which is the answer

        // iterate over starting positions
        let answer = positions
            .into_par_iter()
            .map(|p| {
                // let mut key = p;
                // let mut steps = 1;
                // for direction in directions.iter().cycle() {
                //     let next_key = nodes.get_next_node(key, *direction);
                //     if next_key[2] == b'Z' {
                //         break;
                //     }
                //     key = next_key;
                //     steps += 1;
                // }
                // steps

                // The iterator chain is a bit faster here.
                // Go through directions for this position
                directions
                    .iter()
                    .cycle()
                    // Like fold but produces an iterator
                    .scan(p, |p, direction| {
                        *p = nodes.get_next_node(p, *direction);
                        Some(p[2] == b'Z')
                    })
                    // Find the index of the position where the predicate is true
                    .position(|is_z| is_z)
                    .unwrap()
                    + 1
            })
            // Find the running LCM
            .reduce(|| 1, lcm);

        println!("Day8b answer: {}", answer);

        Ok(())
    }
}

/// LCM using GCD-euclidean algorithm.
///
/// https://www.hackertouch.com/least-common-multiple-in-rust.html
pub fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

pub fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
