use std::{path::PathBuf, str::FromStr};

use clap::Parser;

use crate::utils::{slurp_file, ParseError};

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day9a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day9a {
    fn main(&self) -> Result<(), DynError> {
        let mut sum = 0;
        for value in slurp_file::<_, OasisValues>(&self.input) {
            let value = value?;
            let x = value.extrapolate();
            sum += x;
        }
        println!("Day9a Answer: {}", sum);
        Ok(())
    }
}

#[derive(Debug)]
pub struct OasisValues {
    pub readings: Vec<isize>,
}

impl OasisValues {
    /// Extrapolate the next value in the sequence
    pub fn extrapolate(self) -> isize {
        let mut stack = vec![];
        stack.push(self.readings);

        // Sentinel to use as we generate each new row to avoid looping over the row again to check if all zero
        let mut is_all_zero = stack.last().unwrap().iter().all(|v| *v == 0);

        // Build the stack
        while !is_all_zero {
            let prev = stack.last().unwrap();
            let mut next = Vec::with_capacity(prev.len() - 1);
            is_all_zero = true;
            // find the difference between each value in this current row
            for (a, b) in prev.iter().zip(prev.iter().skip(1)) {
                let diff = b - a;
                if diff != 0 {
                    is_all_zero = false;
                }
                next.push(diff)
            }
            stack.push(next);
        }

        // Now walk back down the stack
        let stack_len = stack.len();
        for i in (0..stack_len).rev() {
            if i == stack_len - 1 {
                // Borrowing explicitly here, and then again further down, to isolate the scope of the mutable borrow
                // otherwise it conflicts with our immutable borrow of stack to look up a row at the difference
                let mut row = &mut stack[i];
                row.push(0);
                continue;
            }
            let prev_diff = stack[i + 1].last().copied().unwrap();
            let mut row = &mut stack[i];
            let last_in_row = row.last().unwrap();

            // Find the value that would result in the difference in the row above this
            let new = last_in_row + prev_diff;
            stack[i].push(new);
        }

        stack[0].last().copied().unwrap()
    }
}

impl FromStr for OasisValues {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let readings: Result<Vec<isize>, _> =
            s.split_ascii_whitespace().map(|v| v.parse::<isize>()).collect();
        let readings =
            readings.map_err(|_e| ParseError::new(format!("Failed to parse `{}`", s)))?;
        Ok(Self { readings })
    }
}
