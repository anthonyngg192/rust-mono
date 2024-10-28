use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(tag = "Type")]
pub enum RoomType {
    Public,
    Private,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(tag = "Type")]
pub enum RoomStatus {
    Stated,
    Expired,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Room {
    #[serde(rename = "_id")]
    pub id: String,
    
    pub owner_code: String,
    pub blacklist: Vec<String>,
    pub code: String,
    pub room_type: RoomType,
    pub expired_at: i64,
    pub display_name: String,
    pub status: RoomStatus,
    pub limit: Option<i8>,
    pub room_start: Option<i64>,
    pub room_end: Option<i64>,
}

pub struct CreateRoom {}
