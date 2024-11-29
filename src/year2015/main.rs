use std::fs::read_to_string;
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    file: String,
}

fn main() {
    let args = Cli::parse();
    let input = read_to_string(&args.file).expect("failed to read file");

    let mut total: u32 = 0;
    for line in input.lines() {
        let first_digit = get_first_digit(line);
        let last_digit = get_last_digit(line);
        let number: u32 = format!("{first_digit}{last_digit}").parse().expect("not a number");
        total += number;
    }
    println!("Part1: {total}");
}

fn get_first_digit(value: &str) -> u32 {
    let mut matches: Vec<u32> = Vec::new();
    for character in value.chars() {
        match character.to_digit(10) {
            Some(digit) => matches.push(digit),
            None => (),
        };
    }
    return matches[0];
}

fn get_last_digit(value: &str) -> u32 {
    let chars = value.chars();
    let mut string = String::new();
    for character in chars.rev() {
        string.push(character);
    }
    get_first_digit(&string)
}
