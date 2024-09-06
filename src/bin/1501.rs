// Year 2015 Day 1
// Not Quite Lisp

use std::fs::read_to_string;

fn main() {
    let content = read_puzzle_input();
    
    let floor = part1(&content);
    println!("Floor: {}", floor);

    let instruction = part2(&content);
    println!("Basement: {}", instruction);
}

fn read_puzzle_input() -> String {
    let input = read_to_string("./data/2015/01.txt").expect("failed to read file");
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
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
        for (input, result) in data {
            assert_eq!(part1(input), result);
        }
    }

    #[test]
    fn test_part2() {
        let data = vec![
            (")", 1),
            ("()())", 5),
        ];
        for (input, result) in data {
            assert_eq!(part2(input), result);
        }
    }
}

fn get_direction(char: char) -> i8 {
    match char {
        '(' => 1,
        ')' => -1,
        '\r' => 0,
        '\n' => 0,
        ' ' => 0,
        _ => panic!("unknown character: {}", char),
    }
}

fn part1(content: &str) -> i32 {
    let mut floor: i32 = 0;
    for direction in content.chars() {
        floor += get_direction(direction) as i32;
    }
    floor
}

fn part2(content: &str) -> i32 {
    let mut floor: i32 = 0;
    let mut instruction: i32 = 0;
    for direction in content.chars() {
        floor += get_direction(direction) as i32;
        instruction += 1;
        if floor == -1 {
            break;
        }
    }
    instruction
}
