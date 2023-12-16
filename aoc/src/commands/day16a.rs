use std::{
    borrow::Cow,
    collections::{HashSet, VecDeque},
    fmt::Display,
    ops::{Index, IndexMut},
    path::PathBuf,
};

use clap::Parser;

use crate::utils::slurp_bytes;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day16a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day16a {
    fn main(&self) -> Result<(), DynError> {
        let bytes = slurp_bytes(&self.input)?;
        let mut grid = Grid2::new(bytes);
        // println!("{:?}", grid);
        // println!("{}", grid);
        let answer = count_energized(&grid, LightBeam::new(Point::new(0, 0), Direction::Left));
        println!("Answer Day16a: {}", answer);

        Ok(())
    }
}

pub fn count_energized(grid: &Grid2, start: LightBeam) -> usize {
    let mut beams = VecDeque::new();
    beams.push_front(start);
    let mut seen = HashSet::new();
    let mut engergized_tiles: HashSet<Point> = HashSet::new();

    while let Some(mut beam) = beams.pop_front() {
        // println!("beam: {:?}", beam);
        if seen.contains(&beam) {
            continue;
        } else {
            seen.insert(beam.clone());
            engergized_tiles.insert(beam.point);
        }
        match grid[beam.point] {
            b'.' | b'>' | b'<' | b'^' | b'v' => {
                // Pass through
                // *(&mut grid[beam.point]) = match beam.coming_from {
                //     Direction::Top => b'v',
                //     Direction::Bottom => b'^',
                //     Direction::Left => b'>',
                //     Direction::Right => b'<',
                // };
                beam.next(&grid).map(|b| beams.push_front(b));
            }
            b'|' => {
                if matches!(beam.coming_from, Direction::Left | Direction::Right) {
                    // Hitting from the left or right, split
                    beam.next_top(&grid, Direction::Bottom).map(|b| beams.push_front(b));
                    beam.next_bottom(&grid, Direction::Top).map(|b| beams.push_front(b));
                } else {
                    // Hitting from the top or bottom, passthrough
                    beam.next(&grid).map(|b| beams.push_front(b));
                }
            }
            b'-' => {
                if matches!(beam.coming_from, Direction::Left | Direction::Right) {
                    // Hitting from the left or right, passthrough
                    beam.next(&grid).map(|b| beams.push_front(b));
                } else {
                    // Hitting from the top or bottom, split
                    beam.next_left(&grid, Direction::Right).map(|b| beams.push_front(b));
                    beam.next_right(&grid, Direction::Left).map(|b| beams.push_front(b));
                }
            }
            b'/' => match beam.coming_from {
                Direction::Top => {
                    // moves left
                    beam.next_left(&grid, Direction::Right).map(|b| beams.push_front(b));
                }
                Direction::Bottom => {
                    // Moves right
                    beam.next_right(&grid, Direction::Left).map(|b| beams.push_front(b));
                }
                Direction::Left => {
                    // Moves top
                    beam.next_top(&grid, Direction::Bottom).map(|b| beams.push_front(b));
                }
                Direction::Right => {
                    // Moves bottom
                    beam.next_bottom(&grid, Direction::Top).map(|b| beams.push_front(b));
                }
            },
            b'\\' => match beam.coming_from {
                Direction::Top => {
                    beam.next_right(&grid, Direction::Left).map(|b| beams.push_front(b));
                }
                Direction::Bottom => {
                    beam.next_left(&grid, Direction::Right).map(|b| beams.push_front(b));
                }
                Direction::Left => {
                    beam.next_bottom(&grid, Direction::Top).map(|b| beams.push_front(b));
                }
                Direction::Right => {
                    beam.next_top(&grid, Direction::Bottom).map(|b| beams.push_front(b));
                }
            },
            _ => unreachable!(),
        }

        // println!("{}", grid);
    }
    // println!("{}", grid);
    engergized_tiles.len()
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct LightBeam {
    coming_from: Direction,
    point: Point,
}

impl LightBeam {
    pub fn new(point: Point, dir: Direction) -> Self {
        Self { point, coming_from: dir }
    }

    // Get next point in current direction, return None if there is no valid next point
    pub fn next(&self, grid: &Grid2) -> Option<Self> {
        // println!("Getting next location");
        match self.coming_from {
            Direction::Top => self.next_bottom(grid, self.coming_from),
            Direction::Bottom => self.next_top(grid, self.coming_from),
            Direction::Left => self.next_right(grid, self.coming_from),
            Direction::Right => self.next_left(grid, self.coming_from),
        }
    }

    pub fn next_left(&self, grid: &Grid2, coming_from: Direction) -> Option<Self> {
        // println!("Next left");
        if self.point.x > 0 {
            Some(Self { coming_from, point: Point::new(self.point.x - 1, self.point.y) })
        } else {
            None
        }
    }
    pub fn next_right(&self, grid: &Grid2, coming_from: Direction) -> Option<Self> {
        // println!("Next right");
        if self.point.x < grid.row_size - 2 {
            Some(Self { coming_from, point: Point::new(self.point.x + 1, self.point.y) })
        } else {
            None
        }
    }
    pub fn next_top(&self, grid: &Grid2, coming_from: Direction) -> Option<Self> {
        // println!("Next top");
        if self.point.y > 0 {
            Some(Self { coming_from, point: Point::new(self.point.x, self.point.y - 1) })
        } else {
            None
        }
    }
    pub fn next_bottom(&self, grid: &Grid2, coming_from: Direction) -> Option<Self> {
        // println!("Next bottom");
        if self.point.y < grid.col_size - 1 {
            Some(Self { coming_from, point: Point::new(self.point.x, self.point.y + 1) })
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Grid2 {
    data: Vec<u8>,
    pub row_size: usize,
    pub col_size: usize,
}

impl Display for Grid2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in &self.data {
            write!(f, "{}", *b as char)?;
        }
        Ok(())
    }
}

impl Grid2 {
    pub fn new(bytes: Vec<u8>) -> Self {
        assert_eq!(*bytes.last().unwrap(), b'\n', "Bytes don't end with a newline");
        let row_size =
            bytes.iter().position(|b| *b == b'\n').expect("No newlines in input data") + 1;
        let col_size = bytes.len() / row_size;

        Self { data: bytes, row_size, col_size }
    }

    pub fn get(&self, point: Point) -> Option<u8> {
        if point.y <= self.col_size && point.x <= self.row_size - 1 {
            Some(self[point])
        } else {
            None
        }
    }
}

impl Index<Point> for Grid2 {
    type Output = u8;

    /// Index by coordinate works by assuming the top left value is (0, 0):
    ///
    /// ```
    /// 0, 1, 2, 3, 4
    /// 5, 6, 7, 8, 9
    /// ```
    ///
    /// (3, 1) in this case is the value 8.
    fn index(&self, index: Point) -> &Self::Output {
        debug_assert!(index.x < self.row_size, "X Index out of bounds");

        &self.data[(self.row_size * index.y) + index.x]
    }
}

impl IndexMut<Point> for Grid2 {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        debug_assert!(index.x < self.row_size, "X Index out of bounds");
        &mut self.data[(self.row_size * index.y) + index.x]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![0, 1, 2, 3, 4, b'\n', 5, 6, 7, 8, 9, b'\n'], Grid2 { data: vec![0, 1, 2, 3, 4, b'\n', 5, 6,7, 8, 9, b'\n'], row_size: 6, col_size: 2})]
    fn test_grid_new(#[case] input: Vec<u8>, #[case] expected: Grid2) {
        let grid = Grid2::new(input);
        assert_eq!(grid, expected);
    }

    // #[rstest]
    // #[case(Grid { data: vec![0, 1, 2, 3, 4, 5, 6,7, 8, 9], row_size: 5, col_size: 2}, vec![5, 6, 7, 8, 9])]
    // fn test_row_iter(#[case] input: Grid<u8>, #[case] expected: Vec<u8>) {
    //     assert_eq!(input.iter_rows().last().unwrap(), expected)
    // }

    // #[rstest]
    // #[case(Grid { data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], row_size: 5, col_size: 2}, vec![0, 5, 1, 6, 2, 7, 3, 8, 4, 9])]
    // fn test_col_iter(#[case] input: Grid<u8>, #[case] expected: Vec<u8>) {
    //     assert_eq!(
    //         input
    //             .iter_cols()
    //             .into_iter()
    //             .map(|c| c.into_iter().copied())
    //             .flatten()
    //             .collect::<Vec<_>>(),
    //         expected
    //     )
    // }

    #[rstest]
    #[case(Point { x: 0, y: 0}, 0)]
    #[case(Point { x: 3, y: 1}, 8)]
    #[case(Point { x: 4, y: 0}, 4)]
    #[case(Point { x: 0, y: 1}, 5)]
    #[case(Point { x: 4, y: 1}, 9)]
    fn test_index(#[case] index: Point, #[case] expected: u8) {
        let grid = Grid2 {
            data: vec![0, 1, 2, 3, 4, b'\n', 5, 6, 7, 8, 9, b'\n'],
            row_size: 6,
            col_size: 2,
        };
        assert_eq!(grid[index], expected)
    }

    #[rstest]
    #[case(Point { x: 0, y: 0}, Some(0))]
    #[case(Point { x: 3, y: 1}, Some(8))]
    #[case(Point { x: 4, y: 0}, Some(4))]
    #[case(Point { x: 6, y: 0}, None)]
    #[case(Point { x: 0, y: 1}, Some(5))]
    #[case(Point { x: 4, y: 1}, Some(9))]
    #[case(Point { x: 6, y: 1}, None)]
    fn test_get(#[case] index: Point, #[case] expected: Option<u8>) {
        let grid = Grid2 {
            data: vec![0, 1, 2, 3, 4, b'\n', 5, 6, 7, 8, 9, b'\n'],
            row_size: 6,
            col_size: 2,
        };
        assert_eq!(grid.get(index), expected)
    }

    // #[rstest]
    // #[case((vec![10, 11, 12, 13, 14], 1), &[0, 1, 2, 3, 4, 10, 11, 12, 13, 14, 5, 6, 7, 8, 9])]
    // fn test_insert_row(#[case] to_insert: (Vec<u8>, usize), #[case] expected: &[u8]) {
    //     let mut grid = Grid { data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], row_size: 5, col_size: 2 };
    //     grid.insert_row_at(&to_insert.0, to_insert.1);
    //     assert_eq!(&grid.data, expected);
    //     assert_eq!(&grid.iter_rows().flatten().copied().collect::<Vec<_>>(), expected)
    // }

    // #[rstest]
    // #[case((vec![10, 11], 1), &[0, 10, 1, 2, 3, 4, 5, 11, 6, 7, 8, 9])]
    // fn test_insert_col(#[case] to_insert: (Vec<u8>, usize), #[case] expected: &[u8]) {
    //     let mut grid = Grid { data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], row_size: 5, col_size: 2 };
    //     grid.insert_col_at(&to_insert.0, to_insert.1);
    //     assert_eq!(&grid.data, expected);
    //     assert_eq!(&grid.iter_rows().flatten().copied().collect::<Vec<_>>(), expected)
    // }

    // #[rstest]
    // #[case(vec![
    //     (Point {x: 0, y: 0}, 0_u8),
    //     (Point {x: 1, y: 0}, 1_u8),
    //     (Point {x: 2, y: 0}, 2_u8),
    //     (Point {x: 3, y: 0}, 3_u8),
    //     (Point {x: 4, y: 0}, 4_u8),
    //     (Point {x: 0, y: 1}, 5_u8),
    //     (Point {x: 1, y: 1}, 6_u8),
    //     (Point {x: 2, y: 1}, 7_u8),
    //     (Point {x: 3, y: 1}, 8_u8),
    //     (Point {x: 4, y: 1}, 9_u8),
    // ])]
    // fn test_point_iter(#[case] expected: Vec<(Point, u8)>) {
    //     let grid = Grid { data: vec![0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9], row_size: 5, col_size: 2 };
    //     let found = grid.iter_points().collect::<Vec<_>>();
    //     assert_eq!(found.len(), expected.len());
    //     for (f, e) in found.into_iter().zip(expected.into_iter()) {
    //         assert_eq!(f.0, e.0);
    //         assert_eq!(*f.1, e.1);
    //     }
    // }
}
