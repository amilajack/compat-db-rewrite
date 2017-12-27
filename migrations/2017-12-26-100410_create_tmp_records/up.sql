CREATE TABLE tmp_records
(
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    proto_chain_id TEXT NOT NULL,
    versions TEXT NOT NULL,
    api_type TEXT NOT NULL,
    caniuse_id TEXT NOT NULL
)
