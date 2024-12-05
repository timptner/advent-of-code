use std::cmp::Ordering;

pub fn part1(content: &str) -> Result<u32, String> {
    let parts: Vec<&str> = content.split("\n\n").collect();
    
    // parse rules from input
    let mut rules = Vec::new();
    for line in parts[0].lines() {
        let rule: [u32; 2] = line.split('|').map(
            |n| n.parse::<u32>().unwrap()
        ).collect::<Vec<u32>>().try_into().unwrap();
        rules.push(rule);
    }

    // parse updates from input
    let mut updates = Vec::new();
    for line in parts[1].lines() {
        let update: Vec<u32> = line.split(',').map(
            |n| n.parse::<u32>().unwrap()
        ).collect();
        updates.push(update);
    }

    // check updates
    let mut total = 0;
    'update: for update in updates {
        // filter relevant rules by pages
        let active_rules: Vec<&[u32; 2]> = rules.iter().filter(
            |&&r| update.contains(&r[0]) && update.contains(&r[1])
        ).collect();
        
        // validate each rule
        'rule: for rule in active_rules {
            let pages: Vec<u32> = update.iter().filter(|&p| rule.contains(p)).map(|p| *p).collect();
            if pages == rule {
                continue 'rule;
            } else {
                continue 'update;
            }
        }
        let index = (update.len() - 1) / 2;
        total += update[index];
    }

    Ok(total)
}

pub fn part2(content: &str) -> Result<u32, String> {
    let parts: Vec<&str> = content.split("\n\n").collect();
    
    // parse rules from input
    let mut rules = Vec::new();
    for line in parts[0].lines() {
        let rule: [u32; 2] = line.split('|').map(
            |n| n.parse::<u32>().unwrap()
        ).collect::<Vec<u32>>().try_into().unwrap();
        rules.push(rule);
    }

    // parse updates from input
    let mut updates = Vec::new();
    for line in parts[1].lines() {
        let update: Vec<u32> = line.split(',').map(
            |n| n.parse::<u32>().unwrap()
        ).collect();
        updates.push(update);
    }

    // get invalid updates
    let mut invalid_updates = Vec::new();
    for mut update in updates {
        // filter relevant rules by pages
        let active_rules: Vec<&[u32; 2]> = rules.iter().filter(
            |&&r| update.contains(&r[0]) && update.contains(&r[1])
        ).collect();
        
        // validate each rule
        let mut has_invalid_rules = false;
        'rule: for rule in active_rules {
            let pages: Vec<u32> = update.iter().filter(|&p| rule.contains(p)).map(|p| *p).collect();
            if pages == rule {
                continue 'rule;
            }
            has_invalid_rules = true;
        }
        if has_invalid_rules {
            // re-order invalid updates
            update.sort_by(|&a, &b| page_sorter(a, b, &rules));
            // collect invalid updates
            invalid_updates.push(dbg!(update));
        }
    }

    // calculate sum of middle number in invalid updates
    let mut total = 0;
    for update in invalid_updates {
        let index = (update.len() - 1) / 2;
        total += update[index];
    }
    Ok(total)
}


fn page_sorter(a: u32, b: u32, rules: &Vec<[u32; 2]>) -> Ordering {
    for rule in rules {
        if rule.contains(&a) && rule.contains(&b) {
            if rule == &[a, b] {
                return Ordering::Equal;
            } else {
                // TODO sorting by greater will reverse update except first item...
                return Ordering::Less;
            }
        }
    }
    panic!("no rule matches");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), Ok(143));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), Ok(123));
    }
}
