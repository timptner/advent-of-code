use log::info;
use reqwest::blocking::Client;
use std::{
    env,
    error::Error,
    fs::{create_dir_all, read_to_string, write},
    io::Error as IOError,
    path::PathBuf,
};

pub struct Puzzle {
    year: u16,
    day: u8,
}

impl Puzzle {
    pub fn new(year: u16, day: u8) -> Puzzle {
        if year < 2015 || year > 2024 {
            panic!("year must be 2015..=2024");
        }

        if day < 1 || day > 25 {
            panic!("day must be 1..=25");
        }

        Puzzle { year, day }
    }

    pub fn get_input(&self, client: Client) -> Result<String, Box<dyn Error>> {
        let contents;
        let path = self.get_input_path()?;
        if path.exists() {
            contents = read_to_string(&path)?;
        } else {
            let url = format!("https://adventofcode.com/{}/day/{}/input", self.year, self.day);
            contents = client.get(url).send()?.text()?;
            write(&path, &contents)?;
            info!("input written to file: {:?}", &path);
        }
        Ok(contents)
    }

    pub fn get_input_path(&self) -> Result<PathBuf, IOError> {
        let base_dir = match env::consts::OS {
            "linux" => PathBuf::from("~/.advent-of-code"),
            "windows" => {
                let data_dir = env::var("APPDATA").expect("APPDATA not set");
                PathBuf::from(data_dir).join("advent-of-code")
            },
            _ => unimplemented!("OS not supported"),
        };

        let path = base_dir
            .join("data")
            .join(format!("{}", self.year))
            .join(format!("day{:02}.txt", self.day));

        let parent_dir = path.parent().unwrap();
        if !parent_dir.exists() && env::consts::OS == "windows" {
            create_dir_all(&parent_dir)?;
            info!("new directory created: {:?}", &parent_dir)
        }

        Ok(path)
    }
}
