use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub league_standings: Vec<PlayerPositions>,
    pub league_cup: Vec<CupMatches>,
    pub league_history: Vec<Season>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CupMatches {
    pub id: i64,
    pub entry_1_entry: i64,
    pub entry_1_name: String,
    pub entry_1_player_name: String,
    pub entry_1_points: i64,
    pub entry_2_entry: i64,
    pub entry_2_name: String,
    pub entry_2_player_name: String,
    pub entry_2_points: i64,
    pub is_knockout: bool,
    pub league_id: i64,
    pub winner_entry: i64,
    pub seed_value: Option<i64>,
    pub event_number: i64,
    pub tiebreak: Option<i64>,
    pub is_bye: bool,
    pub knockout_name: String
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
    pub events: Vec<EventHistory>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventHistory {
    pub event: i64,
    pub points: i64,
    pub total_points: i64,
    pub rank: i64,
    pub overall_rank: i64,
    pub rank_percentile: f64,
    pub overall_rank_percentile: f64,
    pub position: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Season {
    pub years: String,
    pub standings: Vec<DetailedSeason>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetailedSeason {
    pub entry_name: String,
    pub player_name: String,
    pub points: i64,
    pub rank: i64,
    pub position: i64
}