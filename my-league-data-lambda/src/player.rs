use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct WelcomePlayers {
    pub current: Vec<HashMap<String, Option<i64>>>,
    pub past: Vec<Past>,
    pub chips: Vec<Chip>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chip {
    pub name: String,
    pub time: String,
    pub event: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Past {
    pub season_name: String,
    pub total_points: i64,
    pub rank: i64,
}