use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ManagerPicks {
    pub active_chip: Option<String>,
    pub automatic_subs: Vec<Pick>,
    pub entry_history: EntryHistory,
    pub picks: Vec<Pick>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryHistory {
    pub event: i64,
    pub points: i64,
    pub total_points: i64,
    pub rank: i64,
    pub rank_sort: i64,
    pub overall_rank: i64,
    pub bank: i64,
    pub value: i64,
    pub event_transfers: i64,
    pub event_transfers_cost: i64,
    pub points_on_bench: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pick {
    pub element: i64,
    pub position: i64,
    pub multiplier: i64,
    pub is_captain: bool,
    pub is_vice_captain: bool,
}
