use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub league_standings: Vec<PlayerPositions>,
    pub league_cup: Vec<Rounds>,
    pub league_history: Vec<Season>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rounds {
    pub title: String,
    pub matches: Vec<CupSeedMatches>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CupSeedMatches {
    pub id: i64,
    pub date: String,
    pub winner: i64,
    pub team1: CupTeamData,
    pub team2: CupTeamData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CupTeamData {
    pub team_name: String,
    pub player_name: String,
    pub points: i64,
    pub entry: i64
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
