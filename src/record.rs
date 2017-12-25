#[derive(Serialize, Deserialize)]
pub enum AstNodeTypes {
    MemberExpression,
    NewExpression,
    CallExpression,
}

pub enum ApiType {
    js_api = 0,
    css_api = 1,
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
    pub apiType: ApiType,
}

pub struct DatabaseRecord {
    pub agents: String,
    pub records: Vec<Record>,
}

pub enum JobStatus {
    pending = 0,
    active,
    failed,
}

pub struct JobQueueRecord {
    pub browserName: String,
    pub status: JobStatus,
}
