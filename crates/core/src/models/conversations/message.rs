use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    #[serde(rename = "_id")]
    pub id: String,

    pub conversation_id: String,
    
    pub text: Option<String>,
    
    pub gif: Option<String>,
    
    pub image: Option<String>,
    
    pub images: Option<Vec<String>>,
}
