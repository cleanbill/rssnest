use log::*;
use serde::Deserialize;
use std::fs;
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
    pub qty_per_page: u8,
    pub ftp: Ftp,
    pub tweet: Tweet,
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ftp {
    pub url: String,
    pub port: u8,
    pub usr: String,
    pub pw: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tweet {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_token_key: String,
    pub access_token_secret: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub general: General,
    pub propagate: Propagate,
}

pub fn get_config(filename: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let data: String = fs::read_to_string(filename).map_err(|e| {
        error!(
            "{} failed to read from file '{}': {:?}",
            "Error:".red().bold(),
            filename,
            e
        );
        e
    })?;
    let config: Config = serde_json::from_str(&data).map_err(|e| {
        error!(
            "{} failed to parse JSON from '{}': {:?}",
            "Error:".red().bold(),
            filename,
            e
        );
        e
    })?;
    warn!("using '{}' config file", filename);
    Ok(config)
}
