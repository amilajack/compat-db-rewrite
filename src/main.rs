extern crate rusqlite;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod assertion_formatter;
mod record;

use std::fs::File;
use std::io::prelude::*;
use rusqlite::Connection;
use record::Record;

fn main() {
    // Open the records
    let mut file = File::open("records.json").expect("file not found");

    // Create a file buffer
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let records: Vec<Record> =
        serde_json::from_str(&contents).expect("Does not match expected schema");
    println!("{:?} Records", records.len());

    // Temporarily using rustqlite. Ideally, we could  migrate to desil so
    // we have a higher level API and we're not tied to one database.
    // Migration to MYSQL is ideal since we can create a pool of connections.
    let conn = Connection::open_in_memory().unwrap();

    conn.execute(
        "CREATE TABLE person (
            id              INTEGER PRIMARY KEY,
            name            TEXT NOT NULL,
            time_created    TEXT NOT NULL,
            data            BLOB
        )",
        &[],
    ).unwrap();
}
