mod user {
    pub mod user;
}

mod room {
    pub mod room;
}

mod conversation {
    pub mod conversation;
    pub mod message;
}

pub use conversation::conversation::AbstractConversation;
pub use conversation::message::AbstractMessage;
pub use room::room::AbstractRoom;
pub use user::user::AbstractUser;

pub trait AbstractDatabase:
    Sync + Send + AbstractUser + AbstractRoom + AbstractConversation + AbstractMessage
{
}
