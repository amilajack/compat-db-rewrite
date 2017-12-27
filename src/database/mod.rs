pub mod schema;
pub mod models;
pub mod tmp_records_database;
pub mod job_queue_database;

pub trait Database {
    fn migrate(&self);
    fn drop(&self);
}
