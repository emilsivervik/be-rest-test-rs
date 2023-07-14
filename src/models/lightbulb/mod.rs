pub mod api;
pub mod storage;

/// Datatype for how Lightbulb is stored in MongoDB.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LightbulbDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(default)]
    pub active: Active,
    // brightness: i32,
    // color: Colorcode?
}

/// Datatype for how Lightbulb is exposed through the API.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LightbulbJSON {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub active: Active,
    // brightness: i32,
    // color: Colorcode?
}

impl From<LightbulbDocument> for LightbulbJSON {
    fn from(lightbulb: LightbulbDocument) -> Self {
        Self {
            active: lightbulb.active,
            id: lightbulb.id.map(|v| v.to_string()),
        }
    }
}

impl From<LightbulbJSON> for LightbulbDocument {
    fn from(lightbulb: LightbulbJSON) -> Self {
        Self {
            active: lightbulb.active,
            id: ObjectId::parse_str(lightbulb.id.unwrap_or_default()).ok(),
        }
    }
}

use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::Active;
