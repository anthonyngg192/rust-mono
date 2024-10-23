use crate::models::rooms::room::Room;
use crate::Result;

#[async_trait]
pub trait AbstractRoom: Sync + Send {
    async fn create_room(&self, room: &Room) -> Result<()>;
    async fn generate_room_code(&self) -> Result<String>;
}
