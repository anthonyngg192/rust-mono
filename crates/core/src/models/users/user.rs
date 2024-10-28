use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(tag = "Type")]
pub enum Status {
    Active,
    InActive,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(with = "String")]
    pub id: Option<String>, // Store ObjectId as String

    pub email: String,
    pub password: String,
    pub name: String,
    pub code: String,
    pub password_updated_at: i64,
    pub status: Status,
    pub blacklist: Vec<String>,
}
