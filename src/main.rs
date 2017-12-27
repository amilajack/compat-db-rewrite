#[macro_use]
extern crate diesel;
extern crate rusqlite;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;
extern crate dotenv;

mod assertion_formatter;
mod multiple_assertion_formatter;
mod record;
mod database;
mod constants;
mod has_prefix;
mod test_initiator;

use std::fs::File;
use record::Record;
use std::io::prelude::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

fn main() {
    // Open the records
    let mut file = File::open("records.json").expect("File not found");

    // Create a file buffer
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");

    // Parse the records
    let records: Vec<Record> =
        serde_json::from_str(&contents).expect("Does not match expected schema");
    println!("{:?} Records", records.len());
}
