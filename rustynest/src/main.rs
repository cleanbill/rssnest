use feeds_utils::FeedInfo;
use log::*;
use rss::{Channel, Item};
use sqlite::Connection;
use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};
use text_colorizer::*;

pub mod config_utils;
pub mod feeds_utils;
pub mod store;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    #[structopt(short)]
    config_filename: String,

    /// Silence all output
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,
    /// Verbose mode (-v, -vv, -vvv, etc)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,
    /// Timestamp (sec, ms, ns, none)
    #[structopt(short = "t", long = "timestamp")]
    ts: Option<stderrlog::Timestamp>,

    /// Silence all output
    #[structopt(short = "d", long = "delete")]
    del: bool,
}

async fn load_feed(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url).await?.bytes().await?;
    
    // First try standard strict parsing
    match Channel::read_from(&content[..]) {
        Ok(channel) => Ok(channel),
        Err(e) => {
            warn!("Strict XML parsing failed for {}: {:?}. Attempting text cleanup...", url, e);
            
            let mut text = String::from_utf8_lossy(&content).into_owned();
            
            // Strip any junk before the actual XML starts (like BOMs or trailing whitespace)
            let start = text.find("<?xml").or_else(|| text.find("<rss")).unwrap_or(0);
            text = text[start..].to_string();
            
            // Clean up typical control characters that break parsers (keeping \n, \r, \t)
            text.retain(|c| c == '\n' || c == '\r' || c == '\t' || c >= '\u{20}');
            
            match Channel::read_from(text.as_bytes()) {
                Ok(channel) => {
                    warn!("Successfully parsed {} after text cleanup.", url);
                    Ok(channel)
                },
                Err(e2) => Err(e2.into())
            }
        }
    }
}

async fn download(name: &str, url: &str, work_dir: &str) -> Result<(), Box<dyn Error>> {
    let content = reqwest::get(url).await?.bytes().await?;

    let url_without_query = url.split('?').next().unwrap_or(url);
    let mut filename = url_without_query.split('/').last().unwrap_or("download");
    if filename.is_empty() {
        filename = "download";
    }

    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    let prefix = format!("{}-{:?}", name, since_the_epoch.as_secs());

    fs::create_dir_all(work_dir)?;

    let filepath = format!("{}/{}-{}", work_dir, prefix, filename);
    let path = Path::new(&filepath);

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", filepath, why),
        Ok(file) => file,
    };
    match file.write_all(&content) {
        Ok(_ok) => eprintln!("{} has been saved", filepath),
        Err(e) => {
            error!(
                "{} failed to get download '{}': {:?}",
                "Error:".red().bold(),
                url,
                e
            );
        }
    }

    Ok(())
}

async fn load_rss(url: &str) -> Result<Channel, String> {
    let data = match load_feed(url).await {
        Ok(channel) => channel,
        Err(e) => {
            error!(
                "{} failed to load rss '{}': {:?}",
                "Error:".red().bold(),
                url,
                e
            );
            return Err("Can't load".to_string());
        }
    };
    return Ok(data);
}

#[tokio::main]
async fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let opt = Opt::from_args();

    let _ = stderrlog::new()
        .module(module_path!())
        .quiet(opt.quiet)
        .verbosity(opt.verbose)
        .timestamp(opt.ts.unwrap_or(stderrlog::Timestamp::Off))
        .init();

    info!("Welcome");

    let config = config_utils::get_config(&opt.config_filename);
    warn!("Finding {}", config.general.feed_file);
    let feed_data: FeedInfo = feeds_utils::get_feeds(&config.general.feed_file);

    let connection = store::create();
    if opt.del {
        warn!("Clearing down all bad feeds");
        store::delete_bad_feed(&connection);
    }

    for feed in feed_data.feeds {
        let maron = feed.name == "wtfpod";
        let bad = store::bad_feed(&feed.url, &connection);
        if bad && maron {
            warn!("Bad Maron Feed data {:?}", feed);
            error!("Skipping the bad mark {}", &feed.url);
        }
        if bad && !maron {
            error!("Marked as bad feed {}", &feed.url);
        } else {
            let file_path = format!("{}/{}", &config.general.audio_dir, feed.dir);
            process(
                feed.name.replace(" ", "-"),
                &feed.url,
                &file_path,
                &connection,
            )
            .await;
        }
    }
    let elapsed = now.elapsed();
    warn!("finished {:.2?}", elapsed);
}

async fn process(name: String, url: &str, work_dir: &str, connection: &Connection) {
    let result = load_rss(url).await;
    match result {
        Ok(data) => {
            if let Some(first_item) = data.items.first() {
                process_item(name, first_item, work_dir, connection, url).await
            } else {
                warn!("Feed '{}' has no items", url);
            }
        }
        Err(_err) => {
            if name == "wtfpod" {
                warn!("Not reporting wtfpod as bad");
            } else {
                store::report_bad_feed(url.to_string(), connection)
            }
        }
    }
}

async fn process_item(
    name: String,
    item: &Item,
    work_dir: &str,
    connection: &Connection,
    url: &str,
) {
    let Some(enclosure_url) = get_latest_url(item) else {
        return;
    };

    let have = store::already_have(enclosure_url, connection);
    if have {
        warn!("Already have {}", enclosure_url);
        store::bump(enclosure_url, connection);
    } else {
        warn!("Found new {}", enclosure_url);
        // TODO download lastest mp3
        match download(&name, enclosure_url, work_dir).await {
            Ok(_) => {
                store::insert(name, enclosure_url);
            }
            Err(e) => {
                error!(
                    "{} failed to process item in rss '{}': {:?}",
                    "Error:".red().bold(),
                    &name,
                    e
                );
                store::report_bad_feed(url.to_string(), connection);
            }
        }
    }
}

fn get_latest_url(item: &Item) -> Option<&str> {
    item.enclosure().map(|e| e.url())
}
