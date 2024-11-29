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
    dotenv::dotenv();

    let args = Arguments::parse();

    match args.command {
        Commands::Input { year, day } => {
            commands::load_input(year, day);
        },
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
        },
        Commands::Answer {
            year,
            day,
            level,
            answer,
        } => {
            commands::submit_answer(year, day, level, answer);
        },
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
        _ => panic!("unexpected year"),
    };
}
