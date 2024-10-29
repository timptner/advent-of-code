mod models;

use clap::{Parser, Subcommand};
use dotenv;
use log::warn;
use models::Puzzle;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
};
use std::{env, error::Error, process::exit};

/// A CLI for interacting with the puzzle platform
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Input {
        year: u16,
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
            println!("failed to get SESSION_TOKEN");
            exit(1);
        }
    };

    let args = Args::parse();

    let client = prepare_client(&token);

    match args.command {
        Commands::Input {year, day} => {
            let puzzle = Puzzle::new(year, day);
            let _content = puzzle.get_input(client)?;
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
