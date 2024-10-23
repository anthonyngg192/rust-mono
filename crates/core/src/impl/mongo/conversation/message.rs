use crate::{
    models::conversations::message::Message, r#impl::mongo::MongoDb, traits::AbstractMessage,
    Result,
};

static COL: &str = "messages";

#[async_trait]
impl AbstractMessage for MongoDb {
    async fn create_message(&self, message: &Message) -> Result<()> {
        self.insert_one(COL, message).await.map(|_| ())
    }
}
