use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub league_standings: Vec<PlayerPositions>,
    // pub league_history: Vec<Season>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerPositions {
    pub event_total: i64,
    pub player_name: String,
    pub rank: i64,
    pub last_rank: i64,
    pub rank_sort: i64,
    pub total: i64,
    pub entry_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Season {
    pub years: String,
    pub standings: Vec<DetailedSeason>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetailedSeason {
    pub entry_name: String,
    pub points: i64,
    pub rank: i64,
    pub position: i64
}