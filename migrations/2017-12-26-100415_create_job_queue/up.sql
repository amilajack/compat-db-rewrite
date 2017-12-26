CREATE TABLE job_queue (
    id                TEXT PRIMARY KEY,
    name              TEXT NOT NULL,
    proto_chain_id    TEXT NOT NULL,
    versions          TEXT NOT NULL,
    type              TEXT NOT NULL,
    caniuse_id        TEXT NOT NULL
)