use log::*;
use serde::Deserialize;
use std::fs;
use text_colorizer::Colorize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Misc {
    pub user: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Feed {
    pub name: String,
    pub desc: String,
    pub url: String,
    pub dir: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FeedInfo {
    pub misc: Misc,
    pub feeds: Vec<Feed>,
}

pub fn get_feeds(filename: &str) -> FeedInfo {
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
    let feed_data: FeedInfo = serde_json::from_str(&data).expect("JSON was not well-formatted");
    warn!("using '{}' feed file", filename);
    feed_data
}
