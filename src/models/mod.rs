use serde::{Deserialize, Serialize};

pub mod lightbulb;
pub mod schedule;
pub mod scheduler;
pub mod storage;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum Active {
    #[serde(rename = "on")]
    On,
    #[default]
    #[serde(rename = "off")]
    Off,
}
