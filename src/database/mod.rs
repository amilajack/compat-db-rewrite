pub mod schema;
pub mod models;
pub mod tmp_records_database;
pub mod job_queue_database;

use rusqlite;

pub struct Database {
    table_name: String,
    connection: rusqlite::Connection
}

pub trait DatabaseTrait<T> {
    /// Remove an item in the database
    fn migrate(&self);
    /// Drop the databases and re-migrate them
    fn drop(&self);
    /// Get the number of records in a database
    fn count() -> u8;
    /// Find all the compatibility records for every version of the same browser
    fn find_same_version_compat_record() -> Vec<T>;
    /// Insert a large amount of records
    fn insert_bulk();
}
