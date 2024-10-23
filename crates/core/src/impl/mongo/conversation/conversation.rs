use mongodb::bson::doc;

use crate::{
    models::conversations::conversation::Conversation, r#impl::mongo::MongoDb,
    traits::AbstractConversation, Error, Result,
};

static COL: &str = "conversations";

#[async_trait]
impl AbstractConversation for MongoDb {
    async fn get_conversation(&self, room_id: &String) -> Result<Conversation> {
        self.find_one(
            COL,
            doc! {
                room_id:room_id
            },
        )
        .await
        .map_err(|_| Error::DatabaseError {
            operation: "find",
            with: "users",
        })
    }
}
