extern crate rusqlite;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde_json::Error;

use rusqlite::Connection;

enum ApiType {
    value,
    attr,
    tag
}

enum AstNodeTypes {
    MemberExpression,
    NewExpression,
    CallExpression
}

struct Record {
    astNodeTypes: AstNodeTypes,
    id: String,
    name: String,
    protoChain: Vec<String>,
    protoChainId: String,
    specIsFinished: bool,
    apiType: ApiType
}

struct DatabaseRecord {
    agents: String,
    records: Vec<Record>
}

fn parseRecords() {
    #[derive(Serialize, Deserialize)]

}

fn main() {
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
        &[]
    ).unwrap();
}
