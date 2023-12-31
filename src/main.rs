mod day_1;
mod day_2;

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
    #[clap(name = "2")]
    Day2,
}

#[derive(Clone, Copy, ValueEnum)]
enum Part {
    #[clap(name = "1")]
    Part1,
    #[clap(name = "2")]
    Part2,
}

macro_rules! generate_solutions_for_days {
    (
        $day_:expr,
        $part:expr,
        $input:expr,
        [$($day:literal),* $(,)?] $(,)?
    ) => {
        paste::paste! {
            match $day_ {
                $(
                    Day::[<Day $day>] => {
                        let part_1_input = $input
                            .as_ref()
                            .map(|input| std::fs::read_to_string(input).expect("`--input` should point to an existing file"))
                            .unwrap_or_else(||
                                    include_str!(
                                        concat!(
                                            "../input/day",
                                            $day,
                                            "_part",
                                            1,
                                            ".txt",
                                        )
                                    ).to_string()
                                );

                        let part_2_input = $input
                            .map(|input| std::fs::read_to_string(input).expect("`--input` should point to an existing file"))
                            .unwrap_or_else(||
                                    include_str!(
                                        concat!(
                                            "../input/day",
                                            $day,
                                            "_part",
                                            2,
                                            ".txt",
                                        )
                                    ).to_string()
                                );

                        let res = match $part {
                            Part::Part1 => [<day_ $day>]::part_1::solution(part_1_input.as_str()),
                            Part::Part2 => [<day_ $day>]::part_2::solution(part_2_input.as_str()),
                        };

                        println!("The answer is: {res}");
                    }
                )*
            }
        }
    };
}

fn main() {
    let Args { day, part, input } = Args::parse();

    generate_solutions_for_days!(day, part, input, [1, 2]);
}
