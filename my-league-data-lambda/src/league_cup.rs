use lambda_runtime::Error;
use crate::result_struct::CupMatches;

pub async fn get_league_cup_matches() -> Result<Vec<CupMatches>, Error> {
    let mut cup_matches: Vec<CupMatches>= Vec::new();
    Ok(cup_matches)
}
