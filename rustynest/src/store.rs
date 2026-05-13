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

pub fn create() -> Result<Connection, sqlite::Error> {
    let connection = sqlite::open(DBNAME)?;

    // Do one for the feed lists?

    connection.execute(
        "
            CREATE TABLE IF NOT EXISTS mp3 
            (id INTEGER PRIMARY KEY AUTOINCREMENT, 
             filename TEXT NOT NULL UNIQUE,
             name TEXT NOT NULL,
             checked_qty INTEGER DEFAULT 1,
             Timestamp DATETIME DEFAULT CURRENT_TIMESTAMP);
            ",
    )?;

    connection.execute(
        "
            CREATE TABLE IF NOT EXISTS badfeed 
            (id INTEGER PRIMARY KEY AUTOINCREMENT, 
             feed_url TEXT NOT NULL UNIQUE,
             Timestamp DATETIME DEFAULT CURRENT_TIMESTAMP);
            ",
    )?;

    Ok(connection)
}

pub fn already_have(filename: &str, connection: &Connection) -> Result<bool, sqlite::Error> {
    let mut statement = connection
        .prepare("SELECT 1 FROM mp3 WHERE filename = ?")?
        .bind(1, filename)?;

    let mut found = false;

    while let State::Row = statement.next()? {
        found = true;
    }
    Ok(found)
}

pub fn bad_feed(feed_url: &str, connection: &Connection) -> Result<bool, sqlite::Error> {
    let mut statement = connection
        .prepare("SELECT 1 FROM badfeed WHERE feed_url = ?")?
        .bind(1, feed_url)?;

    let mut found = false;

    while let State::Row = statement.next()? {
        found = true;
    }
    Ok(found)
}

pub fn delete_bad_feed(connection: &Connection) -> Result<(), sqlite::Error> {
    // let connection = sqlite::open(DBNAME).unwrap();
    let sql = format!(" DELETE FROM badfeed ");
    warn!("Deleting {}", sql);
    connection.execute(sql)?;
    Ok(())
}

pub fn report_bad_feed(feed_url: String, connection: &Connection) -> Result<(), sqlite::Error> {
    warn!("inserting bad feed {}", feed_url);
    let mut statement = connection
        .prepare("INSERT INTO badfeed (feed_url) VALUES (?)")?
        .bind(1, &*feed_url)?;
    statement.next()?;
    Ok(())
}

pub fn insert(name: String, filename: &str) -> Result<(), sqlite::Error> {
    let connection = sqlite::open(DBNAME)?;
    warn!("inserting mp3 {} for {}", filename, name);
    let mut statement = connection
        .prepare("INSERT INTO mp3 (filename, name) VALUES (?, ?)")?
        .bind(1, filename)?
        .bind(2, &*name)?;
    statement.next()?;
    Ok(())
}

pub fn bump(filename: &str, connection: &Connection) -> Result<(), sqlite::Error> {
    let mut statement = connection
        .prepare("UPDATE mp3 SET checked_qty = checked_qty + 1 WHERE filename = ?")?
        .bind(1, filename)?;
    statement.next()?;
    Ok(())
}

pub fn housekeep(amount: u8, name: &str) -> Result<(), sqlite::Error> {
    let connection = sqlite::open(DBNAME)?;
    let amount = amount as i64;
    let mut statement = connection
        .prepare("DELETE FROM mp3 WHERE name = ? AND id NOT IN (SELECT DISTINCT id FROM mp3 ORDER BY id DESC LIMIT ?)")?
        .bind(1, name)?
        .bind(2, amount)?;
    statement.next()?;
    Ok(())
}
