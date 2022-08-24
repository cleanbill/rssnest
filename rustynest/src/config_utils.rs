use log::*;
use serde::Deserialize;
use std::{fs};
use text_colorizer::Colorize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct General {
    pub feed_file: String,
    pub audio_dir: String,
    pub visual_dir: String,
    pub data_dir: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Propagate {
    qty_per_page: u8,
    ftp: Ftp,
    tweet: Tweet,
    api_key: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ftp {
    url: String,
    port: u8,
    usr: String,
    pw: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tweet {
    consumer_key: String,
    consumer_secret: String,
    access_token_key: String,
    access_token_secret: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub general: General,
    pub propagate: Propagate,
}

pub fn get_config(filename: &str) -> Config {
    let data: String = match fs::read_to_string(filename) {
        Ok(v) => v,
        Err(e) => {
            error!(
                "{} failed to read from file '{}': {:?}",
                "Error:".red().bold(),
                filename,
                e
            );
            std::process::exit(1);
        }
    };
    let config: Config = serde_json::from_str(&data).expect("JSON was not well-formatted");
    warn!("using '{}' config file", filename);
    config
}
