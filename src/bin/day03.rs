use advent_of_code::{
    print_header,
    get_input,
    days::day03,
};

use std::process::exit;

const DAY: u8 = 3;
const TITLE: &str = "Perfectly Spherical Houses in a Vacuum";

fn main() {
    print_header(DAY, TITLE);

    let input = match get_input(DAY) {
        Ok(result) => result,
        Err(e) => {
            println!("failed to get input: {e}");
            exit(1);
        },
    };

    let input = input.trim();

    let result = part1(&input);
    println!("Visited houses: {}", result);

    let result = part2(&input);
    println!("Visited houses with Robo-Santa: {}", result);
}

fn part1(input: &str) -> u16 {
    let directions: Vec<char> = input.chars().collect();
    day03::count_houses(directions)
}

fn part2(input: &str) -> u16 {
    let directions: Vec<char> = input.chars().collect();
    day03::count_combined_houses(directions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1() {
        let result = part1(">");
        assert_eq!(result, 2);

        let result = part1("^>v<");
        assert_eq!(result, 4);

        let result = part1("^v^v^v^v^v");
        assert_eq!(result, 2);
    }

    #[test]
    fn check_part2() {
        let result = part2("^v");
        assert_eq!(result, 3);

        let result = part2("^>v<");
        assert_eq!(result, 3);

        let result = part2("^v^v^v^v^v");
        assert_eq!(result, 11);
    }
}
