use core::num;
use std::{path::PathBuf, thread::current};

use clap::Parser;

use crate::utils::slurp_bytes;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day3a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day3a {
    fn main(&self) -> Result<(), DynError> {
        let bytes = slurp_bytes(&self.input)?;
        let scheme = Schematic::from_bytes(&bytes);
        println!("Day3a Answer: {:?}", scheme.get_valid_parts_sum());
        Ok(())
    }
}

#[derive(Debug)]
pub struct Schematic {
    /// One row per row in the schematic, which contains the point coordinates a number spans
    number_points: Vec<Vec<NumGroup>>,
    symbol_points: Vec<Vec<Point>>,
}

impl Schematic {
    pub fn from_bytes(bytes: &[u8]) -> Schematic {
        let first_row_size =
            bytes.iter().enumerate().find(|(i, b)| **b == b'\n').map(|(i, b)| i).unwrap();
        let mut number_points = Vec::with_capacity(bytes.len() / first_row_size);
        let mut symbol_points = Vec::with_capacity(bytes.len() / first_row_size);
        let mut current_row_numbers = vec![];
        let mut current_row_symbols = vec![];

        let mut current_row_index = 0;
        let mut current_col_index = 0;

        let mut index = 0;

        // NOTE: Requires last line to end in newline
        while index < bytes.len() {
            let byte = bytes[index];
            if byte == b'\n' {}

            match byte {
                b'\n' => {
                    let current_numbers = std::mem::replace(&mut current_row_numbers, vec![]);
                    number_points.push(current_numbers);
                    let current_symbols = std::mem::replace(&mut current_row_symbols, vec![]);
                    symbol_points.push(current_symbols);
                    current_col_index = 0;
                    current_row_index += 1;
                    index += 1;
                }
                b'0'..=b'9' => {
                    let mut current_number = 0;
                    let original_col_index = current_col_index;
                    while index < bytes.len() && bytes[index] >= b'0' && bytes[index] <= b'9' {
                        let byte = bytes[index];
                        current_number = (current_number * 10) + (byte - 48) as usize;
                        current_col_index += 1;
                        index += 1;
                    }
                    // Walk back one to account for overrunning to the next not number value
                    current_col_index -= 1;
                    index -= 1;

                    let mut group = vec![];
                    for i in original_col_index..=current_col_index {
                        group.push(Point { x: current_row_index, y: i, value: current_number });
                    }
                    current_row_numbers.push(NumGroup { points: group });
                    index += 1;
                    current_col_index += 1;
                }
                b'!'..=b'-' | b'/' | b':'..=b'@' | b'['..=b'`' | b'{'..=b'~' => {
                    current_row_symbols.push(Point {
                        x: current_row_index,
                        y: current_col_index,
                        value: byte as usize,
                    });
                    index += 1;
                    current_col_index += 1;
                }
                _ => {
                    index += 1;
                    current_col_index += 1;
                }
            }
        }

        Schematic { number_points, symbol_points }
    }

    pub fn get_valid_parts_sum(&self) -> usize {
        let mut sum = 0;
        for (row_index, row) in self.number_points.iter().enumerate() {
            for number_group in row.iter() {
                let mut is_adjacent = false;
                for number in &number_group.points {
                    if is_adjacent {
                        break;
                    }
                    // Check above
                    if !is_adjacent && row_index > 0 {
                        let symbol_row = &self.symbol_points[row_index - 1];
                        for symbol in symbol_row {
                            if number.is_adjacent(symbol) {
                                sum += number.value;
                                is_adjacent = true;
                                break;
                            }
                        }
                    }

                    // Check same row
                    if !is_adjacent {
                        let symbol_row = &self.symbol_points[row_index];
                        for symbol in symbol_row {
                            if number.is_adjacent(symbol) {
                                sum += number.value;
                                is_adjacent = true;
                                break;
                            }
                        }
                    }

                    // Check below
                    if !is_adjacent && row_index + 1 < self.symbol_points.len() {
                        let symbol_row = &self.symbol_points[row_index + 1];
                        for symbol in symbol_row {
                            if number.is_adjacent(symbol) {
                                sum += number.value;
                                is_adjacent = true;
                                break;
                            }
                        }
                    }
                }
            }
        }
        sum
    }

    /// A gear is a `*` symbol adjacent to two part numbers, and the gear ratios is the product of the two part numbers.
    pub fn get_sum_of_gear_ratios(&self) -> usize {
        let mut sum = 0;

        for (row_index, row) in self.symbol_points.iter().enumerate() {
            for part in row.iter().filter(|p| p.value == b'*' as usize) {
                let mut number_one = None;
                let mut number_two = None;
                let mut third = false;

                // Look above
                if row_index > 0 {
                    let groups = &self.number_points[row_index - 1];
                    for group in groups {
                        for number in &group.points {
                            if part.is_adjacent(number) {
                                if number_one.is_none() {
                                    number_one = Some(number.value);
                                } else if number_two.is_none() {
                                    number_two = Some(number.value);
                                } else {
                                    third = true;
                                }
                                break;
                            }
                        }
                    }
                }

                // Look same row
                if !(number_one.is_some() && number_two.is_some() && third) {
                    let groups = &self.number_points[row_index];
                    for group in groups {
                        for number in &group.points {
                            if part.is_adjacent(number) {
                                if number_one.is_none() {
                                    number_one = Some(number.value);
                                } else if number_two.is_none() {
                                    number_two = Some(number.value);
                                } else {
                                    third = true;
                                }
                                break;
                            }
                        }
                    }
                }

                // Look below
                if !(number_one.is_some() && number_two.is_some() && third)
                    && row_index + 1 < self.number_points.len()
                {
                    let groups = &self.number_points[row_index + 1];
                    for group in groups {
                        for number in &group.points {
                            if part.is_adjacent(number) {
                                if number_one.is_none() {
                                    number_one = Some(number.value);
                                } else if number_two.is_none() {
                                    number_two = Some(number.value);
                                } else {
                                    third = true;
                                }
                                break;
                            }
                        }
                    }
                }

                if number_one.is_some() && number_two.is_some() && !third {
                    sum += (number_one.unwrap() * number_two.unwrap())
                }
            }
        }

        sum
    }
}

#[derive(Debug)]
pub struct Point {
    x: usize,
    y: usize,
    value: usize,
}

impl Point {
    fn is_adjacent(&self, other: &Point) -> bool {
        let dx = (self.x as isize - other.x as isize).abs();
        let dy = (self.y as isize - other.y as isize).abs();
        dx <= 1 && dy <= 1
    }
}

/// The grouping of points that a number covers
#[derive(Debug)]
pub struct NumGroup {
    points: Vec<Point>,
}
