use csv::WriterBuilder;
use log::{error, info};
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use serde::Serialize;
use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::{create_dir_all, OpenOptions, read_to_string, write},
    io::Error as IOError,
    path::PathBuf,
};
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Validate)]
pub struct Answer {
    #[validate(range(min = 1, max = 2))]
    level: u8,
    #[validate(length(min = 1), custom(function = "validate_answer"))]
    value: String,
}

impl Answer {
    pub fn new(level: u8, value: String) -> Result<Answer, ValidationErrors> {
        let answer = Answer { level, value };
        match answer.validate() {
            Ok(_) => Ok(answer),
            Err(error) => Err(error),
        }
    }

    pub fn as_map(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert(String::from("level"), self.level.to_string());
        params.insert(String::from("answer"), self.value.clone());
        params
    }
}

fn validate_answer(value: &str) -> Result<(), ValidationError> {
    if value.chars().all(char::is_alphanumeric) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid character"))
    }
}

#[derive(Validate)]
pub struct Puzzle {
    #[validate(range(min = 2015, max = 2024))]
    year: u16,
    #[validate(range(min = 1, max = 25))]
    day: u8,
}

impl Puzzle {
    pub fn new(year: u16, day: u8) -> Result<Puzzle, ValidationErrors> {
        let puzzle = Puzzle { year, day };
        match puzzle.validate() {
            Ok(_) => Ok(puzzle),
            Err(error) => Err(error),
        }
    }

    pub fn get_input(&self, client: Client) -> Result<String, Box<dyn Error>> {
        let path = self.get_input_path()?;
        let url = format!("https://adventofcode.com/{}/day/{}/input", self.year, self.day);
        let contents = client.get(url).send()?.text()?;
        write(&path, &contents)?;
        info!("input written to file: {:?}", path);
        Ok(contents)
    }

    pub fn read_input(&self, client: Client) -> Result<String, Box<dyn Error>> {
        let path = self.get_input_path()?;
        let contents;
        if path.exists() {
            contents = read_to_string(path)?;
        } else {
            contents = self.get_input(client)?;
        }
        Ok(contents)
    }

    fn get_input_path(&self) -> Result<PathBuf, Box<dyn Error>> {
        let path = self.get_data_dir()?.join("input.txt");
        Ok(path)
    }

    fn get_data_dir(&self) -> Result<PathBuf, IOError> {
        let mut path = match env::consts::OS {
            "linux" => PathBuf::from("~/.advent-of-code"),
            "windows" => {
                let data_dir = env::var("APPDATA").expect("APPDATA not set");
                PathBuf::from(data_dir).join("advent-of-code")
            },
            _ => unimplemented!("OS not supported"),
        };

        path.push("data");
        path.push(format!("{}", self.year));
        path.push(format!("{:02}", self.day));

        if !path.exists() && env::consts::OS == "windows" {
            create_dir_all(&path)?;
            info!("new directory created: {:?}", &path)
        }

        Ok(path)
    }

    pub fn submit_answer(&self, client: Client, answer: Answer) -> Result<String, Box<dyn Error>> {
        let url = format!("https://adventofcode.com/{}/day/{}/answer", self.year, self.day);
        let contents = client.post(url).form(&answer.as_map()).send()?.text()?;

        let selector = Selector::parse("html body main article p").unwrap();
        let document = Html::parse_document(&contents);
        let result: String = match document.select(&selector).next() {
            Some(reference) => reference.inner_html(),
            None => {
                error!("missing answer in response");
                String::from("NO ANSWER FOUND")
            },
        };

        let result = result.split('.').next().unwrap().to_owned();

        let path = self.get_data_dir()?.join("answers.csv");
        let mut writer;
        if path.exists() {
            let file = OpenOptions::new()
                .append(true)
                .open(path)?;
            writer = WriterBuilder::new().has_headers(false).from_writer(file);
        } else{
            writer = WriterBuilder::new().from_path(path)?;
        }
        writer.serialize(Row {
            level: answer.level,
            answer: answer.value,
            result: result.clone(),
        })?;
        writer.flush()?;
        info!("answer written to file");

        Ok(result)
    }
}

#[derive(Serialize)]
struct Row {
    level: u8,
    answer: String,
    result: String,
}
