mod day_1;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
/// The Rust-based solutions to the 2023 Advent of Code challenges.
struct Args {
    /// The calendar day to solve.
    #[arg(short, long)]
    day: Day,
    /// The challenge part to solve.
    #[arg(short, long)]
    part: Part,
    /// The path to the input data file.
    input: Option<String>,
}

#[derive(Clone, Copy, ValueEnum)]
enum Day {
    #[clap(name = "1")]
    Day1,
}

#[derive(Clone, Copy, ValueEnum)]
enum Part {
    #[clap(name = "1")]
    Part1,
    #[clap(name = "2")]
    Part2,
}

fn main() {
    let Args { day, part, input } = Args::parse();

    let input = input
        .map(|input| std::fs::read(input).expect("`--input` should point to an existing file"))
        .unwrap_or_else(|| include_bytes!("../input/day1_part1.txt").to_vec());

    match day {
        Day::Day1 => match part {
            Part::Part1 => {
                let res = day_1::part_1::solution(input.as_slice());

                println!("The answer is: {res}");
            }
            Part::Part2 => todo!(),
        },
    }
}
