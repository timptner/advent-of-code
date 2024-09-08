use advent_of_code::{
    print_header,
    get_input,
    days::day01,
};

use std::process::exit;

const DAY: u8 = 1;
const TITLE: &str = "Not Quite Lisp";

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

fn part1(input: &str) -> i32 {
    day01::get_floor(input)
}

fn part2(input: &str) -> i32 {
    match day01::get_position(input) {
        Ok(position) => position,
        Err(e) => {
            println!("part2: {e}");
            exit(1);
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1() {
        let data = vec![
            ("(())", 0),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            ("))(", -1),
            (")))", -3),
            (")())())", -3),
        ];
        for (input, answer) in data {
            let result = part1(input);
            assert_eq!(result, answer);
        }
    }

    #[test]
    fn check_part2() {
        let data = vec![
            (")", 1),
            ("()())", 5),
        ];
        for (input, answer) in data {
            let result = part2(input);
            assert_eq!(result, answer);
        }
    }
}
