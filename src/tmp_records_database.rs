/// A job queue for temporary records. These records are called "temporary"
/// because they
///
/// Temporarily using rustqlite. Ideally, we could  migrate to Diesel so
/// we have a higher level API and we're not tied to one database.
/// Migration to MYSQL is ideal since we can create a pool of connections.
///
// @TODO: Migrate to desil
use std::collections::HashMap;
use record::{ApiType, Record};
use diesel::prelude::*;

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

struct TemporaryRecordDatabase {}

impl TemporaryRecordDatabase {
    pub fn migrate() {
        let connection =
	    	SqliteConnection::establish(&"./database/compat-db.sqlite").expect("Connection could not be established");

    }

    /// Find all the compatibility records for every version of the same browser
    pub fn find_same_version_compat_record(record: Record, caniuse_id: String) {
        // connection.where({
        //     name: caniuseId,
        //     type: record.type,
        //     protoChainId: record.protoChainId,
        //     caniuseId
        // })
    }

    /// Efficiently insert many temporary records
    pub fn insert_bulk_records(record: Record,
                               caniuse_id: String,
                               versions: Vec<String>,
                               is_supported: bool)
                               -> Result<Vec<TemporaryRecord>, ConnectionError> {
        let newly_generated_record_versions: HashMap<String, bool> = HashMap::new();

        for version in versions {
            newly_generated_record_versions.insert(version, is_supported);
        }

        connection
          .where({
            caniuseId,
            name: caniuseId,
            protoChainId: record.protoChainId
          })
          .save({
            name: caniuseId,
            type: record.type,
            protoChainId: record.protoChainId,
            versions: serde_json::to_string(&newlyGenerateRecordVersions).unwrap(),
            caniuseId
          })
    }
}
