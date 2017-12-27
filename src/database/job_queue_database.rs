// Manages a queue of test assertion jobs to be run
struct WhereClause {
    browser_name: String,
    proto_chain_id: String,
}

struct JobQueueDatabase {}

impl JobQueueDatabase {
    pub fn migrate() {
        // table_name: Jobs
        //
        // table.increments('id');
        // table.string('name');
        // table.string('browserName');
        // table.string('platform');
        // table.string('protoChainId');
        // table.string('version');
        // table.enu('type', ['js-api', 'css-api', 'html-api']);
        // table.string('record', 5000);
        // table.string('caniuseId');
        // table.enu('status', ['queued', 'running']).defaultTo('queued');
    }

    pub fn mark_job_status() {}
}
