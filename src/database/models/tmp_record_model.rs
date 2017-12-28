use record::ApiType;

pub struct TmpRecords {
    pub id: String,
    pub name: String,
    pub proto_chain_id: String,
    pub versions: String,
    pub api_type: ApiType,
    pub caniuse_id: String,
}
