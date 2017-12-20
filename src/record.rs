#[derive(Serialize, Deserialize)]
pub enum AstNodeTypes {
    MemberExpression,
    NewExpression,
    CallExpression,
}

#[derive(Serialize, Deserialize)]
pub struct Record {
    // @TODO
    pub astNodeTypes: Vec<AstNodeTypes>,
    pub id: String,
    pub name: String,
    pub protoChain: Vec<String>,
    pub protoChainId: String,
    pub isStatic: bool,
    pub specNames: Vec<String>,
    pub specIsFinished: bool,
    pub apiType: String,
}

pub struct DatabaseRecord {
    pub agents: String,
    pub records: Vec<Record>,
}
