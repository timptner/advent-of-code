// Year 2015 Day 6
// Probably a Fire Hazard

use std::fs::{read_to_string, File};
use std::io::Write;

const SIZE: usize = 1_000;

fn main() {
    let input = read_to_string("./data/2015/06.txt").expect("input file should be readable");
    let mut matrix = Matrix::new();
    for (index, line) in input.lines().enumerate() {
        if line.starts_with("turn on") {
            let rectangle = parse_line(line, "turn on ");
            matrix.switch(rectangle, "turn_on");
        } else if line.starts_with("turn off") {
            let rectangle = parse_line(line, "turn off ");
            matrix.switch(rectangle, "turn_off");
        } else if line.starts_with("toggle") {
            let rectangle = parse_line(line, "toggle ");
            matrix.switch(rectangle, "toggle");
        } else {
            panic!("unexpected line content: {}", line);
        }
        // if index % 25 == 0 {
        //     matrix.plot(&format!("step{:03}.txt", index + 1));
        // }
    }
    println!("{}", matrix.count());
}

fn parse_line(line: &str, prefix: &str) -> Rectangle {
    let items: Vec<&str> = line.strip_prefix(prefix).unwrap().split(' ').collect();
    assert!(items.len() == 3, "malformed line: {:?}", line);
    Rectangle::new(parse_point(items[0]), parse_point(items[2]))
}

fn parse_point(values: &str) -> Point {
    let items: Vec<usize> = values.split(',').map(|s| s.parse().unwrap()).collect();
    assert!(items.len() == 2, "malformed point: {:?}", values);
    Point {
        x: items[0],
        y: items[1],
    }
}

struct Point {
    x: usize,
    y: usize,
}

struct Rectangle {
    start: Point,
    end: Point,
}

impl Rectangle {
    fn new(start: Point, end: Point) -> Rectangle {
        assert!(start.x <= end.x, "x is invalid");
        assert!(start.y <= end.y, "y is invalid");
        Rectangle { start, end }
    }
}

struct Matrix {
    data: Vec<Vec<bool>>,
}

impl Matrix {
    fn new() -> Matrix {
        Matrix {
            data: vec![vec![false; SIZE]; SIZE],
        }
    }

    fn get(&self, point: &Point) -> bool {
        self.data[point.x][point.y]
    }

    fn set(&mut self, point: &Point, value: bool) {
        self.data[point.x][point.y] = value;
    }

    fn turn_on(&mut self, point: &Point) {
        self.set(point, true);
    }

    fn turn_off(&mut self, point: &Point) {
        self.set(point, false);
    }

    fn toggle(&mut self, point: &Point) {
        self.set(point, !self.get(point));
    }

    fn switch(&mut self, rectangle: Rectangle, action: &str) {
        for y in rectangle.start.y..=rectangle.end.y {
            for x in rectangle.start.x..=rectangle.end.x {
                let point = Point { x, y };
                match action {
                    "turn_on" => self.turn_on(&point),
                    "turn_off" => self.turn_off(&point),
                    "toggle" => self.toggle(&point),
                    _ => panic!("unknown action: {:?}", action),
                }
            }
        }
    }

    fn count(&self) -> u32 {
        let mut counter: u32 = 0;
        for x in 0..SIZE {
            for y in 0..SIZE {
                let point = Point { x, y };
                match self.get(&point) {
                    true => counter += 1,
                    false => {}
                }
            }
        }
        counter
    }

    fn plot(&self, name: &str) {
        let path = String::from("./output/") + name;
        let mut file = File::create(path).unwrap();
        for x in 0..SIZE {
            for y in 0..SIZE {
                let point = Point { x, y };
                let value = match self.get(&point) {
                    true => '*',
                    false => ' ',
                };
                write!(&mut file, "{value}").unwrap();
            }
            writeln!(&mut file, "").unwrap();
        }
    }
}
