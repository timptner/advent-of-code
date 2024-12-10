pub fn part1(content: &str) -> Result<u64, &str> {
    let mut total = 0;
    for line in content.lines() {
        let (test_value, numbers) = line.split_once(':').unwrap();
        let test_value: u64 = test_value.parse().unwrap();
        let numbers: Vec<u32> = numbers.split_whitespace().map(|n| n.parse().unwrap()).collect();
        
        let dfs = DFS::new(numbers, vec![Operators::Add, Operators::Multiply]);
        let results: Vec<u64> = dfs.into_iter().filter(|&n| n > 0).collect();

        let valid_results: Vec<u64> = results.iter().map(|&n| n).filter(|&n| n == test_value).collect();
        if valid_results.len() > 0 {
            total += test_value;
        }
    }
    Ok(total)
}

pub fn part2(content: &str) -> Result<u64, &str> {
    let mut total = 0;
    for line in content.lines() {
        let (test_value, numbers) = line.split_once(':').unwrap();
        let test_value: u64 = test_value.parse().unwrap();
        let numbers: Vec<u32> = numbers.split_whitespace().map(|n| n.parse().unwrap()).collect();
        
        let dfs = DFS::new(numbers, vec![Operators::Add, Operators::Multiply, Operators::Concat]);
        let results: Vec<u64> = dfs.into_iter().filter(|&n| n > 0).collect();

        let valid_results: Vec<u64> = results.iter().map(|&n| n).filter(|&n| n == test_value).collect();
        if valid_results.len() > 0 {
            total += test_value;
        }
    }
    Ok(total)
}

#[derive(Clone)]
struct State {
    index: usize,
    value: u64,
}

impl State {
    fn new(index: usize, value: u64) -> Self {
        Self {
            index,
            value,
        }
    }
}

enum Operators {
    Add,
    Multiply,
    Concat,
}

struct DFS {
    numbers: Vec<u32>,
    stack: Vec<(State, u32)>,
    max_depth: u32,
    operators: Vec<Operators>,
}

impl DFS {
    fn new(numbers: Vec<u32>, operators: Vec<Operators>) -> Self {
        let value = numbers[0] as u64;
        let state = State::new(0, value);
        let max_depth = numbers.len() as u32 - 1;
        Self {
            numbers,
            stack: vec![(state, 0)],
            max_depth,
            operators,
        }
    }

    fn next_states_from(&self, numbers: &Vec<u32>, state: &State) -> Vec<State> {
        let index = state.index + 1;
        let next_value = numbers[index] as u64;
        let current_value = state.value;

        let mut states = Vec::new();
        for operator in &self.operators {
            let value = match *operator {
                Operators::Add => current_value + next_value,
                Operators::Multiply => current_value * next_value,
                Operators::Concat => {
                    format!("{current_value}{next_value}").parse::<u64>().unwrap()
                },
            };
            let state = State::new(index, value);
            states.push(state);
        }
        
        states
    }
}

impl Iterator for DFS {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let (state, depth) = self.stack.pop()?;
        
        if depth < self.max_depth {
            for next_state in self.next_states_from(&self.numbers, &state) {
                self.stack.push((next_state, depth + 1));
            }
        }
        if depth == self.max_depth {
            return Some(state.value);
        } else {
            return Some(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), Ok(3749));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), Ok(11387));
    }
}
