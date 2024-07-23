// Year 2015 Day 7
// Some Assembly Required

use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let input = read_to_string("./data/2015/07.txt").expect("failed to read file");
    
    let mut map: HashMap<String, String> = HashMap::new();
    for line in input.lines() {
        let (source, target) = parse_line(line);
        map.insert(target, source);
    }

    let mut results: HashMap<String, u16> = HashMap::new();
    let answer = get_origin(&mut results, &map, "a");
    
    println!("Part 1: {}", answer);

    let mut results: HashMap<String, u16> = HashMap::new();
    results.insert("b".to_owned(), answer);
    let answer = get_origin(&mut results, &map, "a");

    println!("Part 2: {}", answer);
}

fn parse_line(line: &str) -> (String, String) {
    let items: Vec<&str> = line.split(" -> ").collect();
    let source = items[0].to_owned();
    let target = items[1].to_owned();
    (source, target)
}

fn get_origin(results: &mut HashMap<String, u16>, map: &HashMap<String, String>, key: &str) -> u16 {
    if results.contains_key(key) {
        let number = results.get(key).unwrap().to_owned();
        return number;
    };

    let value = map.get(key).unwrap();
    let items: Vec<&str> = value.split(' ').collect();

    let number: u16;
    
    match items.len() {
        1 => {
            let a = items[0];

            if is_numeric(a) {
                number = a.parse().unwrap();
            } else {
                number = get_origin(results, map, a);
            }
        },
        2 => {
            let a = items[0];
            let b = items[1];

            assert!(a.eq("NOT"));

            number = !get_origin(results, map, b);
        },
        3 => {
            let a = items[0];
            let b = items[1];
            let c = items[2];

            match b {
                "AND" => {
                    if is_numeric(a) {
                        let a: u16 = a.parse().unwrap();

                        number = a & get_origin(results, map, c);
                    } else {
                        number = get_origin(results, map, a) & get_origin(results, map, c);
                    }
                },
                "OR" => {
                    if is_numeric(a) {
                        let a: u16 = a.parse().unwrap();

                        number = a | get_origin(results, map, c);
                    } else {
                        number = get_origin(results, map, a) | get_origin(results, map, c);
                    }
                },
                "LSHIFT" => {
                    let c: u16 = c.parse().unwrap();

                    number = get_origin(results, map, a) << c;
                },
                "RSHIFT" => {
                    let c: u16 = c.parse().unwrap();

                    number = get_origin(results, map, a) >> c;
                },
                _ => panic!("unexpected operation: {b}"),
            };
        },
        _ => panic!("unexpected amount of items"),
    };

    results.insert(key.to_owned(), number);

    // println!("{} {}", key, number);

    number
}

fn is_numeric(value: &str) -> bool {
    match value.parse::<u16>() {
        Ok(_) => true,
        Err(_) => false,
    }
}
