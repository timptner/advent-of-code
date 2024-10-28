use std::{env, process::exit};

use dotenv;
use reqwest::{blocking::ClientBuilder, header::{HeaderMap, HeaderValue}};

fn main() {
    match dotenv::dotenv() {
        Ok(_) => {},
        Err(_) => {
            println!("failed to load .env");
            exit(1);
        },
    };

    let token = match env::var("SESSION_TOKEN") {
        Ok(value) => value,
        Err(_) => {
            println!("failed to get SESSION_TOKEN");
            exit(1);
        }
    };

    let cookie = format!("session={}", token);
    let value = HeaderValue::from_str(&cookie).unwrap();

    let mut headers  = HeaderMap::new();
    headers.insert("Cookie", value);

    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();
    let body = match client.get("https://adventofcode.com/2023/day/1/input").send() {
        Ok(response) => response,
        Err(_) => {
            println!("failed to request page");
            exit(1);
        }
    };
    
    println!("{:?}", body.text());
}
