use std::ops::Deref;

use mongodb::Client;

use crate::{
    environment::{DATABASE_NAME, DB_CONNECTION_STRING},
    r#impl::mongo::MongoDb,
    traits::AbstractDatabase,
};

#[derive(Clone, Debug)]
pub enum Database {
    MongoDb(MongoDb),
}

pub enum DatabaseInfo {
    Auto,
    MongoDb { uri: String, database_name: String },
    MongoDbFromClient(::mongodb::Client, String),
}

impl DatabaseInfo {
    #[async_recursion]
    pub async fn connect(self) -> Result<Database, String> {
        Ok(match self {
            DatabaseInfo::Auto => {
                DatabaseInfo::MongoDb {
                    uri: DB_CONNECTION_STRING.to_string(),
                    database_name: DATABASE_NAME.to_string(),
                }
                .connect()
                .await?
            }
            DatabaseInfo::MongoDb { uri, database_name } => {
                let client = Client::with_uri_str(uri)
                    .await
                    .map_err(|_err| "Failed to connection to database".to_string())?;
                Database::MongoDb(MongoDb(client, database_name))
            }
            DatabaseInfo::MongoDbFromClient(client, database_name) => {
                Database::MongoDb(MongoDb(client, database_name))
            }
        })
    }
}

impl Deref for Database {
    type Target = dyn AbstractDatabase;

    fn deref(&self) -> &Self::Target {
        match self {
            Database::MongoDb(mongo) => mongo,
        }
    }
}
