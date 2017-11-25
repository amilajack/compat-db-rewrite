extern crate rusqlite;
extern crate time;

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
    api_type: ApiType,
    ast_node_types: AstNodeTypes
}

struct DatabaseRecord {
    agents: String,
    records: Vec<Record>
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
