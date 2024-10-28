use mongodb::bson::doc;

use crate::{
    models::users::user::User, r#impl::mongo::MongoDb, traits::AbstractUser, Error, Result,
};

static COL: &str = "users";

#[async_trait]
impl AbstractUser for MongoDb {
    async fn create_user(&self, user: &User) -> Result<bool> {
        let result = self.insert_one(COL, user).await;
        match result {
            Ok(data_result) => {
                println!("{:?}", data_result.inserted_id.clone());
                Ok(true)
            }
            Err(err) => {
                println!("{:?}", err.clone());
                Err(Error::DatabaseError {
                    operation: "find",
                    with: "users",
                })
            }
        }
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

    async fn get_all(&self) -> Result<Vec<User>> {
        self.find_with_option(COL, doc! {}).await
    }
}
