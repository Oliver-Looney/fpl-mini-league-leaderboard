use lambda_runtime::Error;
use crate::league_standings_cup::LeagueStandingsCupAPI;
use crate::result_struct::CupMatches;

pub async fn get_league_cup_matches(league_cup: &Option<LeagueStandingsCupAPI>) -> Result<Vec<CupMatches>, Error> {
    if let Some(league_cup) = league_cup {
        Ok(league_cup.results.clone())
    } else {
        Ok(Vec::new())
    }
}
