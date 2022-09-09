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
}

async fn load_feed(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

async fn download(name: &str, url: &str, work_dir: &str) -> Result<(), Box<dyn Error>> {
    let content = reqwest::get(url).await?.bytes().await?;

    let slash_index = url.rfind("/").unwrap() + 1;
    let end_index = url.rfind(".").unwrap() + 4;
    let filename = &url[slash_index..end_index];
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    let prefix = format!("{}-{:?}", name, since_the_epoch);

    fs::create_dir_all(work_dir)?;

    let filepath = format!("{}/{}-{}", work_dir, prefix, filename);
    let path = Path::new(&filepath);

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
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
    };

    return Ok(());
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

    stderrlog::new()
        .module(module_path!())
        .quiet(opt.quiet)
        .verbosity(opt.verbose)
        .timestamp(opt.ts.unwrap_or(stderrlog::Timestamp::Off))
        .init()
        .unwrap();
    info!("Welcome");

    let config = config_utils::get_config(&opt.config_filename);
    warn!("Finding {}", config.general.feed_file);
    let feed_data: FeedInfo = feeds_utils::get_feeds(&config.general.feed_file);

    let connection = store::create();

    for feed in feed_data.feeds {
        warn!("Feed data {:?}", feed);
        let bad = store::bad_feed(&feed.url, &connection);

        if bad {
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
        Ok(data) => process_item(name, data.items.first().unwrap(), work_dir, connection, &url).await,
        Err(_err) => {
            store::report_bad_feed(url.to_string(), &connection)
        },
    }
}

async fn process_item(name: String, item: &Item, work_dir: &str, connection: &Connection, url: &str) {
    let filename_opt = get_latest(item);
    if filename_opt == None {
        return;
    }
    let filename = filename_opt.unwrap();

    let have = store::already_have(filename, &connection);
    if have {
        warn!("Already have {}", filename);
        store::bump(filename, &connection);
    } else {
        warn!("Found new {}", &filename);
        // TODO download lastest mp3
        match download(&name, filename, work_dir).await {
            Ok(channel) => channel,
            Err(e) => {
                error!(
                    "{} failed to get process item in rss '{}': {:?}",
                    "Error:".red().bold(),
                    &name,
                    e
                );
                store::report_bad_feed(url.to_string(), &connection);
                return;
            }
        };
        store::insert(name, filename);
    }
}

fn get_latest(item: &Item) -> Option<&str> {
    let enclosure_opt = item.enclosure();
    if enclosure_opt == None {
        return None;
    }
    let enclosure = enclosure_opt.unwrap();
    return Some(enclosure.url());
}
