use std::path::PathBuf;

use clap::Parser;

use crate::utils::slurp_bytes;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day10a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day10a {
    fn main(&self) -> Result<(), DynError> {
        let bytes = slurp_bytes(&self.input)?;
        let grid = Grid::from_bytes(&bytes);
        let path = grid.find_loop();
        println!("Day10a Answer: {}", path.len() / 2);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
    None,
}

#[derive(Debug)]
pub struct Grid {
    pub grid: Vec<Vec<Point>>,
    pub start: Point,
}

impl Grid {
    pub fn find_loop(&self) -> Vec<Point> {
        let mut path = vec![];
        path.push(self.start);

        let mut came_from = Direction::None;

        loop {
            if path.len() > 1 && *path.last().unwrap() == self.start {
                break;
            }

            let current = path.last().unwrap();

            // Look at possible connection locations
            if came_from != Direction::North && current.pipe.connectable_north() {
                if let Some(north) = self.get_north(current) {
                    if north.pipe.connectable_south() {
                        path.push(*north);
                        came_from = Direction::South;
                        continue;
                    }
                }
            }

            if came_from != Direction::South && current.pipe.connectable_south() {
                if let Some(south) = self.get_south(current) {
                    if south.pipe.connectable_north() {
                        came_from = Direction::North;
                        path.push(*south);
                        continue;
                    }
                }
            }

            if came_from != Direction::East && current.pipe.connectable_east() {
                if let Some(east) = self.get_east(current) {
                    if east.pipe.connectable_west() {
                        came_from = Direction::West;
                        path.push(*east);
                        continue;
                    }
                }
            }

            if came_from != Direction::West && current.pipe.connectable_west() {
                if let Some(west) = self.get_west(current) {
                    if west.pipe.connectable_east() {
                        came_from = Direction::East;
                        path.push(*west);
                        continue;
                    }
                }
            }
        }
        path.pop();
        path
    }

    pub fn get_west(&self, point: &Point) -> Option<&Point> {
        if point.x > 0 {
            Some(&self.grid[point.y][point.x - 1])
        } else {
            None
        }
    }

    pub fn get_east(&self, point: &Point) -> Option<&Point> {
        if point.x + 1 <= self.grid[0].len() {
            Some(&self.grid[point.y][point.x + 1])
        } else {
            None
        }
    }

    pub fn get_south(&self, point: &Point) -> Option<&Point> {
        if point.y + 1 <= self.grid.len() {
            Some(&self.grid[point.y + 1][point.x])
        } else {
            None
        }
    }

    pub fn get_north(&self, point: &Point) -> Option<&Point> {
        if point.y > 0 {
            Some(&self.grid[point.y - 1][point.x])
        } else {
            None
        }
    }

    /// [
    ///  [(0,0), (1, 0), (2, 0)],
    ///  [(0,1), (1, 1), (2, 1)],
    ///  [(0,2), (1, 2), (2, 2)],
    /// ]
    pub fn from_bytes(bytes: &[u8]) -> Grid {
        let first_row_size =
            bytes.iter().enumerate().find(|(i, b)| **b == b'\n').map(|(i, b)| i).unwrap();
        let mut current_row = Vec::with_capacity(first_row_size);
        let mut rows = Vec::with_capacity(bytes.len() / first_row_size);
        let mut start = None;

        let mut current_x_index = 0;
        let mut current_y_index = 0;

        let mut index = 0;

        while index < bytes.len() {
            let byte = bytes[index];
            if byte == b'\n' {}

            match byte {
                b'\n' => {
                    let curr_row =
                        std::mem::replace(&mut current_row, Vec::with_capacity(first_row_size));
                    rows.push(curr_row);
                    current_x_index = 0;
                    current_y_index += 1;
                    index += 1;
                }
                byte => {
                    let p = Point {
                        x: current_x_index,
                        y: current_y_index,
                        pipe: Pipe::from_byte(byte),
                    };
                    if start.is_none() && p.pipe.is_start() {
                        start = Some(p);
                    }
                    current_row.push(p);
                    index += 1;
                    current_x_index += 1
                }
            }
        }
        Grid { grid: rows, start: start.unwrap() }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub pipe: Pipe,
}

impl Point {}

pub const GROUND: u8 = b'.';

/// Pipes. Every pipe in the path will have two connectinos
///
/// | is a vertical pipe connecting north and south.
/// - is a horizontal pipe connecting east and west.
/// L is a 90-degree bend connecting north and east.
/// J is a 90-degree bend connecting north and west.
/// 7 is a 90-degree bend connecting south and west.
/// F is a 90-degree bend connecting south and east.
/// . is ground; there is no pipe in this tile.
/// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Pipe {
    Vertical = b'|',
    Horizontal = b'-',
    LBend = b'L',
    JBend = b'J',
    SevenBend = b'7',
    FBend = b'F',
    Ground = b'.',
    Start = b'S',
}

impl Pipe {
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            b'|' => Self::Vertical,
            b'-' => Self::Horizontal,
            b'L' => Self::LBend,
            b'J' => Self::JBend,
            b'7' => Self::SevenBend,
            b'F' => Self::FBend,
            b'.' => Self::Ground,
            b'S' => Self::Start,
            _ => unreachable!(),
        }
    }
    pub fn is_start(&self) -> bool {
        match self {
            Self::Start => true,
            _ => false,
        }
    }

    pub fn connectable_north(&self) -> bool {
        match self {
            Pipe::Vertical => true,
            Pipe::Horizontal => false,
            Pipe::LBend => true,
            Pipe::JBend => true,
            Pipe::SevenBend => false,
            Pipe::FBend => false,
            Pipe::Ground => false,
            Pipe::Start => true,
        }
    }
    pub fn connectable_south(&self) -> bool {
        match self {
            Pipe::Vertical => true,
            Pipe::Horizontal => false,
            Pipe::LBend => false,
            Pipe::JBend => false,
            Pipe::SevenBend => true,
            Pipe::FBend => true,
            Pipe::Ground => false,
            Pipe::Start => true,
        }
    }
    pub fn connectable_east(&self) -> bool {
        match self {
            Pipe::Vertical => false,
            Pipe::Horizontal => true,
            Pipe::LBend => true,
            Pipe::JBend => false,
            Pipe::SevenBend => false,
            Pipe::FBend => true,
            Pipe::Ground => false,
            Pipe::Start => true,
        }
    }
    pub fn connectable_west(&self) -> bool {
        match self {
            Pipe::Vertical => false,
            Pipe::Horizontal => true,
            Pipe::LBend => false,
            Pipe::JBend => true,
            Pipe::SevenBend => true,
            Pipe::FBend => false,
            Pipe::Ground => false,
            Pipe::Start => true,
        }
    }
}
