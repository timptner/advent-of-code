use std::fs::read_to_string;
use std::io::Error;

pub mod days;

pub fn print_header(day: u8, title: &str) {
	println!("Year 2015, Day {day} - {title}");
}

pub fn get_input(day: u8) -> Result<String, Error> {
	let path = format!("./data/day{day:02}.txt");
	let text = read_to_string(path)?;
	Ok(text)
}
