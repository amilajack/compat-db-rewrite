/// A job queue for temporary records. These records are called "temporary"
/// because they
///
/// Temporarily using rustqlite. Ideally, we could  migrate to Diesel so
/// we have a higher level API and we're not tied to one database.
/// Migration to MYSQL is ideal since we can create a pool of connections.
///
// @TODO: Migrate to Diesel
use serde_json;
use rayon::prelude::*;
use std::collections::HashMap;
use record::{ApiType, Record};
use database::models::tmp_record_model::TmpRecords;
use rusqlite::{Connection};

struct RecordQuery {
    api_type: ApiType,
    proto_chain_id: String,
}

pub struct TemporaryRecordDatabase {
    connection: Connection,
}

/// Store temporary records that will eventually be compiled
/// and formatted later
impl TemporaryRecordDatabase {
    fn new() -> Self {
        Self { connection: Connection::open(":memory:").unwrap() }
    }

    fn migrate(&self) {
        &self.connection.execute(
            r#"
                CREATE TABLE tmp_records (
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,
                    proto_chain_id TEXT NOT NULL,
                    versions TEXT NOT NULL,
                    api_type TEXT NOT NULL,
                    caniuse_id TEXT NOT NULL
                )
            "#,
            &[]
        );
    }

    /// Drop the database
    fn drop(&self) {
        &self.connection.execute("DROP TABLE tmp_records", &[]).unwrap();
    }

    /// Find all the compatibility records for every version of the same browser
    fn find_same_version_compat_record(&self, record: Record, caniuse_id: String) -> Vec<TmpRecords> {
        let connection = Connection::open(":memory:").unwrap();
        let mut query = connection
            .prepare(r#"
                SELECT *
                FROM tmp_records
                WHERE name = ?
                AND proto_chain_id = ?
                AND caniuse_id = ?
            "#)
            .unwrap();

        let rows = query.query_map(&[], |row| {
            TmpRecords {
                api_type: ApiType::js_api,
                id: row.get(0),
                name: row.get(0),
                proto_chain_id: row.get(0),
                versions: row.get(0),
                caniuse_id: row.get(0),
            }
        })
        .unwrap();

        let res = rows.map(|e| e.unwrap()).collect();
        res
    }

    /// Efficiently insert many temporary records
    pub fn insert_bulk_records(&self,
                            record: Record,
                            caniuse_id: String,
                            versions: Vec<String>,
                            is_supported: bool) {
        let mut newly_generated_record_versions: HashMap<String, bool> = HashMap::new();

        for version in versions {
            newly_generated_record_versions.insert(version, is_supported);
        }

        // Serialize the version records
        let stringifyed = serde_json::to_string(&newly_generated_record_versions).unwrap();

        // Create the temporary record
        &self.connection.execute(
            r#"
                UPDATE tmp_records
                SET versions = ?1
                WHERE caniuse_id = ?2
                AND name = ?3
                AND proto_chain_id = ?4
            "#,
            &[
                &stringifyed,
                &caniuse_id,
                &caniuse_id,
                &record.protoChainId
            ]
        );
    }
}
