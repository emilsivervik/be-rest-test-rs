#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum WeekDays {
    #[serde(rename = "monday")]
    Monday,
    #[serde(rename = "tuesday")]
    Tuesday,
    #[serde(rename = "wednesdag")]
    Wednesday,
    #[serde(rename = "thursday")]
    Thursday,
    #[serde(rename = "friday")]
    Friday,
    #[serde(rename = "saturday")]
    Saturday,
    #[serde(rename = "sunday")]
    Sunday,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Repeating {
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Time {
    hour: u8,
    minute: u8,
}

/// Shared datatype for Schedule entitys.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SharedFields {
    repeating: Repeating,
    week_days: Option<WeekDays>,
    #[serde(default)]
    active: Active,
}

/// Datatype for how Schedule is stored in MongoDB.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[serde(default = "Vec::default")]
    pub lightbulbs: Vec<ObjectId>,

    #[serde(flatten)]
    shared_fields: SharedFields,

    on_time: bson::DateTime,
    off_time: bson::DateTime,
    // brightness: i32,
    // color: Colorcode?
}

/// Datatype for how Schedule is exposed through the API.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleJSON {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(default = "Vec::default")]
    lightbulbs: Vec<LightbulbJSON>,

    #[serde(flatten)]
    shared_fields: SharedFields,

    on_time: Time,
    off_time: Time,
}

/// Datatype for how Schedule is exported using MongoDB Aggregation.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleAggregation {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,

    #[serde(default = "Vec::default")]
    lightbulbs: Vec<LightbulbDocument>,

    #[serde(flatten)]
    shared_fields: SharedFields,

    on_time: DateTime,
    off_time: DateTime,
}

impl From<DateTime> for Time {
    fn from(datetime: DateTime) -> Time {
        let chrono_time = chrono::DateTime::from(datetime);

        Time {
            hour: chrono_time.hour() as u8,
            minute: chrono_time.minute() as u8,
        }
    }
}

impl From<crate::models::schedule::Time> for DateTime {
    fn from(time: crate::models::schedule::Time) -> DateTime {
        time_to_bson(time.hour, time.minute, 0)
    }
}

pub fn time_to_bson(hour: u8, minute: u8, second: u8) -> DateTime {
    DateTime::builder()
        .year(1970)
        .month(1)
        .day(1)
        .hour(hour)
        .minute(minute)
        .second(second)
        .build()
        .unwrap()
}

impl From<ScheduleJSON> for ScheduleDocument {
    fn from(schedule: ScheduleJSON) -> ScheduleDocument {
        let mut lightbulbs = vec![];
        for v in schedule.lightbulbs {
            if let Some(id) = v.id {
                if let Ok(oid) = ObjectId::parse_str(id) {
                    lightbulbs.push(oid)
                }
            }
        }
        let shared = SharedFields {
            active: schedule.shared_fields.active,
            repeating: schedule.shared_fields.repeating,
            week_days: schedule.shared_fields.week_days,
        };

        Self {
            id: ObjectId::parse_str(schedule.id.unwrap_or_default()).ok(),
            shared_fields: shared,
            off_time: DateTime::from(schedule.off_time),
            on_time: DateTime::from(schedule.on_time),
            lightbulbs,
        }
    }
}

impl From<ScheduleAggregation> for ScheduleJSON {
    fn from(schedule: ScheduleAggregation) -> ScheduleJSON {
        let shared = SharedFields {
            active: schedule.shared_fields.active,
            repeating: schedule.shared_fields.repeating,
            week_days: schedule.shared_fields.week_days,
        };

        let mut lightbulbs = vec![];
        for lightbulb in schedule.lightbulbs {
            lightbulbs.push(LightbulbJSON::from(lightbulb))
        }

        Self {
            id: schedule.id.map(|v| v.to_string()),
            shared_fields: shared,
            off_time: Time::from(schedule.off_time),
            on_time: Time::from(schedule.on_time),
            lightbulbs,
        }
    }
}

use bson::{oid::ObjectId, DateTime};
use chrono::Timelike;
use serde::{Deserialize, Serialize};

use super::{
    lightbulb::{LightbulbDocument, LightbulbJSON},
    Active,
};

pub mod api;
pub mod storage;
