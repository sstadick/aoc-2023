use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use clap::Parser;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day1b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day1b {
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
            sum += find_numbers_day_1b(&buffer) as usize;
            buffer.clear();
        }

        println!("Day1b answer: {:?}", sum);
        Ok(())
    }
}

fn find_numbers_day_1b(buffer: &[u8]) -> u8 {
    let mut first = None;
    let mut last = 0;
    let mut index = 0;
    while index < buffer.len() - 1 {
        let byte = &buffer[index];
        let b = match *byte {
            48..=57 => Some(byte - 48),
            // For words, use this trie like structure to progressively match.
            97..=122 => match *byte {
                b't' => {
                    if let Some(next_byte) = buffer.get(index + 1) {
                        match next_byte {
                            b'w' => {
                                if check_if_number(buffer, index, TWO) {
                                    Some(2)
                                } else {
                                    None
                                }
                            } // two
                            b'h' => {
                                if check_if_number(buffer, index, THREE) {
                                    Some(3)
                                } else {
                                    None
                                }
                            } // three
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
                b'f' => {
                    if let Some(next_byte) = buffer.get(index + 1) {
                        match next_byte {
                            b'o' => {
                                if check_if_number(buffer, index, FOUR) {
                                    Some(4)
                                } else {
                                    None
                                }
                            } // four
                            b'i' => {
                                if check_if_number(buffer, index, FIVE) {
                                    Some(5)
                                } else {
                                    None
                                }
                            } // five
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
                b's' => {
                    if let Some(next_byte) = buffer.get(index + 1) {
                        match next_byte {
                            b'i' => {
                                if check_if_number(buffer, index, SIX) {
                                    Some(6)
                                } else {
                                    None
                                }
                            } // six
                            b'e' => {
                                if check_if_number(buffer, index, SEVEN) {
                                    Some(7)
                                } else {
                                    None
                                }
                            } // seven
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
                b'o' => {
                    if check_if_number(buffer, index, ONE) {
                        Some(1)
                    } else {
                        None
                    }
                }
                b'e' => {
                    if check_if_number(buffer, index, EIGHT) {
                        Some(8)
                    } else {
                        None
                    }
                }
                b'n' => {
                    if check_if_number(buffer, index, NINE) {
                        Some(9)
                    } else {
                        None
                    }
                }
                b'z' => {
                    if check_if_number(buffer, index, ZERO) {
                        Some(0)
                    } else {
                        None
                    }
                }
                _ => None,
            },
            _ => None,
        };
        if let Some(b) = b {
            last = b;
            if first.is_none() {
                first = Some(last)
            }
        }
        index += 1;
    }
    (first.unwrap_or(0) * 10) + last
}

#[inline]
fn check_if_number(buffer: &[u8], start: usize, number: &[u8]) -> bool {
    let end = start + number.len();
    end < buffer.len() && &buffer[start..start + number.len()] == number
}

const ONE: &[u8] = b"one";

const TWO: &[u8] = b"two";
const THREE: &[u8] = b"three";

const FOUR: &[u8] = b"four";
const FIVE: &[u8] = b"five";

const SIX: &[u8] = b"six";
const SEVEN: &[u8] = b"seven";

const EIGHT: &[u8] = b"eight";

const NINE: &[u8] = b"nine";

const ZERO: &[u8] = b"zero";
