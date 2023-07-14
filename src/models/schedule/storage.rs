pub struct ScheduleStorage {
    collection: TScheduleCollection,
}

#[async_trait]
impl Storage<ScheduleDocument, TScheduleCollection> for ScheduleStorage {
    fn new(collection: TScheduleCollection) -> Self {
        Self { collection }
    }

    fn get_collection(&self) -> TScheduleCollection {
        self.collection.clone()
    }
}

impl ScheduleStorage {
    pub async fn get_aggregation(
        &self,
        filter: Option<bson::Document>,
    ) -> Result<Vec<ScheduleAggregation>, Error> {
        let mut pipeline = vec![];
        if let Some(f) = filter {
            pipeline.push(doc! {
                "$match": f
            })
        }
        let stage_lookup_comments = doc! {
           "$lookup": {
              "from": "lightbulbs",
              "localField": "lightbulbs",
              "foreignField": "_id",
              "as": "lightbulbs",
           }
        };
        pipeline.push(stage_lookup_comments);

        let mut cursor = self
            .get_collection()
            .clone()
            .lock()
            .await
            .aggregate(pipeline, None)
            .await?;

        let mut response = vec![];

        while let Some(result) = cursor.next().await {
            response.push(bson::from_document::<ScheduleAggregation>(result?)?);
        }

        Ok(response)
    }
}

use super::{ScheduleAggregation, ScheduleDocument};
use crate::{models::storage::Storage, services::mongodb::TScheduleCollection};
use bson::doc;
use futures::stream::StreamExt;
use mongodb::error::Error;
use rocket::async_trait;
