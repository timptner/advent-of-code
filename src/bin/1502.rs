// Year 2015 Day 2
// I Was Told There Would Be No Math

use std::{fs::read_to_string, vec};

fn main() {
    let input = read_to_string("./data/2015/02.txt").expect("failed to read file");
    let mut total_area = 0;
    let mut total_ribbon = 0;

    for line in input.lines() {
        let dimensions: Vec<&str> = line.split("x").collect();
        let length: i32 = dimensions[0].parse().expect("failed to parse as i32");
        let width: i32 = dimensions[1].parse().expect("failed to parse as i32");
        let height: i32 = dimensions[2].parse().expect("failed to parse as i32");

        total_area += calc_area(length, width, height);
        total_ribbon += calc_ribbon(length, width, height);
    }

    println!("Paper: {}", total_area);
    println!("Ribbon: {}", total_ribbon);
}

fn calc_area(length: i32, width: i32, height: i32) -> i32 {
    let x = width * height;
    let y = length * height;
    let z = length * width;

    let slack = vec![x, y, z]
        .iter()
        .min()
        .expect("empty iterator")
        .to_owned();
    let area = 2 * (x + y + z);

    area + slack
}

fn calc_ribbon(length: i32, width: i32, height: i32) -> i32 {
    let bow = length * width * height;

    let mut dimensions = vec![length, width, height];
    dimensions.sort();
    let wrap = &dimensions[..2].iter().sum() * 2;

    wrap + bow
}
