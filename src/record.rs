#[derive(Serialize, Deserialize)]
pub enum AstNodeTypes {
    MemberExpression,
    NewExpression,
    CallExpression,
}

#[derive(Serialize, Deserialize)]
pub struct Record {
    // @TODO
    // astNodeTypes: AstNodeTypes,
    pub id: String,
    pub name: String,
    pub protoChain: Vec<String>,
    pub protoChainId: String,
    pub specIsFinished: bool,
    pub apiType: String,
}

pub struct DatabaseRecord {
    pub agents: String,
    pub records: Vec<Record>,
}
