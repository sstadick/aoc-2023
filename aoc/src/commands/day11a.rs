use std::path::PathBuf;

use clap::Parser;
use itertools::Itertools;

use crate::utils::{slurp_bytes, Grid};

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day11a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day11a {
    fn main(&self) -> Result<(), DynError> {
        let mut grid = Grid::<u8>::from_file_bytes(&slurp_bytes(&self.input)?);
        add_rows(&mut grid);
        add_cols(&mut grid);
        let galaxies = grid
            .iter_points()
            .filter_map(|(p, v)| if *v == GALAXY { Some(p) } else { None })
            .collect::<Vec<_>>();
        let sum: usize = galaxies
            .into_iter()
            .tuple_combinations()
            .map(|(p1, p2)| {
                let dist = p1.steps_to(&p2);
                // println!("Dist from {:?} to {:?} is {}", p1, p2, dist);
                dist
            })
            .sum();
        println!("Day11a Answer: {}", sum);
        Ok(())
    }
}

pub const GALAXY: u8 = b'#';
pub const EMPTY: u8 = b'.';

// Add rows
pub fn add_rows(grid: &mut Grid<u8>) {
    let mut rows_to_add = vec![];
    for (i, row) in grid.iter_rows().enumerate() {
        if !row.contains(&GALAXY) {
            rows_to_add.push(i)
        }
    }
    for (index_of_to_add, insert_at) in rows_to_add.into_iter().enumerate() {
        grid.insert_row_at(&vec![EMPTY; grid.row_size], insert_at + index_of_to_add);
    }
}

// Add cols
pub fn add_cols(grid: &mut Grid<u8>) {
    let mut cols_to_add = vec![];
    for (i, col) in grid.iter_cols().enumerate() {
        if !col.into_iter().any(|v| v == &GALAXY) {
            cols_to_add.push(i)
        }
    }
    for (index_of_to_add, insert_at) in cols_to_add.into_iter().enumerate() {
        grid.insert_col_at(&vec![EMPTY; grid.col_size], insert_at + index_of_to_add);
    }
}
