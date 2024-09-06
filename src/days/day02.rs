use std::num::ParseIntError;

pub fn calc_wrapping_paper(input: &str) -> Result<u32, ParseIntError> {
    let mut total = 0;

    for line in input.lines() {
        let dimensions: Vec<&str> = line.split('x').collect();

        let length: u16 = dimensions[0].parse()?;
        let width: u16 = dimensions[1].parse()?;
        let height: u16 = dimensions[2].parse()?;

        let area = calc_area(length, width, height);
        let items = vec![length, width, height];
        let slack = calc_slack(items);

        total += (area + slack) as u32;
    }

    Ok(total)
}

fn calc_area(length: u16, width: u16, height: u16) -> u16 {
    let x = width * height;
    let y = height * length;
    let z = length * width;
    
    let area = 2 * (x + y + z);
    area
}

fn calc_slack(mut items: Vec<u16>) -> u16 {
    items.sort();
    let slack = items[0] * items[1];
    slack
}

pub fn calc_ribbon(input: &str) -> Result<u32, ParseIntError> {
    let mut total = 0;

    for line in input.lines() {
        let dimensions: Vec<&str> = line.split("x").collect();

        let length: u16 = dimensions[0].parse()?;
        let width: u16 = dimensions[1].parse()?;
        let height: u16 = dimensions[2].parse()?;

        let items = vec![length, width, height];
        let wrap = calc_wrap(items);
        let bow = calc_bow(length, width, height);

        total += (wrap + bow) as u32;
    }

    Ok(total)
}

fn calc_wrap(mut items: Vec<u16>) -> u16 {
    items.sort();
    let wrap = (items[0] + items[1]) * 2;
    wrap
}

fn calc_bow(length: u16, width: u16, height: u16) -> u16 {
    let bow = length * width * height;
    bow
}
