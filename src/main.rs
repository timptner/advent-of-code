use advent_of_code;
use clap::Parser;
use log::error;

/// Solve puzzle
#[derive(Parser)]
struct Arguments {
    /// Year of puzzle
    year: u16,
    /// Day of puzzle
    day: u8,
}

fn main() {
    let arguments = Arguments::parse();

    if !(2015..=2024).contains(&arguments.year) {
        error!("year must be number between 2015 and 2023");
        return;
    }

    if !(1..=25).contains(&arguments.day) {
        error!("day must be number between 1 and 25");
        return;
    }

    let content = advent_of_code::get_input(arguments.year, arguments.day);

    match arguments.year {
        2023 => match arguments.day {
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
