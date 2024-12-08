use std::collections::HashSet;

pub fn part1(content: &str) -> Result<u32, String> {
    let mut grid = Grid::new(content);
    let (row, column) = grid.position('^').unwrap();
    grid.set(row, column, '.').unwrap();
    let mut guard = Guard::new(row, column, Orientation::North);
    let mut visited_positions = HashSet::new();
    visited_positions.insert((row, column));
    loop {
        let (row, column) = guard.next_field();
        let next_value = match grid.get(row, column) {
            Some(value) => value,
            None => break,
        };
        match next_value {
            '.' => guard.move_forward(),
            '#' => guard.turn_right(),
            _ => panic!("unknown value"),
        }
        visited_positions.insert((guard.row, guard.column));
    }
    Ok(visited_positions.len() as u32)
}

// 1795 - to high
// 1681 - to high
pub fn part2(content: &str) -> Result<u32, String> {
    let mut grid = Grid::new(content);
    let (row, column) = grid.position('^').unwrap();
    grid.set(row, column, '.').unwrap();
    let orientation = Orientation::North;
    let mut guard = Guard::new(row, column, orientation);
    let mut visited_positions = HashSet::new();
    visited_positions.insert((guard.row, guard.column, guard.orientation));
    let mut blocking_positions = HashSet::new();
    loop {
        let (row, column) = guard.next_field();
        let next_value = match grid.get(row, column) {
            Some(value) => value,
            None => break,
        };
        if next_value == '.'
            // Note: already visited route can't be used as an position for
            // obsticles because guard would've colisioned earlier
            // Idea: mark traveled route with custom char instead
            && !(visited_positions.contains(&(row, column, Orientation::North))
                || visited_positions.contains(&(row, column, Orientation::East))
                || visited_positions.contains(&(row, column, Orientation::South))
                || visited_positions.contains(&(row, column, Orientation::West)))
            && projection_is_cycle(&grid, &guard, &visited_positions)
        {
            blocking_positions.insert((row, column));
        }
        match next_value {
            '.' => guard.move_forward(),
            '#' => guard.turn_right(),
            _ => panic!("unknown value"),
        }
        visited_positions.insert((guard.row, guard.column, guard.orientation));
    }
    blocking_positions.remove(&(row, column));
    Ok(blocking_positions.len() as u32)
}

fn projection_is_cycle(
    grid: &Grid,
    guard: &Guard,
    visited_positions: &HashSet<(i32, i32, Orientation)>,
) -> bool {
    let mut grid = grid.clone();
    let mut guard = guard.clone();
    let (row, column) = guard.next_field();
    grid.set(row, column, '#').unwrap();

    let mut visited_positions = visited_positions.clone();
    loop {
        let (row, column) = guard.next_field();
        let next_value = match grid.get(row, column) {
            Some(value) => value,
            None => return false,
        };
        match next_value {
            '.' => guard.move_forward(),
            '#' => guard.turn_right(),
            _ => panic!("unkown value"),
        }
        let is_new = visited_positions.insert((guard.row, guard.column, guard.orientation));
        if !is_new {
            return true;
        }
    }
}

#[derive(Clone, Debug)]
struct Grid {
    data: Vec<char>,
    rows: i32,
    columns: i32,
}

impl Grid {
    fn new(content: &str) -> Self {
        let data: Vec<_> = content
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        let rows = data.len() as i32;
        let columns = data[0].len() as i32;
        let data: Vec<char> = data.into_iter().flatten().collect();
        Self {
            data,
            rows,
            columns,
        }
    }

    fn get_index(&self, row: i32, column: i32) -> Result<usize, &'static str> {
        if !(0..self.rows).contains(&row) {
            return Err("row is off grid");
        }
        if !(0..self.columns).contains(&column) {
            return Err("column is off grid");
        }
        let index = (self.columns * row + column) as usize;
        if !(0..self.data.len()).contains(&index) {
            return Err("data does not contain calculated index");
        }
        Ok(index)
    }

    fn get(&self, row: i32, column: i32) -> Option<char> {
        let index = match self.get_index(row, column) {
            Ok(index) => index,
            Err(_) => return None,
        };
        Some(self.data[index])
    }

    fn set(&mut self, row: i32, column: i32, value: char) -> Result<(), &str> {
        let index = match self.get_index(row, column) {
            Ok(index) => index,
            Err(_) => return Err("can't set value off grid"),
        };
        self.data[index] = value;
        Ok(())
    }

    fn position(&self, value: char) -> Option<(i32, i32)> {
        let (row, column) = match self.data.iter().position(|&x| x == value) {
            Some(index) => {
                let row = index as i32 / self.columns;
                let column = index as i32 % self.columns;
                (row, column)
            }
            None => return None,
        };
        Some((row, column))
    }
}

#[derive(Copy, Hash, Eq, PartialEq, Debug)]
enum Orientation {
    North,
    East,
    South,
    West,
}

impl Clone for Orientation {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Clone, Debug)]
struct Guard {
    row: i32,
    column: i32,
    orientation: Orientation,
}

impl Guard {
    fn new(row: i32, column: i32, orientation: Orientation) -> Self {
        Self {
            row,
            column,
            orientation,
        }
    }

    fn next_field(&self) -> (i32, i32) {
        match self.orientation {
            Orientation::North => (self.row - 1, self.column),
            Orientation::East => (self.row, self.column + 1),
            Orientation::South => (self.row + 1, self.column),
            Orientation::West => (self.row, self.column - 1),
        }
    }

    fn move_forward(&mut self) {
        let (row, column) = self.next_field();
        self.row = row;
        self.column = column;
    }

    fn turn_right(&mut self) {
        let orientation = match self.orientation {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
        };
        self.orientation = orientation;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), Ok(41));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), Ok(6));
    }
}
