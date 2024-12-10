use advent_of_code::commands;
use clap::{Parser, Subcommand};
use dotenv;
use log::error;

/// Your handy tool for solving puzzles and interacting with the puzzle platform
#[derive(Parser)]
#[command(version)]
struct Arguments {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Solve puzzle and show solution
    Solve {
        /// Year of puzzle
        year: u16,
        /// Day of puzzle
        day: u8,
    },
    /// Retrieve input from platform
    Input {
        /// Year of puzzle
        year: u16,
        /// Day of puzzle
        day: u8,
    },
    /// Submit answer to platform
    Answer {
        /// Year of puzzle
        year: u16,
        /// Day of puzzle
        day: u8,
        /// Level of puzzle
        level: u8,
        /// Answer to submit
        answer: String,
    },
}

fn main() {
    env_logger::init();
    let _ = dotenv::dotenv();
    let args = Arguments::parse();

    match args.command {
        Commands::Input { year, day } => {
            commands::load_input(year, day);
        }
        Commands::Solve { year, day } => {
            if !(2015..=2024).contains(&year) {
                error!("year must be number between 2015 and 2023");
                return;
            }

            if !(1..=25).contains(&day) {
                error!("day must be number between 1 and 25");
                return;
            }

            solve_puzzle(year, day);
        }
        Commands::Answer {
            year,
            day,
            level,
            answer,
        } => {
            commands::submit_answer(year, day, level, answer);
        }
    };
}

pub fn solve_puzzle(year: u16, day: u8) {
    let content = advent_of_code::get_input(year, day);

    match year {
        2023 => match day {
            1 => {
                use advent_of_code::year2023::day01;
                println!("Part 1: {:?}", day01::part1(&content));
                println!("Part 2: {:?}", day01::part2(&content));
            }
            _ => panic!("unexpected day"),
        },
        2024 => match day {
            1 => {
                use advent_of_code::year2024::day01;
                println!("Part 1: {:?}", day01::part1(&content));
                println!("Part 2: {:?}", day01::part2(&content));
            }
            2 => {
                use advent_of_code::year2024::day02;
                println!("Part 1: {:?}", day02::part1(&content));
                println!("Part 2: {:?}", day02::part2(&content));
            }
            3 => {
                use advent_of_code::year2024::day03;
                println!("Part 1: {:?}", day03::part1(&content));
                println!("Part 2: {:?}", day03::part2(&content));
            }
            4 => {
                use advent_of_code::year2024::day04;
                println!("Part 1: {:?}", day04::part1(&content));
                println!("Part 2: {:?}", day04::part2(&content));
            }
            5 => {
                use advent_of_code::year2024::day05;
                println!("Part 1: {:?}", day05::part1(&content));
                println!("Part 2: {:?}", day05::part2(&content));
            }
            6 => {
                use advent_of_code::year2024::day06;
                println!("Part 1: {:?}", day06::part1(&content));
                println!("Part 2: {:?}", day06::part2(&content));
            }
            7 => {
                use advent_of_code::year2024::day07;
                println!("Part 1: {:?}", day07::part1(&content));
                println!("Part 2: {:?}", day07::part2(&content));
            }
            _ => panic!("unexpected day"),
        },
        _ => panic!("unexpected year"),
    };
}
