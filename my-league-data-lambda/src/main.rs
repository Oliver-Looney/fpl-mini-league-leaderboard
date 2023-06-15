mod constants;
mod league_standings;
mod player;
mod result_struct;
mod event_status;
mod live_event_data;
mod manager_picks;
mod get_league_cup;
mod get_league_standings;
mod get_league_history;
mod league_standings_cup;

use std::collections::HashMap;
use std::ops::Index;
use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::chrono;
use aws_lambda_events::encodings::Body;
use chrono::Datelike;
use league_standings::Root;
use result_struct::Output;
use crate::constants::{MY_FRIEND_LEAGUE_ID};
use crate::player::WelcomePlayers;
use crate::result_struct::{DetailedSeason, EventHistory, PlayerPositions, Season};
use lambda_runtime::{Error, LambdaEvent};
use serde_json::json;
use ureq::Agent;
use crate::event_status::EventStatus;
use crate::league_standings_cup::LeagueStandingsCupAPI;
use crate::live_event_data::LiveEventData;
use crate::manager_picks::ManagerPicks;

// build cmd:
// cargo lambda build --release --output-format zip

// FOR LOCAL TESTING

// #[tokio::main]
// async fn main() -> Result<(), Error>{
//     let start_time = Instant::now();
//     let ureq_agent = Agent::new();
//     let (league_standings, player_history, event_status, league_cup_data): (Root, HashMap<i64, WelcomePlayers>, EventStatus, Option<LeagueStandingsCupAPI>) = get_league_standings_and_player_history_from_api(&ureq_agent).await?;
//     println!("Elapsed time: {:?}", start_time.elapsed());
//     let output_result = get_output_result(&ureq_agent, &league_standings, &player_history, &event_status, &league_cup_data).await?;
//     println!("{:#?}", output_result);
//     Ok(())
// }


#[tokio::main]
async fn main() -> Result<(), Error>{
    let lambda_handler = lambda_runtime::service_fn(handler);
    lambda_runtime::run(lambda_handler).await?;
    Ok(())
}

async fn handler(_lambda_event: LambdaEvent<ApiGatewayProxyRequest>) -> Result<ApiGatewayProxyResponse, Error> {
    let ureq_agent = Agent::new();
    let (league_standings, player_history, event_status, league_cup_data): (Root, HashMap<i64, WelcomePlayers>, EventStatus, Option<LeagueStandingsCupAPI>) = get_league_standings_and_player_history_from_api(&ureq_agent).await?;

    let output_result = get_output_result(&ureq_agent, &league_standings, &player_history, &event_status, &league_cup_data).await?;
    return Ok(ApiGatewayProxyResponse{
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(
            Body::Text(
                json!(output_result).to_string())),
        is_base64_encoded: Some(false),
    })
}

async fn get_output_result(ureq_agent: &Agent, league_standings: &Root, player_history: &HashMap<i64, WelcomePlayers>, event_status: &EventStatus, league_cup_data: &Option<LeagueStandingsCupAPI>) -> Result<Output, Error> {
    let output_result = Output {
        league_standings: get_league_standings::get_current_league_standings(&league_standings, &player_history, &event_status, &ureq_agent).await?,
        league_history: get_league_history::get_result_seasons(&player_history, &league_standings, &event_status).await?,
        league_cup: get_league_cup::get_league_cup_matches(&league_cup_data).await?
    };
    Ok(output_result)
}

async fn get_league_standings_and_player_history_from_api(ureq_agent: &Agent) -> Result<(Root, HashMap<i64, WelcomePlayers>, EventStatus, Option<LeagueStandingsCupAPI>), Error> {
    let league_standings: Root = serde_json::from_str(&ureq_agent
        .get(&format!("https://fantasy.premierleague.com/api/leagues-classic/{}/standings/", MY_FRIEND_LEAGUE_ID)).call()?.into_string()?)?;

    let mut player_history: HashMap<i64, WelcomePlayers> = HashMap::new();
    for player in &league_standings.standings.results {
        player_history.insert(
            player.entry,
            serde_json::from_str(&ureq_agent
                .get(&format!("https://fantasy.premierleague.com/api/entry/{}/history/", player.entry)).call()?.into_string()?)?
        );
    }
    let event_status: EventStatus = serde_json::from_str(&ureq_agent.get("https://fantasy.premierleague.com/api/event-status/").call()?.into_string()?)?;

    let league_cup: Option<LeagueStandingsCupAPI> =
        if league_standings.league.has_cup && league_standings.league.league_type != "s" && league_standings.league.cup_league.is_some() {
            Some(serde_json::from_str(&ureq_agent
                .get(&format!("https://fantasy.premierleague.com/api/leagues-h2h-matches/league/{}/", league_standings.league.cup_league.as_ref().unwrap())).call()?.into_string()?)?)
        } else {
            None
    };
    Ok((league_standings, player_history, event_status, league_cup))
}
