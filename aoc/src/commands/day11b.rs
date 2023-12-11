use std::path::PathBuf;

use clap::Parser;
use itertools::Itertools;
use ordered_float::OrderedFloat;

use crate::utils::{slurp_bytes, Grid, Point};

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day11b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day11b {
    fn main(&self) -> Result<(), DynError> {
        let mut grid = Grid::<u8>::from_file_bytes(&slurp_bytes(&self.input)?);
        let ex_rows = expansion_rows(&mut grid);
        let ex_cols = expansion_cols(&mut grid);
        let galaxies = grid
            .iter_points()
            .filter_map(|(p, v)| if *v == GALAXY { Some(p) } else { None })
            .collect::<Vec<_>>();
        let sum: usize = galaxies
            .into_iter()
            .tuple_combinations()
            .map(|(p1, p2)| {
                let dist = steps_to(&p1, &p2, &ex_rows, &ex_cols);
                // println!("Dist from {:?} to {:?} is {}", p1, p2, dist);
                dist
            })
            .sum();
        println!("Day11b Answer: {}", sum);
        Ok(())
    }
}

/// Find the minimal number of whole number steps to get from self to other, accounting for zones that expand
pub fn steps_to(a: &Point, other: &Point, ex_rows: &[usize], ex_cols: &[usize]) -> usize {
    let mut current = a.clone();
    let mut steps = 0;
    let directions_to_try = &mut [current; 4];
    while current != *other {
        let past = current;
        directions_to_try.fill(current);
        directions_to_try[0].x += 1;
        directions_to_try[1].x = directions_to_try[1].x.saturating_sub(1);
        directions_to_try[2].y += 1;
        directions_to_try[3].y = directions_to_try[3].y.saturating_sub(1);

        // The distance doesn't need to account for expansions
        current =
            *directions_to_try.iter().min_by_key(|d| OrderedFloat(other.distance(*d))).unwrap();

        // Only big step when stepping out of a zone
        if current.x.abs_diff(past.x) != 0 && ex_cols.contains(&past.x) {
            steps += 1_000_000;
        } else if current.y.abs_diff(past.y) != 0 && ex_rows.contains(&past.y) {
            steps += 1_000_000;
        } else {
            steps += 1;
        }
    }
    steps
}

pub const GALAXY: u8 = b'#';
pub const EMPTY: u8 = b'.';

pub fn expansion_rows(grid: &mut Grid<u8>) -> Vec<usize> {
    let mut rows_to_add = vec![];
    for (i, row) in grid.iter_rows().enumerate() {
        if !row.contains(&GALAXY) {
            rows_to_add.push(i)
        }
    }
    rows_to_add
}

pub fn expansion_cols(grid: &mut Grid<u8>) -> Vec<usize> {
    let mut cols_to_add = vec![];
    for (i, col) in grid.iter_cols().enumerate() {
        if !col.into_iter().any(|v| v == &GALAXY) {
            cols_to_add.push(i)
        }
    }
    cols_to_add
}
