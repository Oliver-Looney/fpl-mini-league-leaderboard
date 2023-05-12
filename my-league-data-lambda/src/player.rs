use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct WelcomePlayers {
    pub current: Vec<Current>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Current {
    pub event: i64,
    pub points: i64,
    pub total_points: i64,
    pub rank: i64,
    pub overall_rank: i64,
    pub bank: i64,
    pub value: i64,
    pub event_transfers: i64,
    pub event_transfers_cost: i64,
    pub points_on_bench: i64
}