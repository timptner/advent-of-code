pub fn part1(content: &str) -> Result<u32, String> {
    let (set1, set2) = get_sets(content);
    let mut total = 0;
    for index in 0..set1.len() {
        let a = set1[index];
        let b = set2[index];
        println!("{a} {b}");
        let distance: u32;
        if b > a {
            distance = b - a;
        } else {
            distance = a - b;
        }
        total += distance;
    }
    Ok(total)
}

pub fn part2(content: &str) -> Result<u32, String> {
    let mut total = 0;
    let (set1, set2) = get_sets(content);
    for index in 0..set1.len() {
        let a = set1[index];
        let amount = set2.iter().filter(|&n| *n == a).count();
        total += a * amount as u32;
    }
    Ok(total)
}

fn get_sets(content: &str) -> (Vec<u32>, Vec<u32>) {
    let mut set1 = Vec::new();
    let mut set2 = Vec::new();
    for line in content.lines() {
        let items: Vec<&str> = line.split(' ').collect();

        let a: u32 = items[0].parse().unwrap();
        let b: u32 = items[3].parse().unwrap();

        set1.push(a);
        set2.push(b);
    }
    set1.sort();
    set2.sort();
    (set1, set2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), Ok(11));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), Ok(31));
    }
}
