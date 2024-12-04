use grid::Grid;

//  O   D   X   Y
//  0   1   1   0
//  0   2   2   0
//  0   3   3   0
//  1   1   1   1
//  1   2   2   2
//  1   3   3   3
//  2   1   0   1
//  2   2   0   2
//  2   3   0   3
//  3   1  -1   1
//  3   2  -2   2
//  3   3  -3   3
//  4   1  -1   0
//  4   2  -2   0
//  4   3  -3   0
//  5   1  -1  -1
//  5   2  -2  -2
//  5   3  -3  -3
//  6   1   0  -1
//  6   2   0  -2
//  6   3   0  -3
//  7   1   1  -1
//  7   2   2  -2
//  7   3   3  -3

pub fn part1(content: &str) -> Result<u32, String> {
    let lines: Vec<&str> = content.lines().collect();
    let columns = lines[0].len();
    let data: Vec<char> = content.replace('\n', "").chars().collect();
    let grid = Grid::from_vec(data, columns);

    let letters = ['X', 'M', 'A', 'S'];
    let mut total = 0;
    for ((x, y), value) in grid.indexed_iter() {
        if value != &letters[0] {
            continue;
        }
        'orientation: for orientation in 0..8 {
            for distance in 1..4 {
                // get next position to check
                let (x1, y1) = match get_position(x, y, orientation, distance) {
                    Some(pos) => pos,
                    None => continue 'orientation,
                };

                // get value of next position
                let letter = match grid.get(x1, y1) {
                    Some(c) => c,
                    None => continue 'orientation,
                };

                // check value
                if letter != &letters[distance as usize] {
                    continue 'orientation;
                }
            }
            total += 1;
        }
    }
    Ok(total)
}


pub fn part2(content: &str) -> Result<u32, String> {
    let lines: Vec<&str> = content.lines().collect();
    let columns = lines[0].len();
    let data: Vec<char> = content.replace('\n', "").chars().collect();
    let grid = Grid::from_vec(data, columns);

    let words = [
        "SMMS",
        "SSMM",
        "MMSS",
        "MSSM",
    ];

    let mut total = 0;
    'outer: for ((x, y), value) in grid.indexed_iter() {
        if value != &'A' {
            continue;
        }

        let mut letters: Vec<char> = Vec::new();
        for n in 0..4 {
            let (x1, y1) = match n {
                0 => (1, 1),
                1 => (-1, 1),
                2 => (-1, -1),
                3 => (1, -1),
                _ => panic!(),
            };
            match grid.get(x as i32 + x1, y as i32 + y1) {
                Some(c) => letters.push(*c),
                None => continue 'outer,
            }
        }
        let word: String = letters.into_iter().collect();
        if words.contains(&word.as_str()) {
            total += 1;
        }
    }
    Ok(total)
}

fn get_position(x: usize, y: usize, orientation: i32, distance: i32) -> Option<(usize, usize)> {
    let (x1, y1) = match orientation {
        0 => (1, 0),
        1 => (1, 1),
        2 => (0, 1),
        3 => (-1, 1),
        4 => (-1, 0),
        5 => (-1, -1),
        6 => (0, -1),
        7 => (1, -1),
        _ => panic!("unexpected distance")
    };

    let x2 = x1 * distance;
    let y2 = y1 * distance;
    
    let x3 = x as i32 + x2;
    let y3 = y as i32 + y2;

    if x3 < 0 || y3 < 0 {
        return None;
    }

    Some((x3 as usize, y3 as usize))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), Ok(18));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), Ok(9));
    }
}
