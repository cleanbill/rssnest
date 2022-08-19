use feeds_utils::Feed_info;
use log::*;
use rss::{Channel, Item};
use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::Path, time::{SystemTime, UNIX_EPOCH},
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
    //dbg!(&channel);
    Ok(channel)
}

async fn download(name: &str, url: &str, work_dir: &str) -> Result<(), Box<dyn Error>> {
    let content = reqwest::get(url).await?.bytes().await?;

    let slash_index = url.rfind("/").unwrap()+1;   
    let filename = &url[slash_index..url.len()];
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH).unwrap();
    let prefix = format!("{}-{:?}", name, since_the_epoch);

    fs::create_dir_all(work_dir)?;

    let filepath = format!("{}/{}-{}", work_dir, prefix, filename);
    let path = Path::new(&filepath);

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    file.write_all(&content);
    eprintln!("{} has been saved", filepath);
    Ok(())
}

async fn load_rss(url: &str) -> Channel {
    let data = match load_feed(url).await {
        Ok(channel) => channel,
        Err(e) => {
            error!(
                "{} failed to get rss '{}': {:?}",
                "Error:".red().bold(),
                url,
                e
            );
            std::process::exit(1);
        }
    };
    return data;
}

#[tokio::main]
async fn main() {
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
    let feed_data: Feed_info = feeds_utils::get_feeds(&config.general.feed_file);
    warn!("Feed data {:?}", feed_data.feeds.first());

    for feed in feed_data.feeds {
        let file_path = format!("{}/{}", &config.general.audio_dir, feed.dir);
        process(feed.name, &feed.url, &file_path).await;
    }
}

async fn process(name: String, url: &str, work_dir: &str) {
    let data = load_rss(url).await;
    process_item(name, data.items.first().unwrap(), work_dir).await;
}

async fn process_item(name: String, item: &Item, work_dir: &str) {
    // TODO find latest mp3
    let filename_opt = get_latest(item);
    if filename_opt == None {
        return;
    }
    let filename = filename_opt.unwrap();

    let connection = store::create();
    let have = store::already_have(filename, &connection);
    if have {
        warn!("Already have {}", filename);
        store::bump(filename, &connection);
    } else {
        warn!("Found new {}", &filename);
        // TODO download lastest mp3
        download(&name,filename, work_dir).await;
        store::insert(name, filename);
    }
}

fn get_latest(item: &Item) -> Option<&str> {
    let enclosure_opt = item.enclosure();
    if (enclosure_opt == None) {
        return None;
    }
    let enclosure = enclosure_opt.unwrap();
    return Some(enclosure.url());
}
