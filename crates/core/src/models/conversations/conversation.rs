use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Conversation {
    #[serde(rename = "_id")]
    pub id: String,
    
    room_id: String,
    participants: Vec<String>,
}
