use crate::models::users::user::User;
use crate::Result;

#[async_trait]
pub trait AbstractUser: Sync + Send {
    async fn create_user(&self, user: &User) -> Result<bool>;
    async fn find_user_by_login(&self, email: &str) -> Result<User>;
    async fn get_all(&self) -> Result<Vec<User>>;
}
