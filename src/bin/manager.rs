use advent_of_code::commands;

use clap::{Parser, Subcommand};
use dotenv;
use log::info;

/// A CLI for interacting with the puzzle platform
#[derive(Parser)]
#[command(version)]
struct Arguments {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Retrieve input from platform
    Input {
        /// Year of puzzle
        year: u16,
        /// Day of puzzle
        day: u8,
    },
    /// Send answer to platform
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

    let arguments = Arguments::parse();

    match dotenv::dotenv() {
        Ok(_) => {}
        Err(e) => info!("dotenv missing: {e}"),
    };

    match arguments.command {
        Commands::Input { year, day } => commands::load_input(year, day),
        Commands::Answer {
            year,
            day,
            level,
            answer,
        } => commands::submit_answer(year, day, level, answer),
    };
}
