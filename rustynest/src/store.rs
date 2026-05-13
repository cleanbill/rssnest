use log::warn;
use sqlite::{Connection, State};
// use chrono::{NaiveDateTime};

// struct Audio {
//     pub id: u8,
//     pub filename: String,
//     pub checked_qty: u8,
//     pub timestamp: NaiveDateTime
// }

static DBNAME: &'static str = "rssnest.db";

pub fn create() -> Connection {
    let connection = sqlite::open(DBNAME).unwrap();

    // Do one for the feed lists?

    connection
        .execute(
            "
            CREATE TABLE IF NOT EXISTS mp3 
            (id INTEGER PRIMARY KEY AUTOINCREMENT, 
             filename TEXT NOT NULL UNIQUE,
             name TEXT NOT NULL,
             checked_qty INTEGER DEFAULT 1,
             Timestamp DATETIME DEFAULT CURRENT_TIMESTAMP);
            ",
        )
        .unwrap();

    connection
        .execute(
            "
            CREATE TABLE IF NOT EXISTS badfeed 
            (id INTEGER PRIMARY KEY AUTOINCREMENT, 
             feed_url TEXT NOT NULL UNIQUE,
             Timestamp DATETIME DEFAULT CURRENT_TIMESTAMP);
            ",
        )
        .unwrap();

    connection
}

pub fn already_have(filename: &str, connection: &Connection) -> bool {
    let mut statement = connection
        .prepare("SELECT 1 FROM mp3 WHERE filename = ?")
        .unwrap()
        .bind(1, filename)
        .unwrap();

    let mut found = false;

    while let State::Row = statement.next().unwrap() {
        found = true;
    }
    found
}

pub fn bad_feed(feed_url: &str, connection: &Connection) -> bool {
    let mut statement = connection
        .prepare("SELECT 1 FROM badfeed WHERE feed_url = ?")
        .unwrap()
        .bind(1, feed_url)
        .unwrap();

    let mut found = false;

    while let State::Row = statement.next().unwrap() {
        found = true;
    }
    found
}

pub fn delete_bad_feed(connection: &Connection) {
    // let connection = sqlite::open(DBNAME).unwrap();
    let sql = format!(" DELETE FROM badfeed ");
    warn!("Deleting {}", sql);
    connection.execute(sql).unwrap();
}

pub fn report_bad_feed(feed_url: String, connection: &Connection) {
    warn!("inserting bad feed {}", feed_url);
    let mut statement = connection
        .prepare("INSERT INTO badfeed (feed_url) VALUES (?)")
        .unwrap()
        .bind(1, &*feed_url)
        .unwrap();
    statement.next().unwrap();
}

pub fn insert(name: String, filename: &str) {
    let connection = sqlite::open(DBNAME).unwrap();
    warn!("inserting mp3 {} for {}", filename, name);
    let mut statement = connection
        .prepare("INSERT INTO mp3 (filename, name) VALUES (?, ?)")
        .unwrap()
        .bind(1, filename)
        .unwrap()
        .bind(2, &*name)
        .unwrap();
    statement.next().unwrap();
}

pub fn bump(filename: &str, connection: &Connection) {
    let mut statement = connection
        .prepare("UPDATE mp3 SET checked_qty = checked_qty + 1 WHERE filename = ?")
        .unwrap()
        .bind(1, filename)
        .unwrap();
    statement.next().unwrap();
}

pub fn housekeep(amount: u8, name: &str) {
    let connection = sqlite::open(DBNAME).unwrap();
    let amount = amount as i64;
    let mut statement = connection
        .prepare("DELETE FROM mp3 WHERE name = ? AND id NOT IN (SELECT DISTINCT id FROM mp3 ORDER BY id DESC LIMIT ?)")
        .unwrap()
        .bind(1, name)
        .unwrap()
        .bind(2, amount)
        .unwrap();
    statement.next().unwrap();
}
