#[derive(Queryable)]
pub struct JobQueueModel {
  pub name: String,
  pub browser_name: String,
  pub proto_chain_id: String,
  pub platform: String,
  pub version: String,
  pub record: String,
  pub api_type: ApiType,
  pub caniuse_id: String,
}