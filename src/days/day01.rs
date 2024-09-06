pub fn get_floor(content: &str) -> i32 {
    let mut floor: i32 = 0;
    for direction in content.chars() {
        floor += get_direction(direction) as i32;
    }
    floor
}

pub fn get_position(content: &str) -> Result<i32, &str> {
    let mut floor: i32 = 0;
    let mut position: i32 = 0;
    for direction in content.chars() {
        position += 1;
        floor += get_direction(direction) as i32;
        if floor == -1 {
            return Ok(position);
        }
    }
    Err("basement was never visited")
}

fn get_direction(char: char) -> i8 {
    match char {
        '(' => 1,
        ')' => -1,
        _ => unimplemented!(),
    }
}
