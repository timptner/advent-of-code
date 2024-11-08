pub mod day01;

use std::{env, fs::read_to_string, io::Error, path::PathBuf};

pub fn get_input(day: u8) -> Result<String, Error> {
    let path = match env::consts::OS {
        "linux" => PathBuf::from("~/.advent-of-code"),
        "windows" => {
            let data_dir = env::var("APPDATA").expect("APPDATA not set");
            PathBuf::from(data_dir).join("advent-of-code")
        },
        _ => unimplemented!("OS not supported"),
    };
    let path = path.join("2023").join(format!("{day:02}")).join("input.txt");
    dbg!(&path);
    let content = read_to_string(path)?;
    Ok(content)
}
