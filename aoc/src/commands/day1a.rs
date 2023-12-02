use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use clap::Parser;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day1a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day1a {
    fn main(&self) -> Result<(), DynError> {
        let mut buffer = vec![];
        let mut reader = File::open(&self.input)?;
        let mut reader = BufReader::new(reader);
        let mut sum = 0;
        while let Ok(bytes) = reader.read_until(b'\n', &mut buffer) {
            if bytes == 0 {
                break;
            }
            // Find the first and last number in the line
            sum += find_numbers_day_1(&buffer) as usize;
            buffer.clear();
        }

        println!("Day1a answer: {:?}", sum);
        Ok(())
    }
}

fn find_numbers_day_1(buffer: &[u8]) -> u8 {
    let mut first = 0;
    let mut last = 0;
    for byte in buffer {
        match *byte {
            48..=57 => {
                let byte = byte - 48;
                last = byte;
                if first == 0 {
                    first = last;
                }
            }
            _ => (),
        }
    }

    (first * 10) + last
}
