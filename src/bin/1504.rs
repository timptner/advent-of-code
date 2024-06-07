// Year 2015 Day 4
// The Ideal Stocking Stuffer

use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./data/2015/04.txt").expect("can't read file");
    let secret = input.lines().next().expect("no more lines");

    // Part 1
    let mut number: u32 = 1;
    loop {
        let value = format!("{}{}", secret, number);
        let hash = md5::compute(value);
        let hex = format!("{:x}", hash);
        if hex.starts_with("00000") {
            break;
        }

        number += 1;
    }

    println!("Part1: {number}");

    // Part 2
    let mut number: u32 = 1;
    loop {
        let value = format!("{}{}", secret, number);
        let hash = md5::compute(value);
        let hex = format!("{:x}", hash);
        if hex.starts_with("000000") {
            break;
        }

        number += 1;
    }

    println!("Part2: {number}");
}
