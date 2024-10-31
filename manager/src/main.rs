mod models;

use clap::{Parser, Subcommand};
use dotenv;
use log::{debug, error, warn};
use models::{Answer, Puzzle};
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
};
use validator::ValidationErrors;
use std::{env, error::Error, process::exit};

/// A CLI for interacting with the puzzle platform
#[derive(Parser)]
#[command(version)]
struct Args {
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
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    match dotenv::dotenv() {
        Ok(_) => {},
        Err(error) => {
            warn!(error:err; "failed to load .env");
        },
    }

    env_logger::init();

    let token = match env::var("SESSION_TOKEN") {
        Ok(value) => value,
        Err(_) => {
            error!("failed to get SESSION_TOKEN");
            exit(1);
        }
    };

    let args = Args::parse();

    let client = prepare_client(&token);

    match args.command {
        Commands::Input {year, day} => {
            let puzzle = create_puzzle(year, day);
            puzzle.get_input(client)?;
        },
        Commands::Answer { year, day, level, answer } => {
            let puzzle = create_puzzle(year, day);
            let answer = match Answer::new(level, answer) {
                Ok(answer) => answer,
                Err(validation) => {
                    log_validation_errors(validation);
                    exit(1);
                }
            };
            let contents = puzzle.submit_answer(client, answer)?;
            println!("{contents}");
        }
    };
    Ok(())
}

fn create_puzzle(year: u16, day: u8) -> Puzzle {
    let puzzle = match Puzzle::new(year, day) {
        Ok(puzzle) => puzzle,
        Err(validation) => {
            log_validation_errors(validation);
            exit(1);
        }
    };
    puzzle
}

fn log_validation_errors(validation: ValidationErrors) {
    for (field, errors) in validation.field_errors()  {
        error!("validation failed for {field}");
        debug!("{:#?}", errors);
    }
}

fn prepare_client(token: &str) -> Client {
    let cookie = format!("session={}", token);
    let value = HeaderValue::from_str(&cookie).unwrap();

    let mut headers  = HeaderMap::new();
    headers.insert("Cookie", value);

    Client::builder()
        .default_headers(headers)
        .build()
        .unwrap()
}
