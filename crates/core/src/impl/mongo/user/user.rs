use mongodb::bson::doc;

use crate::{
    models::users::user::User, r#impl::mongo::MongoDb, traits::AbstractUser, Error, Result,
};

static COL: &str = "users";

#[async_trait]
impl AbstractUser for MongoDb {
    async fn create_user(&self, user: &User) -> Result<()> {
        self.insert_one(COL, user).await.map(|_| ())
    }

    async fn find_user_by_login(&self, email: &str) -> Result<User> {
        self.find_one(
            COL,
            doc! {
                email:email
            },
        )
        .await
        .map_err(|_| Error::DatabaseError {
            operation: "find",
            with: "users",
        })
    }
}
