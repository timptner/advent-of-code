use std::collections::HashMap;

pub fn part1(content: &str) -> Result<u32, String> {
    let mut total: u32 = 0;
    for line in content.lines() {
        total += find_number(line)?;
    }
    Ok(total)
}

pub fn part2(content: &str) -> Result<u32, String> {
    let mut total: u32 = 0;
    for line in content.lines() {
        total += find_number_extended(line)?;
    }
    Ok(total)
}

const DIGITS: &'static [char] = &['1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn get_first_digit(line: &str) -> Option<u32> {
    for value in line.chars() {
        if DIGITS.contains(&value) {
            let digit = value.to_digit(10).unwrap();
            return Some(digit);
        };
    }
    None
}

fn get_last_digit(line: &str) -> Option<u32> {
    let line: String = line.chars().rev().collect();
    get_first_digit(line.as_str())
}

fn find_number(line: &str) -> Result<u32, String> {
    let first_digit = match get_first_digit(line) {
        Some(digit) => digit,
        None => {
            return Err(String::from("no first digit found"));
        },
    };
    
    let second_digit = match get_last_digit(line) {
        Some(digit) => digit,
        None => {
            return Err(String::from("no last digit found"));
        }
    };

    Ok(first_digit * 10 + second_digit)
}

const DIGITS_WRITTEN: &str = "one
two
three
four
five
six
seven
eight
nine";

fn build_index_map(line: &str) -> HashMap<usize, u32> {
    let mut indexes: HashMap<usize, u32> = HashMap::new();

    for value in DIGITS_WRITTEN.lines() {
        for (index, _) in line.match_indices(value) {
            let digit = match value {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => panic!("not a digit"),
            };
            indexes.insert(index, digit);
        }
    }
    
    for value in DIGITS.iter().map(|c| c.to_string()) {
        for (index, _) in line.match_indices(value.as_str()) {
            let digit: u32 = value.parse().unwrap();
            indexes.insert(index, digit);
        }
    }
    
    indexes
}

fn get_first_digit_extended(line: &str) -> Option<u32> {
    let indexes = build_index_map(line);

    for index in 0..line.len() {
        match indexes.get(&index) {
            Some(value) => {
                return Some(*value);
            },
            None => continue,
        }
    }

    None
}

fn get_last_digit_extended(line: &str) -> Option<u32> {
    let indexes = build_index_map(line);

    for index in 0..line.len() {
        let rev_index = line.len() - index;
        match indexes.get(&(rev_index - 1)) {
            Some(value) => {
                return Some(*value);
            },
            None => continue,
        }
    }

    None
}
fn find_number_extended(line: &str) -> Result<u32, String> {
    let first_digit = match get_first_digit_extended(line) {
        Some(digit) => digit,
        None => {
            return Err(String::from("no first digit found"));
        }
    };

    let last_digit = match get_last_digit_extended(line) {
        Some(digit) => digit,
        None => {
            return Err(String::from("no last digit found"));
        }
    };

    Ok(first_digit * 10 + last_digit)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_PART1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_find_number() {
        let answers = vec![12, 38, 15, 77];
        for (index, line) in INPUT_PART1.lines().enumerate() {
            let answer = answers[index];
            assert_eq!(find_number(line), Ok(answer));
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT_PART1), Ok(142));
    }

    const INPUT_PART2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_find_number_extended() {
        let answers = vec![29, 83, 13, 24, 42, 14, 76];
        for (index, line) in INPUT_PART2.lines().enumerate() {
            let answer = answers[index];
            assert_eq!(find_number_extended(line), Ok(answer));
        }
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT_PART2), Ok(281));
    }
}
