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
    let filename = "records.json";
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let p: Vec<Record> = serde_json::from_str(&contents).expect("Does not match expected schema");
    println!("{:?} Records", p.len());

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
