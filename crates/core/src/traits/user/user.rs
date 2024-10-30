use crate::models::users::user::{CreateNewUser, User, UserToken};
use crate::Result;

#[async_trait]
pub trait AbstractUser: Sync + Send {
    async fn create_user(
        &self,
        user: &CreateNewUser,
        password: &String,
        code: &String,
    ) -> Result<bool>;
    async fn find_user_by_login(&self, email: &str) -> Result<User>;
    async fn get_all(&self) -> Result<Vec<User>>;
    async fn generate_user_code(&self) -> Result<String>;
    async fn validate_user_token(&self, user_token: &UserToken) -> bool;
}
