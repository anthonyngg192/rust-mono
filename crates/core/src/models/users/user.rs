use core::fmt;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
// #[serde(tag = "status")]
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

    #[schemars(with = "String")]
    pub status: Status,

    pub blacklist: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct CreateNewUser {
    pub email: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct NewUserPayload {
    pub email: String,
    pub name: String,
    pub password: String,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Status::Active => "Active",
            Status::InActive => "InActive",
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct UserToken {
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: User,
    pub login_session: String,
}
