mod models;

use clap::{Parser, Subcommand};
use dotenv;
use log::{debug, error, warn};
use models::Puzzle;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
};
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
            let puzzle = match Puzzle::new(year, day) {
                Ok(puzzle) => puzzle,
                Err(validation) => {
                    for (field, errors) in validation.field_errors()  {
                        error!("validation failed for {field}");
                        debug!("{:#?}", errors)
                    }
                    exit(1);
                }
            };
            let path = puzzle.get_input_path()?;
            if !path.exists() {
                puzzle.get_input(client)?;
            }
            println!("{}", path.to_str().unwrap());
        },
    };
    Ok(())
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
