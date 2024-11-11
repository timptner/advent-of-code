use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("missing argument for day");
    }

    let day: u8 = args[1].parse()?;

    if !(1..=25).contains(&day) {
        println!("argument must be number between 1 and 25");
    }

    let content = year2023::get_input(day)?;
    
    let answer = match day {
        1 => year2023::day01::part1(&content)?,
        _ => panic!("unsupported day"),
    };
    println!("Part 1: {answer}");

    let answer = match day {
        1 => year2023::day01::part2(&content)?,
        _ => panic!("unsupported day"),
    };
    println!("Part 2: {answer}");

    Ok(())
}
