pub mod day0;
pub mod day1a;
pub mod day1b;
pub mod day2a;
pub mod day2b;
pub mod day3a;
pub mod day3b;
pub mod day4a;
pub mod day4b;
pub mod day5a;
pub mod day5b;
pub mod day6a;
pub mod day6b;
pub mod day7a;
pub mod day7b;
pub mod day8a;
pub mod day8b;
pub mod day9a;
pub mod day9b;

use std::error::Error;

use enum_dispatch::enum_dispatch;

pub type DynError = Box<dyn Error + 'static>;

#[enum_dispatch]
pub trait CommandImpl {
    fn main(&self) -> Result<(), DynError>;
}
