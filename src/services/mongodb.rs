pub type DB = State<MongoDB>;
pub type AMCollection<T> = Arc<Mutex<Collection<T>>>;
pub type TLightbulbCollection = AMCollection<LightbulbDocument>;
pub type TScheduleCollection = AMCollection<ScheduleDocument>;

/// Datatype for storing collections in MongoDB.
pub struct MongoDB {
    pub lightbulb_storage: LightbulbStorage,
    pub schedule_storage: ScheduleStorage,
}

impl MongoDB {
    pub async fn init() -> Result<Self, mongodb::error::Error> {
        let uri = "mongodb://localhost:27017";
        let mut client_options = ClientOptions::parse(uri).await?;

        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        let client = Client::with_options(client_options)?;

        let db = client.database("somethingsomething");
        let lightbulbs_collection = db.collection::<LightbulbDocument>("lightbulbs");
        let schedule_collection = db.collection::<ScheduleDocument>("schedules");
        let lightbulb_storage = LightbulbStorage::new(Arc::new(Mutex::new(lightbulbs_collection)));
        let schedule_storage = ScheduleStorage::new(Arc::new(Mutex::new(schedule_collection)));

        Ok(Self {
            lightbulb_storage,
            schedule_storage,
        })
    }
}

use std::sync::Arc;

use mongodb::{
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Collection,
};

use crate::models::lightbulb::{storage::LightbulbStorage, LightbulbDocument};
use crate::models::schedule::{storage::ScheduleStorage, ScheduleDocument};
use crate::models::storage::Storage;
use rocket::{tokio::sync::Mutex, State};
