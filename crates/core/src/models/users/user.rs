use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(tag = "Type")]
pub enum Status {
    Active,
    InActive,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(tag = "Type")]
pub struct User {
    #[serde(rename = "_id")]
    pub id: String,
    
    pub email: String,
    pub password: String,
    pub name: String,
    pub code: String,
    pub password_updated_at: i128,
    pub status: Status,
    pub blacklist: Vec<String>,
}
