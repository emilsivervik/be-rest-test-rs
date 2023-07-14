pub struct LightbulbAPI {}

impl LightbulbAPI {
    pub async fn post(
        storage: &LightbulbStorage,
        create_data: LightbulbJSON,
    ) -> Result<LightbulbJSON, Error> {
        let inserted = storage
            .create(&LightbulbDocument::from(create_data))
            .await?;
        // Todo(Remove unwrwap)
        let storage_document = storage
            .get(&inserted.inserted_id.as_object_id().unwrap())
            .await?
            .unwrap();
        Ok(LightbulbJSON::from(storage_document))
    }

    pub async fn get_all(storage: &LightbulbStorage) -> Result<Vec<LightbulbJSON>, Error> {
        let storage_documents = storage.list().await?;

        let list_data = storage_documents.into_iter().map(LightbulbJSON::from);

        Ok(Vec::from_iter(list_data))
    }

    pub async fn get(
        storage: &LightbulbStorage,
        oid: ObjectId,
    ) -> Result<Option<LightbulbJSON>, Error> {
        let storage_document = storage.get(&oid).await?;

        if let Some(document) = storage_document {
            return Ok(Some(LightbulbJSON::from(document)));
        }
        Ok(None)
    }

    pub async fn put(
        storage: &LightbulbStorage,
        oid: ObjectId,
        update_data: LightbulbJSON,
    ) -> Result<Option<LightbulbJSON>, Error> {
        let update_db_data =
            bson::to_document::<LightbulbDocument>(&LightbulbDocument::from(update_data))?;
        storage.update(&oid, &update_db_data).await?;

        let updated_document = storage.get(&oid).await?;

        Ok(updated_document.map(LightbulbJSON::from))
    }

    pub async fn del(storage: &LightbulbStorage, oid: ObjectId) -> Result<bool, Error> {
        storage.remove(&oid).await
    }
}

extern crate rocket;
use super::storage::LightbulbStorage;
use super::{LightbulbDocument, LightbulbJSON};
use crate::models::storage::Storage;
use bson::oid::ObjectId;
use mongodb::error::Error;
