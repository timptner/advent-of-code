// Year 2015 Day 5
// Doesn't He Have Intern-Elves For This?

use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./data/2015/05.txt").expect("can't read file");
    let mut counter1 = 0;
    for line in input.lines() {
        let has_vowels = contains_vowels(line);
        let has_duplicates = contains_duplicate(line);
        let has_blacklist = contains_blacklist(line);
        if has_vowels & has_duplicates & !has_blacklist {
            counter1 += 1;
        }
    }
    println!("Part 1: {counter1}");

    let mut counter2 = 0;
    for line in input.lines() {
        let has_pairs = contains_pair_of_two_letters(line);
        let has_duplicate_one_apart = contains_duplicate_letters_with_one_apart(line);
        if has_pairs & has_duplicate_one_apart {
            counter2 += 1;
        }
    }
    println!("Part 2: {counter2}");
}

fn contains_vowels(line: &str) -> bool {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    let mut counter = 0;
    for letter in line.chars() {
        if vowels.contains(&letter) {
            counter += 1;
        }
    }
    if counter > 2 {
        true
    } else {
        false
    }
}

fn contains_duplicate(line: &str) -> bool {
    let mut letters = line.chars();
    let mut last_letter = letters.next().unwrap();
    for letter in letters {
        if letter.eq(&last_letter) {
            return true;
        }
        last_letter = letter;
    }
    false
}

fn contains_blacklist(line: &str) -> bool {
    let blacklist = vec!["ab", "cd", "pq", "xy"];
    for item in blacklist {
        if line.contains(item) {
            return true
        }
    }
    false
}

fn contains_pair_of_two_letters(line: &str) -> bool {
    let end = line.len() + 1;
    for n in 0..end-4 {
        let pair = &line[n..n+2];
        for m in n+2..end-2 {
            if pair == &line[m..m+2] {
                return true;
            }
        }
    }
    false
}

fn contains_duplicate_letters_with_one_apart(line: &str) -> bool {
    let end = line.len() + 1;
    for n in 0..end-3 {
        if &line[n..n+1] == &line[n+2..n+3] {
            return true;
        }
    }
    false
}
