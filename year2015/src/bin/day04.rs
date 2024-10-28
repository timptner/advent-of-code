use advent_of_code::{
    print_header,
    get_input,
    days::day04,
};

use std::process::exit;

const DAY: u8 = 4;
const TITLE: &str = "The Ideal Stocking Stuffer";

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
    println!("Floor: {}", result);

    let result = part2(&input);
    println!("Position: {}", result);
}

fn part1(input: &str) -> u32 {
    match day04::get_index(input, "00000") {
        Ok(index) => index,
        Err(e) => {
            println!("part1: {e}");
            exit(1);
        }
    }
}

fn part2(input: &str) -> u32 {
    match day04::get_index(input, "000000") {
        Ok(index) => index,
        Err(e) => {
            println!("part2: {e}");
            exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1() {
        let result = part1("abcdef");
        assert_eq!(result, 609043);
        
        let result = part1("pqrstuv");
        assert_eq!(result, 1048970);
    }
}
