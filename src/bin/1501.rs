// Year 2015 Day 1
// Not Quite Lisp

use std::fs::read_to_string;

fn main() {
    let content = read_puzzle_input();
    let floor = part1(content.clone());
    println!("Floor: {}", floor);
    let instruction = part2(content);
    println!("Basement: {}", instruction);
}

fn read_puzzle_input() -> String {
    let input = read_to_string("./data/2015/01.txt").expect("failed to read file");
    input
}

fn part1(content: String) -> i32 {
    let mut floor: i32 = 0;
    for direction in content.chars() {
        floor += match direction as u32 {
            40 => 1,  // opening parenthesis
            41 => -1, // closing parenthesis
            13 => 0,  // cariage return
            10 => 0,  // linefeed
            32 => 0,  // whitespace
            _ => panic!("unknown character: {}", direction as u32),
        };
    }
    floor
}

fn part2(content: String) -> i32 {
    let mut floor: i32 = 0;
    let mut instruction: i32 = 1;
    for direction in content.chars() {
        floor += match direction as u32 {
            40 => 1,  // opening parenthesis
            41 => -1, // closing parenthesis
            13 => 0,  // cariage return
            10 => 0,  // linefeed
            32 => 0,  // whitespace
            _ => panic!("unknown character: {}", direction as u32),
        };
        if floor == -1 {
            break;
        }
        instruction += 1;
    }
    instruction
}
