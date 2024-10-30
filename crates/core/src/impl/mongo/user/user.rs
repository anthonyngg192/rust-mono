use chrono::{Local, Utc};
use mongodb::bson::{doc, oid::ObjectId, Document};

use crate::{
    environment::HASH_ROUND,
    models::users::user::{CreateNewUser, Status, User},
    r#impl::mongo::MongoDb,
    traits::AbstractUser,
    Error, Result,
};

static COL: &str = "users";

#[async_trait]
impl AbstractUser for MongoDb {
    async fn generate_user_code(&self) -> Result<String> {
        let local_date = Local::now().naive_local().date();
        let formatted_date = local_date.format("%y%m%d").to_string();
        let prefix = nanoid::nanoid!(10, &nanoid::alphabet::SAFE);
        let user_code = format!("{}{}", formatted_date, prefix);
        Ok(user_code)
    }

    async fn create_user(
        &self,
        create_user: &CreateNewUser,
        password: &String,
        code: &String,
    ) -> Result<bool> {
        let hash_password = bcrypt::hash(password, *HASH_ROUND).unwrap();
        let now = Utc::now();
        let timestamp_millis = now.timestamp_millis();
        let object_id = ObjectId::new();

        let email_exists = self
            .col::<Document>(&COL)
            .find_one(doc! {
                "email":create_user.email.to_string()
            })
            .await;

        match email_exists {
            Ok(Some(_)) => {
                return Err(Error::EmailAlreadyExisted);
            }
            Ok(None) => {
                let result = self
                    .insert_one(
                        COL,
                        User {
                            id: Some(object_id.to_string()),
                            name: create_user.name.to_string(),
                            email: create_user.email.to_string(),
                            password: hash_password,
                            password_updated_at: timestamp_millis.clone(),
                            status: Status::Active,
                            code: code.to_string(),
                            blacklist: vec![],
                        },
                    )
                    .await;
                match result {
                    Ok(_data_result) => Ok(true),
                    Err(_err) => Err(Error::EmailAlreadyExisted),
                }
            }
            Err(_) => return Err(Error::InternalServer),
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
