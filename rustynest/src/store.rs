use sqlite::{Connection, State};
use chrono::{NaiveDateTime};

#[derive(Debug)]
struct Audio {
    id: u8,
    filename: String,
    checked_qty: u8,
    timestamp: NaiveDateTime
}

static DBNAME: &'static str = "rssnest.db";

pub fn create() -> Connection {
    let connection = sqlite::open(DBNAME).unwrap();

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
}

pub fn already_have(filename: &str, connection: &Connection) -> bool {

    let mut statement = connection
    .prepare("SELECT * FROM mp3 WHERE filename = ?")
    .unwrap()
    .bind(1, &*filename)
    .unwrap();

    let mut found = false;

    while let State::Row = statement.next().unwrap() {
        let _filename = statement.read::<String>(0).unwrap();
        found = true;
    }
    return found
}

pub fn insert(name: String, filename: &str) {
    let connection = sqlite::open(DBNAME).unwrap();
    let sql = format!(" INSERT INTO mp3 (filename, name) VALUES ('{}','{}')", filename, name);
    connection.execute(sql).unwrap();
}

pub fn bump(filename: &str, connection: &Connection) {
    let sql = format!(
        " UPDATE mp3 SET checked_qty = checked_qty WHERE filename = '{}'",
        filename
    );
    connection.execute(sql).unwrap();
}

pub fn housekeep(amount: u8, name: &str) {
    let connection = sqlite::open(DBNAME).unwrap();
    let sql = format!("DELETE FROM mp3 WHERE name = '{}' and id NOT IN (SELECT DISTINCT id FROM mp3 ORDER BY id DESC LIMIT {})", name, amount);
    connection.execute(sql).unwrap();
}