use crate::{models::rooms::room::Room, r#impl::mongo::MongoDb, traits::AbstractRoom, Result};
use chrono::prelude::*;

static COL: &str = "rooms";

#[async_trait]
impl AbstractRoom for MongoDb {
    async fn create_room(&self, room: &Room) -> Result<()> {
        self.insert_one(COL, room).await.map(|_| ())
    }

    async fn generate_room_code(&self) -> Result<String> {
        let local_date = Local::now().naive_local().date();
        let formatted_date = local_date.format("%y%m%d").to_string();
        let prefix = nanoid::nanoid!(10, &nanoid::alphabet::SAFE);
        let code = format!("{}{}", formatted_date, prefix);
        Ok(code)
    }
}
