use std::path::PathBuf;

use clap::Parser;

use crate::{
    commands::day16a::{count_energized, Direction, Grid2, LightBeam, Point},
    utils::slurp_bytes,
};

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day16b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day16b {
    fn main(&self) -> Result<(), DynError> {
        let bytes = slurp_bytes(&self.input)?;
        let mut grid = Grid2::new(bytes);
        // println!("{:?}", grid);
        // println!("{}", grid);

        // Try all top row points
        let answer = (0..grid.row_size - 1)
            .into_iter()
            .map(|x| count_energized(&grid, LightBeam::new(Point::new(x, 0), Direction::Top)))
            .chain(
                // Left side points
                (0..grid.col_size).into_iter().map(|y| {
                    count_energized(&grid, LightBeam::new(Point::new(0, y), Direction::Left))
                }),
            )
            .chain(
                // Right side points
                (0..grid.col_size).into_iter().map(|y| {
                    count_energized(
                        &grid,
                        LightBeam::new(Point::new(grid.row_size - 2, y), Direction::Right),
                    )
                }),
            )
            .chain(
                // Bottom side points
                (0..grid.row_size - 1).into_iter().map(|x| {
                    count_energized(
                        &grid,
                        LightBeam::new(Point::new(x, grid.col_size - 1), Direction::Bottom),
                    )
                }),
            )
            .max();

        println!("Answer Day16a: {}", answer.unwrap());
        Ok(())
    }
}
