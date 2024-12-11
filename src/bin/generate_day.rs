use clap::Parser;
use std::{
    env::current_dir,
    fs::{create_dir, write, OpenOptions},
    io::prelude::*,
    path::PathBuf,
};

/// Prepare your package for solving puzzles
#[derive(Parser, Debug)]
#[command(version)]
struct Arguments {
    /// Year of the event
    year: u16,
    /// Day of the year
    day: u8,
}

fn main() {
    let args = Arguments::parse();
    let project_dir = current_dir().unwrap();

    // check if rust project exists
    let package_dir = project_dir.join("src");
    if !package_dir.is_dir() {
        panic!("not a rust project")
    }

    create_module_day(&package_dir, args.year, args.day).unwrap();
    create_module_year(&package_dir, args.year, args.day).unwrap();

    // update library
    let module = package_dir.join("lib.rs");
    if !module.is_file() {
        panic!("library module is missing");
    }
}

/// Create a new module for specific year and day.
fn create_module_day(package_dir: &PathBuf, year: u16, day: u8) -> Result<(), String> {
    let module_dir = package_dir.join(format!("year{year:04}"));
    if !module_dir.is_dir() {
        create_dir(&module_dir).expect("unable to create directory");
    }

    let module = module_dir.join(format!("day{day:02}.rs"));
    if module.is_file() {
        return Err(format!("file already exists: {module:?}"));
    }

    let contents = b"\
pub fn part1(content: &str) -> u32 {
    let total = 0;
    total
}

pub fn part2(content: &str) -> u32 {
    let total = 0;
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = \"\";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
";

    write(&module, contents).expect("unable to create file");
    println!("new module created: {module:?}");

    Ok(())
}

/// Update or create module for specific year.
fn create_module_year(package_dir: &PathBuf, year: u16, day: u8) -> Result<(), String> {
    let module = package_dir.join(format!("year{year:04}.rs"));
    let contents = format!("pub mod day{day:02};\n");

    if module.is_file() {
        let mut file = OpenOptions::new().append(true).open(&module).expect("unable to open file");
        file.write(contents.as_bytes()).expect("unable to write to file");
        println!("module updated: {module:?}");
    } else {
        write(&module, contents.as_bytes()).expect("unable to create file");
        println!("new module created: {module:?}");
    }
    
    Ok(())
}
