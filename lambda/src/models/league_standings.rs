use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Root {
    pub new_entries: NewEntries,
    pub last_updated_data: String,
    pub league: League,
    pub standings: NewEntries,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct League {
    pub id: i64,
    pub name: String,
    pub created: String,
    pub closed: bool,
    pub max_entries: Option<serde_json::Value>,
    pub league_type: String,
    pub scoring: String,
    pub admin_entry: Option<serde_json::Value>,
    pub start_event: i64,
    pub code_privacy: String,
    pub has_cup: bool,
    pub cup_league: Option<serde_json::Value>,
    pub rank: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEntries {
    pub has_next: bool,
    pub page: i64,
    pub results: Vec<Result>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
    pub id: i64,
    pub event_total: i64,
    pub player_name: String,
    pub rank: i64,
    pub last_rank: i64,
    pub rank_sort: i64,
    pub total: i64,
    pub entry: i64,
    pub entry_name: String,
}