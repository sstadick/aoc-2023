use std::path::PathBuf;

use clap::Parser;

use crate::{
    commands::day10a::{Grid, Pipe},
    utils::slurp_bytes,
};

use super::{day10a::Point, CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day10b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day10b {
    fn main(&self) -> Result<(), DynError> {
        let bytes = slurp_bytes(&self.input)?;
        let grid = Grid::from_bytes(&bytes);
        let path = grid.find_loop();

        // I was on the right track but ran out of time, copied from here: https://github.com/MeisterLLD/aoc2023/blob/main/10.py
        // For each point not in the loop, count intersections to the left of it with the loop, except if they're - or J or L.
        // Crossing the loop makes you alternate (*) between in/out the regions enclosed by the loop (but crossing a - or J or L,
        // since you're going left, just makes you touching the boundary in a tangent way, so not alternating your status)
        // (*) Courtesy of Camille Jordan.

        let mut inner_points = 0;
        for x in 0..grid.grid[0].len() {
            for y in 0..grid.grid.len() {
                let point = &grid.grid[y][x];
                // Skip any points that are path points
                if path.contains(point) {
                    continue;
                }

                // Look to the West (decrease x)
                let mut is_inner = false;
                let x_delta = -1;
                let y_delta = 0;
                let mut x = (point.x as isize) + x_delta;
                let mut y = (point.y as isize) + y_delta;

                let mut crossings = 0;
                while (x as isize) >= 0
                    && (y as isize) >= 0
                    && (x as usize) < grid.grid[0].len()
                    && (y as usize) < grid.grid.len()
                {
                    // check
                    let point_to_inspect = &grid.grid[y as usize][x as usize];
                    if path.iter().any(|p| {
                        p == point_to_inspect
                            && p.pipe != Pipe::JBend
                            && p.pipe != Pipe::Horizontal
                            && p.pipe != Pipe::LBend
                    }) {
                        is_inner = !is_inner;
                    }

                    x = x as isize + x_delta;
                    y = y as isize + y_delta;
                }
                if is_inner {
                    inner_points += 1;
                }
            }
        }

        println!("Day10b Answer: {}", inner_points);
        Ok(())
    }
}

/// Count the number of times we cross a path point.
///
/// The will return as soon as it crosses a non-path point, or when we hit the edge of the grid in a direction
///
/// [
///  [(0,0), (1, 0), (2, 0)],
///  [(0,1), (1, 1), (2, 1)],
///  [(0,2), (1, 2), (2, 2)],
/// ]
pub fn look_for_crossings(
    point: &Point,
    grid: &Grid,
    path: &[Point],
    x_delta: isize,
    y_delta: isize,
) -> usize {
    let mut x = (point.x as isize) + x_delta;
    let mut y = (point.y as isize) + y_delta;

    let mut crossings = 0;
    while (x as isize) >= 0
        && (y as isize) >= 0
        && (x as usize) < grid.grid[0].len()
        && (y as usize) < grid.grid.len()
    {
        // check
        let point_to_inspect = &grid.grid[y as usize][x as usize];
        if path.iter().any(|p| p == point_to_inspect) {
            crossings += 1;
        }

        x = x as isize + x_delta;
        y = y as isize + y_delta;
    }
    crossings
}
