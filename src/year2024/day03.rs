use regex::Regex;

pub fn part1(content: &str) -> Result<u32, String> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total = 0;
    for (_, [a, b]) in re.captures_iter(content).map(|c| c.extract()) {
        let a: u32 = a.parse().unwrap();
        let b: u32 = b.parse().unwrap();
        total += a * b;
    };
    Ok(total)
}

pub fn part2(content: &str) -> Result<u32, String> {
    let re = Regex::new(r"(don't|do|mul)\(((\d{1,3}),(\d{1,3}))?\)").unwrap();
    let mut total = 0;
    let mut disabled = false;
    for items in re.captures_iter(content) {
        let cmd = &items[1];
        match cmd {
            "don't" => {
                disabled = true;
                continue;
            },
            "do" => {
                disabled = false;
                continue;
            },
            "mul" => {
                if disabled {
                    continue;
                }
                let a: u32 = items[3].parse().unwrap();
                let b: u32 = items[4].parse().unwrap();
                total += a * b;
            },
            _ => panic!("unknown command"),
        }
    };
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(input), Ok(161));
    }

    #[test]
    fn test_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(input), Ok(48));
    }
}
