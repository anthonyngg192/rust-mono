use async_std::stream::StreamExt;
use std::{collections::HashMap, ops::Deref};

use crate::{traits::AbstractDatabase, Error, Result};
use mongodb::{
    bson::{doc, to_document, Document},
    results::{InsertOneResult, UpdateResult},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod user {
    pub mod user;
}

pub mod room {
    pub mod room;
}

pub mod conversation {
    pub mod conversation;
    pub mod message;
}

#[derive(Debug, Clone)]
pub struct MongoDb(pub ::mongodb::Client, pub String);

impl AbstractDatabase for MongoDb {}

impl Deref for MongoDb {
    type Target = mongodb::Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl MongoDb {
    pub fn db(&self) -> mongodb::Database {
        self.database("rust_demo")
    }

    pub fn col<T>(&self, collection: &str) -> mongodb::Collection<T>
    where
        T: Send + Sync,
    {
        self.db().collection(collection)
    }

    async fn insert_one<T: Serialize>(
        &self,
        collection: &'static str,
        document: T,
    ) -> Result<InsertOneResult>
    where
        T: Send + Sync,
    {
        self.col::<T>(collection)
            .insert_one(document)
            .await
            .map_err(|_| Error::DatabaseError {
                operation: "insert_one",
                with: collection,
            })
    }

    async fn find_with_option<T: DeserializeOwned + Unpin + Send + Sync>(
        &self,
        collection: &'static str,
        projection: Document,
    ) -> Result<Vec<T>> {
        let result = self.col::<T>(collection).find(projection).await;
        let cursor = result.map_err(|_| Error::DatabaseError {
            operation: "find",
            with: collection,
        })?;

        let documents: Vec<T> = cursor
            .filter_map(|doc| {
                match doc {
                    Ok(item) => Some(item), // Successfully retrieved document
                    Err(e) => {
                        eprintln!("Error processing document: {:?}", e); // Log the error
                        None
                    }
                }
            })
            .collect::<Vec<T>>()
            .await;

        // Return the collected documents
        Ok(documents)
    }

    async fn find_one<T: DeserializeOwned + Unpin + Send + Sync>(
        &self,
        collection: &'static str,
        projection: Document,
    ) -> Result<T> {
        self.find_one_with_options(collection, projection).await
    }

    async fn find_one_with_options<T: DeserializeOwned + Unpin + Send + Sync>(
        &self,
        collection: &'static str,
        projection: Document,
    ) -> Result<T> {
        self.col::<T>(collection)
            .find_one(projection)
            .await
            .map_err(|_| Error::DatabaseError {
                operation: "find_one",
                with: collection,
            })?
            .ok_or(Error::NotFound)
    }

    async fn find_one_by_id<T: DeserializeOwned + Unpin + Send + Sync>(
        &self,
        collection: &'static str,
        id: &str,
    ) -> Result<T> {
        self.find_one(
            collection,
            doc! {
                "_id":id
            },
        )
        .await
    }

    async fn update_one<P, T: Serialize>(
        &self,
        collection: &'static str,
        projection: Document,
        partial: T,
        remove: Vec<&dyn IntoDocumentPath>,
        prefix: P,
    ) -> Result<UpdateResult>
    where
        P: Into<Option<String>>,
        T: Send + Sync,
    {
        let prefix = prefix.into();

        let mut unset = doc! {};

        for field in remove {
            if let Some(path) = field.as_path() {
                if let Some(prefix) = &prefix {
                    unset.insert(prefix.to_owned() + path, 1_i32);
                } else {
                    unset.insert(path, 1_i32);
                }
            }
        }

        let query = doc! {
            "$unset":unset,
            "$set":if let Some(prefix) = &prefix{
                to_document(&prefix_key(&partial,prefix))
            }else{
                to_document(&partial)
            }.map_err(|_| Error::DatabaseError { operation: "to_document", with: collection })?
        };

        self.col::<T>(collection)
            .update_one(projection, query)
            .await
            .map_err(|_| Error::DatabaseError {
                operation: "update_one",
                with: collection,
            })
    }
}

#[derive(Deserialize)]
pub struct DocumentId {
    #[serde(rename = "_id")]
    pub id: String,
}

pub trait IntoDocumentPath: Send + Sync {
    fn as_path(&self) -> Option<&'static str>;
}

pub fn prefix_key<T: Serialize>(t: &T, prefix: &str) -> HashMap<String, serde_json::Value> {
    let v: String = serde_json::to_string(t).unwrap();
    let v: HashMap<String, serde_json::Value> = serde_json::from_str(&v).unwrap();

    v.into_iter()
        .filter(|(_k, v)| !v.is_null())
        .map(|(k, v)| (format!("{}{}", prefix.to_owned(), k), v))
        .collect()
}
