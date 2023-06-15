use serde_json::Value;
use serde::{Deserialize, Serialize};
use crate::result_struct::CupMatches;

#[derive(Debug, Serialize, Deserialize)]
pub struct LeagueStandingsCupAPI {
    pub has_next: bool,
    pub page: i64,
    pub results: Vec<CupMatches>,
}
