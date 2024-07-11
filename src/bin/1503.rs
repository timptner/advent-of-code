// Year 2015 Day 3
// Perfectly Spherical Houses in a Vacuum

use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./data/2015/03.txt").expect("failed to read file");

    // Part 1
    let directions: Vec<char> = input.chars().collect();
    let houses = count_houses(directions);
    println!("Visited houses: {}", houses.len());

    // Part 2
    let mut santa = Vec::new();
    let mut robo = Vec::new();

    for (i, c) in input.chars().enumerate() {
        if i % 2 == 0 {
            santa.push(c);
        } else {
            robo.push(c);
        }
    }

    let houses_santa = count_houses(santa);
    let houses_robo = count_houses(robo);
    let houses_total: Vec<_> = houses_santa.union(&houses_robo).collect();
    println!("Visited houses with Robo-Santa: {}", houses_total.len());
}

fn count_houses(directions: Vec<char>) -> HashSet<(i32, i32)> {
    let mut houses = HashSet::new();

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    houses.insert((x, y));

    for direction in directions {
        (x, y) = match direction {
            '>' => (x + 1, y),
            '<' => (x - 1, y),
            '^' => (x, y + 1),
            'v' => (x, y - 1),
            unknown => {
                println!("Ignored value: {}", unknown as u32);
                continue;
            }
        };

        let coord = (x, y);

        if !houses.contains(&coord) {
            houses.insert(coord);
        }
    }

    houses
}
