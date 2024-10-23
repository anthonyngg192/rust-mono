use crate::models::conversations::message::Message;
use crate::Result;

#[async_trait]
pub trait AbstractMessage: Sync + Send {
    async fn create_message(&self, message: &Message) -> Result<()>;
}
