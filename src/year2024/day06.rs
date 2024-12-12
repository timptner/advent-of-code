use std::collections::HashSet;
use crate::grid::Grid;

pub fn part1(content: &str) -> Result<u32, String> {
    let mut grid = Grid::new(content);
    let position = grid.position('^').unwrap();
    grid.set(position.row, position.column, '.').unwrap();

    let mut guard = Guard::new(position.row, position.column, Orientation::North);
    let mut visited_positions = HashSet::new();
    visited_positions.insert((position.row, position.column));
    
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
    let position = grid.position('^').unwrap();
    grid.set(position.row, position.column, '.').unwrap();

    let orientation = Orientation::North;
    let mut guard = Guard::new(position.row, position.column, orientation);

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
    blocking_positions.remove(&(position.row, position.column));
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
