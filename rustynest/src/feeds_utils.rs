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

pub fn get_feeds(filename: &str) -> Result<FeedInfo, Box<dyn std::error::Error>> {
    let data: String = fs::read_to_string(filename).map_err(|e| {
        error!(
            "{} failed to read from file '{}': {:?}",
            "Error:".red().bold(),
            filename,
            e
        );
        e
    })?;
    let feed_data: FeedInfo = serde_json::from_str(&data).map_err(|e| {
        error!(
            "{} failed to parse JSON from '{}': {:?}",
            "Error:".red().bold(),
            filename,
            e
        );
        e
    })?;
    warn!("using '{}' feed file", filename);
    Ok(feed_data)
}
