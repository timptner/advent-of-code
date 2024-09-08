use advent_of_code::{
    print_header,
    get_input,
    days::day02,
};

use std::process::exit;

const DAY: u8 = 2;
const TITLE: &str = "I Was Told There Would Be No Math";

fn main() {
    print_header(DAY, TITLE);

    let input = match get_input(DAY) {
        Ok(result) => result,
        Err(e) => {
            println!("failed to get input: {e}");
            exit(1);
        },
    };

    let result = part1(&input);
    println!("Square feet of wrapping paper: {}", result);

    let result = part2(&input);
    println!("Feet of ribbon: {}", result);
}

fn part1(input: &str) -> u32 {
    match day02::calc_wrapping_paper(input) {
        Ok(result) => result,
        Err(e) => {
            println!("part1: {e}");
            exit(1);
        },
    }
}

fn part2(input: &str) -> u32 {
    match day02::calc_ribbon(input) {
        Ok(result) => result,
        Err(e) => {
            println!("part1: {e}");
            exit(1);
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1() {
        let result = part1("2x3x4");
        assert_eq!(result, 58);

        let result = part1("1x1x10");
        assert_eq!(result, 43);
    }

    #[test]
    fn check_part2() {
        let result = part2("2x3x4");
        assert_eq!(result, 34);

        let result = part2("1x1x10");
        assert_eq!(result, 14);
    }
}
