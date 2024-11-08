use std::char::from_digit;

pub fn part1(content: &str) -> Result<u16, String> {
    let mut total: u16 = 0;
    for line in content.lines() {
        let first_digit = get_first_digit(line)?;
        let last_digit = get_last_digit(line)?;
        let number: u16 = format!("{first_digit}{last_digit}").parse().unwrap();
        total += number;
    }
    Ok(total)
}

fn get_first_digit(value: &str) -> Result<char, String> {
    let digits: Vec<char> = (0..10).map(|d| from_digit(d, 10).unwrap()).collect();
    for char in value.chars() {
        if digits.contains(&char) {
            return Ok(char);
        };
    }
    Err(String::from("no digit found"))
}

fn get_last_digit(value: &str) -> Result<char, String> {
    let value: String = value.chars().rev().collect();
    let digit = get_first_digit(value.as_str())?;
    Ok(digit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_digit() {
        assert_eq!(get_first_digit("1abc2"), Ok('1'));
    }

    #[test]
    fn test_get_last_digit() {
        assert_eq!(get_last_digit("1abc2"), Ok('2'));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"), Ok(142));
    }
}
