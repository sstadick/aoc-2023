use std::{collections::HashMap, path::PathBuf};

use clap::Parser;

use crate::utils::slurp_file;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day8a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day8a {
    fn main(&self) -> Result<(), DynError> {
        let mut lines = slurp_file::<_, String>(&self.input).map(|line| line.unwrap());
        let directions = Direction::get_directions(lines.next().unwrap());
        let _ = lines.next(); // skip empty line
        let nodes = Nodes::from_lines(lines);

        // Cycle over directions forever
        let end = &Nodes::to_id("ZZZ"); // ZZZ
        let mut key = &Nodes::to_id("AAA"); // AAA
        let mut steps = 1;

        // The for loop is faster
        // let steps = directions
        //     .into_iter()
        //     .cycle()
        //     .scan(&Nodes::to_id("AAA"), |node, dir| {
        //         *node = nodes.get_next_node(node, dir);
        //         Some(*node == end)
        //     })
        //     .position(|is_z| is_z)
        //     .unwrap()
        //     + 1;

        for direction in directions.into_iter().cycle() {
            let next_key = nodes.get_next_node(key, direction);
            if next_key == end {
                break;
            }
            key = next_key;
            steps += 1;
        }
        println!("Day8a answer: {}", steps);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left = 0,
    Right = 1,
}

impl Direction {
    pub fn get_directions(line: String) -> Vec<Direction> {
        line.chars()
            .into_iter()
            .map(|c| match c {
                'L' => Self::Left,
                'R' => Self::Right,
                _ => unreachable!(),
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct Nodes {
    lookup: HashMap<[u8; 3], [[u8; 3]; 2]>,
}

impl Nodes {
    /// Get all the keys that end with `A`.
    pub fn get_starts(&self) -> Vec<&[u8; 3]> {
        self.lookup.keys().filter(|k| k[2] == b'A').collect()
    }

    pub fn get_next_node(&self, key: &[u8; 3], dir: Direction) -> &[u8; 3] {
        let possible = self.lookup.get(key).unwrap();
        &possible[dir as usize]
    }

    pub fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut map = HashMap::new();
        for line in lines {
            let mut tokens = line.split_ascii_whitespace();
            let key = tokens.next().unwrap();
            let _ = tokens.next(); // skip =
            let left = tokens.next().unwrap().strip_prefix("(").unwrap().strip_suffix(",").unwrap();
            let right = tokens.next().unwrap().strip_suffix(")").unwrap();
            // if map.contains_key(&Self::to_id(key)) {
            //     println!("{} - {}", key, Self::to_id(key));
            //     panic!("Duplicate key");
            // }
            map.insert(Self::to_id(key), [Self::to_id(left), Self::to_id(right)]);
        }
        Self { lookup: map }
    }

    #[inline]
    pub fn to_id(node: &str) -> [u8; 3] {
        let mut id = [0; 3];
        id.copy_from_slice(node.as_bytes());
        // for b in node.as_bytes() {
        //     // Handle numbers that are already in the 10's place
        //     id = (if id < 10 { id * 10 } else { id * 100 }) + (b - b'A') as usize;
        // }
        id
    }
}
