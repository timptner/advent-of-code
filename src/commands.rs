use csv::WriterBuilder;
use log::{error, info, warn};
use reqwest::{
    blocking,
    header::{HeaderMap, HeaderValue},
    StatusCode,
};
use scraper;
use serde::Serialize;
use std::{collections::HashMap, env, fs, path::PathBuf};

pub fn load_input(year: u16, day: u8) {
    let client = prepare_client();

    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let response = match client.get(url).send() {
        Ok(response) => response,
        Err(e) => {
            error!("request failed: {e}");
            return;
        }
    };
    let status = response.status();
    let content = response.text().unwrap_or_else(|e| {
        error!("failed to decode: {e}");
        String::new()
    });
    match status {
        StatusCode::OK => (),
        StatusCode::BAD_REQUEST => {
            error!("response failed [{status:?}]: {content}");
            return;
        }
        _ => panic!("unexpected status"),
    }

    let path = get_data_dir(year, day).join("input.txt");

    match fs::write(&path, &content) {
        Ok(_) => info!("input written to file: {:?}", &path),
        Err(e) => error!("failed to write: {e}"),
    };
}


pub fn submit_answer(year: u16, day: u8, level: u8, answer: String) {
    let client = prepare_client();

    let url = format!("https://adventofcode.com/{year}/day/{day}/answer");
    let mut params = HashMap::new();
    params.insert(String::from("level"), level.to_string());
    params.insert(String::from("answer"), answer.clone());
    let response = match client.post(url).form(&params).send() {
        Ok(response) => response,
        Err(e) => {
            error!("request failed: {e}");
            return;
        }
    };
    let content = response.text().unwrap_or_else(|e| {
        error!("failed to decode: {e}");
        String::new()
    });

    let selector = scraper::Selector::parse("html body main article p").unwrap();
    let document = scraper::Html::parse_document(&content);
    let result: String = match document.select(&selector).next() {
        Some(reference) => reference.inner_html(),
        None => {
            error!("missing answer in response");
            return;
        }
    };

    let result = result.split('.').next().unwrap().to_owned();

    let path = get_data_dir(year, day).join("answers.csv");
    let mut writer;
    if path.exists() {
        let file = match fs::OpenOptions::new().append(true).open(path) {
            Ok(file) => file,
            Err(e) => {
                error!("failed to open file: {e}");
                return;
            }
        };
        writer = WriterBuilder::new().has_headers(false).from_writer(file);
    } else {
        writer = match WriterBuilder::new().from_path(path) {
            Ok(writer) => writer,
            Err(e) => {
                error!("failed to build writer: {e}");
                return;
            }
        };
    }
    let row = Row {
        level,
        answer,
        result,
    };
    match writer.serialize(row) {
        Ok(_) => (),
        Err(e) => {
            error!("failed to serialize: {e}");
            return;
        }
    };
    match writer.flush() {
        Ok(_) => info!("answer written to file"),
        Err(e) => error!("failed t write: {e}"),
    };
}

fn prepare_client() -> blocking::Client {
    let key: &str = "SESSION_TOKEN";
    let token = env::var(key).unwrap_or_else(|e| {
        warn!("{e}: {key}");
        String::new()
    });

    let cookie = format!("session={}", token);
    let value = HeaderValue::from_str(&cookie).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("Cookie", value);

    blocking::Client::builder()
        .default_headers(headers)
        .build()
        .expect("prepared client")
}

pub fn get_data_dir(year: u16, day: u8) -> PathBuf {
    let mut path = PathBuf::from("data");
    path.push(format!("{year}"));
    path.push(format!("{day:02}"));

    if !path.exists() {
        match fs::create_dir_all(&path) {
            Ok(_) => (),
            Err(e) => error!("failed to make directories: {e}"),
        };
        info!("new directory created: {:?}", &path)
    }

    path
}

#[derive(Serialize)]
struct Row {
    level: u8,
    answer: String,
    result: String,
}
