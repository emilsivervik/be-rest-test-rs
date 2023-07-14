/// Entity that's responsible for handling cycling logic for turning lights on and off according to specified schedules.
pub struct Scheduler {
    lightbulb_storage: LightbulbStorage,
    schedule_storage: ScheduleStorage,
}

impl Scheduler {
    pub fn new(db: &MongoDB) -> Self {
        Self {
            lightbulb_storage: LightbulbStorage::new(db.lightbulb_storage.get_collection()),
            schedule_storage: ScheduleStorage::new(db.schedule_storage.get_collection()),
        }
    }
    pub fn init(db: &MongoDB) {
        let scheduler = Scheduler::new(db.to_owned());
        rocket::tokio::spawn(async move {
            loop {
                scheduler.do_cycle().await;
                rocket::tokio::time::sleep(std::time::Duration::from_secs(10_u64)).await;
            }
        });
    }

    async fn do_cycle(&self) {
        let local_time = Local::now();
        self.turn_on_lights(&local_time).await.unwrap();
        self.turn_off_lights(&local_time).await.unwrap();
    }

    async fn turn_on_lights(&self, local_time: &DateTime<Local>) -> Result<(), Error> {
        let bson_time = time_to_bson(
            local_time.hour() as u8,
            local_time.minute() as u8,
            local_time.second() as u8,
        );

        let filter = doc! {
            "onTime": { "$lte": &bson_time },
            "offTime": { "$gte": &bson_time },
            "active": "off"
        };

        let mut cursor = self.schedule_storage.find(Some(filter)).await?;

        while let Some(result_doc) = cursor.next().await {
            if let Ok(current_doc) = result_doc {
                let oid = current_doc.id.unwrap();
                let lightbulbs = current_doc.lightbulbs;
                let db_result = self
                    .schedule_storage
                    .update(&oid, &doc! { "active": "on"})
                    .await;
                for lightbulb in lightbulbs {
                    let db_result = self
                        .lightbulb_storage
                        .update(&lightbulb, &doc! { "active": "on"})
                        .await;
                    if let Err(err) = db_result {
                        println!("Could not set schedule {:?} due to {}", oid, err);
                    } else {
                        println!("Set lightbulb {:?} as ON", oid);
                    }
                }
                if let Err(err) = db_result {
                    println!("Could not set schedule {:?} due to {}", oid, err);
                } else {
                    println!("Set schedule {:?} as ON", oid);
                }
            }
        }

        Ok(())
    }

    async fn turn_off_lights(&self, local_time: &DateTime<Local>) -> Result<(), Error> {
        let bson_time = time_to_bson(
            local_time.hour() as u8,
            local_time.minute() as u8,
            local_time.second() as u8,
        );

        let filter = doc! {
            "$or": [
                doc! { "onTime": { "$gte": &bson_time }, "offTime": { "$gte": &bson_time }, "active": "on" },
                doc! { "onTime": { "$lte": &bson_time }, "offTime": { "$lte": &bson_time }, "active": "on" },
            ]

        };

        println!("{}", filter);

        let mut cursor = self.schedule_storage.find(Some(filter)).await?;

        while let Some(result_doc) = cursor.next().await {
            if let Ok(current_doc) = result_doc {
                let oid = current_doc.id.unwrap();
                let lightbulbs = current_doc.lightbulbs;
                let db_result = self
                    .schedule_storage
                    .update(&oid, &doc! { "active": "off"})
                    .await;
                for lightbulb in lightbulbs {
                    let db_result = self
                        .lightbulb_storage
                        .update(&lightbulb, &doc! { "active": "off"})
                        .await;
                    if let Err(err) = db_result {
                        println!("Could not set schedule {:?} due to {}", oid, err);
                    } else {
                        println!("Set lightbulb {:?} as OFF", oid);
                    }
                }
                if let Err(err) = db_result {
                    println!("Could not set schedule {:?} due to {}", oid, err);
                } else {
                    println!("Set schedule {:?} as OFF", oid);
                }
            }
        }

        Ok(())
    }
}

use crate::models::schedule::time_to_bson;
use crate::models::{
    lightbulb::storage::LightbulbStorage, schedule::storage::ScheduleStorage, storage::Storage,
};
use crate::services::mongodb::MongoDB;
use bson::doc;
use chrono::{DateTime, Local, Timelike};
use futures::stream::StreamExt;
use mongodb::error::Error;

// active: 'off',
// onTime: ISODate("1970-01-01T08:41:00.000Z"),
// offTime: ISODate("1970-01-01T08:42:00.000Z")
