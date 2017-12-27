/// A job queue for temporary records. These records are called "temporary"
/// because they
///
/// Temporarily using rustqlite. Ideally, we could  migrate to Diesel so
/// we have a higher level API and we're not tied to one database.
/// Migration to MYSQL is ideal since we can create a pool of connections.
///
// @TODO: Migrate to Diesel
use std::collections::HashMap;
use record::{ApiType, Record};
// use diesel::prelude::*;
// use database::schema::tmp_records::dsl;
use database::models::tmp_record_model::TmpRecords;
use rusqlite::Connection;
use database::Database;
use serde_json;

// #[derive(Queryable)]
struct TemporaryRecord {
    id: String,
    name: String,
    proto_chain_id: String,
    versions: String,
    api_type: ApiType,
    caniuse_id: String,
}

struct RecordQuery {
    api_type: ApiType,
    proto_chain_id: String,
}

// #[derive(Queryable)]
struct TemporaryRecordDatabase {
    connection: Connection,
}

/// Store temporary records that will eventually be compiled
/// and formatted later
impl TemporaryRecordDatabase {
    fn new() -> Self {
        Self { connection: Connection::open_in_memory().unwrap() }
    }

    fn migrate(&self) {
        &self.connection.execute(r#"
            CREATE TABLE tmp_records (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                proto_chain_id TEXT NOT NULL,
                versions TEXT NOT NULL,
                api_type TEXT NOT NULL,
                caniuse_id TEXT NOT NULL
            )
        "#,
        &[]);
    }

    fn drop(&self) {
        &self.connection.execute("DROP TABLE tmp_records", &[]).unwrap();
    }

    /// Find all the compatibility records for every version of the same browser
    fn find_same_version_compat_record(&self, record: Record, caniuse_id: String) {
        &self.connection.execute(
            r#"
                SELECT *
                FROM tmp_records
                WHERE name = ?1
                AND proto_chain_id = ?2
                AND caniuse_id = ?3
            "#,
            &[&caniuse_id, &record.protoChainId, &caniuse_id]
        ).unwrap();
    }

    /// Efficiently insert many temporary records
    fn insert_bulk_records(&self,
                            record: Record,
                            caniuse_id: String,
                            versions: Vec<String>,
                            is_supported: bool) {
        let mut newly_generated_record_versions: HashMap<String, bool> = HashMap::new();

        for version in versions {
            newly_generated_record_versions.insert(version, is_supported);
        }

        let stringifyed = serde_json::to_string(&newly_generated_record_versions).unwrap();

        &self.connection.execute(r#"
                INSERT INTO tmp_records (name, proto_chain_id, versions, api_type, caniuse_id)
                WHERE caniuse_id = ?1
                AND name = ?2
                AND proto_chain_id = ?3
                VALUES (?4, ?5, ?6, ?7, ?8)
            "#,
            &[&caniuse_id, &caniuse_id, &record.protoChainId, &caniuse_id, &record.protoChainId, &stringifyed, &"js_api", &caniuse_id]
        );

        // connection
        //   .where({
        //     caniuseId,
        //     name: caniuseId,
        //     protoChainId: record.protoChainId
        //   })
        //   .save({
        //     name: caniuseId,
        //     type: record.type,
        //     protoChainId: record.protoChainId,
        //     versions: serde_json::to_string(&newlyGenerateRecordVersions).unwrap(),
        //     caniuseId
        //   })
    }
}
