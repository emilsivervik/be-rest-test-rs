#[async_trait]
/// Generic trait for implementing basic MongoDB operations.
pub trait Storage<T: Serialize + DeserializeOwned + Unpin + Send + Sync, C> {
    fn new(collection: C) -> Self;

    fn get_collection(&self) -> Arc<Mutex<Collection<T>>>;

    async fn create(&self, create_data: &T) -> Result<InsertOneResult, Error> {
        self.get_collection()
            .clone()
            .lock()
            .await
            .insert_one(create_data, None)
            .await
    }

    async fn update(&self, oid: &ObjectId, update_value: &Document) -> Result<UpdateResult, Error> {
        self.get_collection()
            .clone()
            .lock()
            .await
            .update_one(doc! {"_id": oid }, doc! { "$set": update_value }, None)
            .await
    }

    async fn get(&self, oid: &ObjectId) -> Result<Option<T>, Error> {
        self.get_collection()
            .clone()
            .lock()
            .await
            .find_one(doc! {"_id": oid }, None)
            .await
    }

    async fn find(&self, filter: Option<Document>) -> Result<Cursor<T>, Error> {
        self.get_collection()
            .clone()
            .lock()
            .await
            .find(filter, None)
            .await
    }

    async fn remove(&self, oid: &ObjectId) -> Result<bool, Error> {
        let deletion = self
            .get_collection()
            .clone()
            .lock()
            .await
            .delete_one(doc! {"_id": oid }, None)
            .await?;

        Ok(deletion.deleted_count == 1)
    }

    async fn list(&self) -> Result<Vec<T>, Error> {
        let cursor = self
            .get_collection()
            .clone()
            .lock()
            .await
            .find(doc! {}, None)
            .await?;
        cursor.try_collect::<Vec<T>>().await
    }
}

use bson::{doc, oid::ObjectId, Document};
use mongodb::error::Error;
use mongodb::results::{InsertOneResult, UpdateResult};
use mongodb::{Collection, Cursor};
use rocket::async_trait;
use rocket::futures::TryStreamExt;
use rocket::serde::DeserializeOwned;
use rocket::tokio::sync::Mutex;
use serde::Serialize;
use std::sync::Arc;
