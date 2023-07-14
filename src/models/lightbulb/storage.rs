pub struct LightbulbStorage {
    collection: TLightbulbCollection,
}

#[async_trait]
impl Storage<LightbulbDocument, TLightbulbCollection> for LightbulbStorage {
    fn new(collection: TLightbulbCollection) -> Self {
        Self { collection }
    }

    fn get_collection(&self) -> TLightbulbCollection {
        self.collection.clone()
    }
}

extern crate rocket;

use super::LightbulbDocument;
use crate::models::storage::Storage;
use crate::services::mongodb::TLightbulbCollection;
use rocket::async_trait;
