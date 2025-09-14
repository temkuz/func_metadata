use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FuncMetadata {
    pub name: String,
    pub input: Vec<FuncParam>,
    pub output: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FuncParam {
    pub name: String,
    pub r#type: String,
}
