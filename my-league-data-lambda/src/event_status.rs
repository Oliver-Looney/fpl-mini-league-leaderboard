use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EventStatus {
    pub status: Vec<Status>,
    pub leagues: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub bonus_added: bool,
    pub date: String,
    pub event: i64,
    pub points: String,
}
