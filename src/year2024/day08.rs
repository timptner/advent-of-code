use crate::grid::{Grid, Position};
use itertools::Itertools;
use std::collections::HashSet;

const ANTENNAS: &str = "\
abcdefghijklmnopqrstuvwxyz\
ABCDEFGHIJKLMNOPQRSTUVWXYZ\
0123456789";

pub fn part1(content: &str) -> u32 {
    let grid = Grid::new(content);
    let mut positions: HashSet<Position> = HashSet::new();
    for antenna in ANTENNAS.chars() {
        let antennas = grid.positions(antenna);
        for combination in antennas.iter().powerset().filter(|c| c.len() == 2) {
            let antenna1 = combination[0];
            let antenna2 = combination[1];
            let antinodes = calculate_antinodes(antenna1, antenna2);
            for position in antinodes {
                if !dbg!(grid.contains(dbg!(&position))) {
                    continue;
                }
                positions.insert(position);
            }
        }
    }
    positions.len() as u32
}

pub fn part2(content: &str) -> u32 {
    let total = 0;
    total
}

fn calculate_antinodes(antenna1: &Position, antenna2: &Position) -> [Position; 2] {
    let distance_row = antenna2.row - antenna1.row;
    let distance_col = antenna2.column - antenna1.column;

    let antinode1 = Position::new(antenna1.row - distance_row, antenna1.column - distance_col);
    let antinode2 = Position::new(antenna2.row + distance_row, antenna2.column + distance_col);

    [antinode1, antinode2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 34);
    }
}
