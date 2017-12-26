table! {
    job_queue (id) {
        id -> Nullable<Integer>,
        name -> Text,
        proto_chain_id -> Text,
        versions -> Text,
        #[sql_name = type]
        type_ -> Text,
        caniuse_id -> Text,
    }
}

table! {
    tmp_records (id) {
        id -> Nullable<Integer>,
        name -> Text,
        proto_chain_id -> Text,
        versions -> Text,
        #[sql_name = type]
        type_ -> Text,
        caniuse_id -> Text,
    }
}
