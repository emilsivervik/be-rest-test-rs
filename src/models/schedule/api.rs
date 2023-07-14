pub struct ScheduleAPI {}

impl ScheduleAPI {
    pub async fn post(
        storage: &ScheduleStorage,
        create_data: ScheduleJSON,
    ) -> Result<ScheduleJSON, Error> {
        let storage_document = storage.create(&ScheduleDocument::from(create_data)).await?;
        let mut aggregated_storage = storage
            .get_aggregation(Some(doc! {"_id": storage_document.inserted_id }))
            .await?;

        if aggregated_storage.is_empty() {}

        Ok(ScheduleJSON::from(aggregated_storage.swap_remove(0)))
    }
    pub async fn get_all(storage: &ScheduleStorage) -> Result<Vec<ScheduleJSON>, Error> {
        let documents = storage
            .get_aggregation(None)
            .await?
            .into_iter()
            .map(ScheduleJSON::from)
            .collect::<Vec<ScheduleJSON>>();
        Ok(documents)
    }

    pub async fn get(
        storage: &ScheduleStorage,
        oid: ObjectId,
    ) -> Result<Option<ScheduleJSON>, Error> {
        let mut documents = storage.get_aggregation(Some(doc! {"_id": oid })).await?;
        if documents.is_empty() {
            return Ok(None);
        }
        Ok(Some(ScheduleJSON::from(documents.remove(0))))
    }

    pub async fn del(storage: &ScheduleStorage, oid: ObjectId) -> Result<bool, Error> {
        storage.remove(&oid).await
    }

    pub async fn put(
        storage: &ScheduleStorage,
        oid: ObjectId,
        update_value: ScheduleJSON,
    ) -> Result<Option<ScheduleJSON>, Error> {
        let update_data =
            bson::to_document::<ScheduleDocument>(&ScheduleDocument::from(update_value))?;
        storage.update(&oid, &update_data).await?;

        let mut documents = storage.get_aggregation(Some(doc! {"_id": oid })).await?;
        if documents.is_empty() {
            return Ok(None);
        }
        Ok(Some(ScheduleJSON::from(documents.remove(0))))
    }
}

extern crate rocket;
use super::{storage::ScheduleStorage, ScheduleDocument, ScheduleJSON};
use crate::models::storage::Storage;
use bson::{doc, oid::ObjectId};
use mongodb::error::Error;
