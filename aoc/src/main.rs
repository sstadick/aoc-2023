#[allow(unused)]
pub mod commands;
pub mod utils;

use commands::*;
use enum_dispatch::enum_dispatch;

use clap::Parser;

#[derive(Parser, Debug)]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[enum_dispatch(CommandImpl)]
#[derive(Parser, Debug)]
enum SubCommand {
    Day0(day0::Day0),
    Day1a(day1a::Day1a),
    Day1b(day1b::Day1b),
    Day2a(day2a::Day2a),
    Day2b(day2b::Day2b),
    Day3a(day3a::Day3a),
    Day3b(day3b::Day3b),
    Day4a(day4a::Day4a),
    Day4b(day4b::Day4b),
    Day5a(day5a::Day5a),
    Day5b(day5b::Day5b),
    Day6a(day6a::Day6a),
    Day6b(day6b::Day6b),
    Day7a(day7a::Day7a),
    Day7b(day7b::Day7b),
    Day8a(day8a::Day8a),
    Day8b(day8b::Day8b),
    Day9a(day9a::Day9a),
    Day9b(day9b::Day9b),
    Day10a(day10a::Day10a),
    Day10b(day10b::Day10b),
}
fn main() -> Result<(), DynError> {
    let opts = Opts::parse();

    opts.subcommand.main()
}
