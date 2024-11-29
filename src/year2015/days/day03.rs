use std::collections::HashSet;

pub fn count_houses(directions: Vec<char>) -> u16 {
    let mut houses = HashSet::new();

    let mut position = (0, 0);

    houses.insert(position);

    for direction in directions {
        position = get_next_position(position, &direction);
        houses.insert(position);
    }

    houses.len() as u16
}

fn get_next_position(position: (i32, i32), direction: &char) -> (i32, i32) {
    let (x, y) = position;
    match direction {
        '>' => (x + 1, y),
        '<' => (x - 1, y),
        '^' => (x, y + 1),
        'v' => (x, y - 1),
        _ => unimplemented!(),
    }
}

pub fn count_combined_houses(directions: Vec<char>) -> u16 {
    let mut houses = HashSet::new();
    houses.insert((0, 0));

    let mut position_santa = (0, 0);
    let mut position_robo = (0, 0);

    for (index, direction) in directions.into_iter().enumerate() {
        if index % 2 == 0 {
            position_santa = get_next_position(position_santa, &direction);
            houses.insert(position_santa);
        } else {
            position_robo = get_next_position(position_robo, &direction);
            houses.insert(position_robo);
        }
    }

    houses.len() as u16
}
