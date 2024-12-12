pub mod commands;
pub mod grid;
pub mod year2023;
pub mod year2024;

use std::fs::read_to_string;

use log::error;

pub fn get_input(year: u16, day: u8) -> String {
    let path = commands::get_data_dir(year, day).join("input.txt");
    let content = match read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            error!("failed to read file: {e}");
            String::new()
        }
    };
    content
}
