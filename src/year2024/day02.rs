pub fn part1(content: &str) -> Result<u32, String> {
    let mut safe_reports = 0;
    for line in content.lines() {
        let items: Vec<_> = line.split(' ').collect();
        let mut numbers = Vec::new();
        for item in items {
            let number: i32 = item.parse().unwrap();
            numbers.push(number);
        }
        let distances = calculate_distances(&numbers);
        if validate_distances(&distances) {
            safe_reports += 1;
        }
    }
    Ok(safe_reports)
}

pub fn part2(content: &str) -> Result<u32, String> {
    let mut safe_reports = 0;
    for line in content.lines() {
        let items: Vec<_> = line.split(' ').collect();
        let mut numbers = Vec::new();
        for item in items {
            let number: i32 = item.parse().unwrap();
            numbers.push(number);
        }
        let distances = calculate_distances(dbg!(&numbers));
        if validate_distances(&distances) {
            dbg!("SAFE");
            safe_reports += 1;
            continue;
        }
        for index in 0..numbers.len() {
            let mut levels_moded = Vec::new();
            for (position, item) in numbers.iter().enumerate() {
                if position == index {
                    continue;
                }
                levels_moded.push(*item);
            }
            let distances_moded = calculate_distances(&levels_moded);
            if validate_distances(&distances_moded) {
                dbg!(format!("SAFE without level {}", index + 1));
                safe_reports += 1;
                break;
            }
        }
    }
    Ok(safe_reports)
}

fn calculate_distances(numbers: &Vec<i32>) -> Vec<i32> {
    let mut distances = Vec::new();
    for index in 1..numbers.len() {
        let last_number = numbers[index - 1];
        let number = numbers[index];
        let distance = last_number - number;
        distances.push(distance);
    }
    distances
}

fn validate_distances(numbers: &Vec<i32>) -> bool {
    let count_increasing = numbers.iter().filter(|&n| *n > 0).count();
    let count_decreasing = numbers.iter().filter(|&n| *n < 0).count();
    if count_increasing > 0 && count_decreasing > 0 {
        return false;
    }
    let range = 1..=3;
    let count_adjacent = numbers.iter().filter(|&n| range.contains(&n.abs())).count();
    if count_adjacent < numbers.len() {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), Ok(2));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), Ok(4));
    }
}
