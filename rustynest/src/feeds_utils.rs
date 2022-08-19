
use log::*;
use serde::Deserialize;
use text_colorizer::Colorize;
use std::{fs};


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
    pub dir: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Feed_info {
    pub misc: Misc,
    pub feeds: Vec<Feed>
}

pub fn get_feeds(filename: &str) -> Feed_info  {
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
    let feed_data: Feed_info =
    serde_json::from_str(&data).expect("JSON was not well-formatted");
   // dbg!(&config);  
   // eprint!("got config back and here the feed file {} ",&config.general.feed_file);
   warn!("using '{}' feed file",filename);
   feed_data
}