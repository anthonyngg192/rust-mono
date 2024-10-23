use crate::models::conversations::conversation::Conversation;
use crate::Result;

#[async_trait]
pub trait AbstractConversation: Sync + Send {
    async fn get_conversation(&self, room_id: &String) -> Result<Conversation>;
}
