mod constants;
mod league_standings;
mod player;
mod result_struct;
mod event_status;
mod live_event_data;
mod manager_picks;
mod league_cup;
mod get_league_standings;
mod get_league_history;

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
use crate::result_struct::{CupMatches, DetailedSeason, EventHistory, PlayerPositions, Season};
use lambda_runtime::{Error, LambdaEvent};
use serde_json::json;
use ureq::Agent;
use crate::event_status::EventStatus;
use crate::live_event_data::LiveEventData;
use crate::manager_picks::ManagerPicks;

// build cmd:
// cargo lambda build --release --output-format zip

// FOR LOCAL TESTING

// #[tokio::main]
// async fn main() -> Result<(), Error>{
//     let ureq_agent = Agent::new();
//     let (league_standings, player_history, event_status): (Root, HashMap<i64, WelcomePlayers>, EventStatus) = get_league_standings_and_player_history_from_api(&ureq_agent).await?;
//     let output_result = Output {
//         league_standings: get_league_standings::get_current_league_standings(&league_standings, &player_history, &event_status, &ureq_agent).await?,
//         league_history: get_league_history::get_result_seasons(&player_history, &league_standings, &event_status).await?,
//         league_cup: league_cup::get_league_cup_matches().await?
//     };
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
    let (league_standings, player_history, event_status): (Root, HashMap<i64, WelcomePlayers>, EventStatus) = get_league_standings_and_player_history_from_api(&ureq_agent).await?;

    let output_result = Output {
        league_standings: get_league_standings::get_current_league_standings(&league_standings, &player_history, &event_status, &ureq_agent).await?,
        league_history: get_league_history::get_result_seasons(&player_history, &league_standings, &event_status).await?,
        league_cup: league_cup::get_league_cup_matches().await?
    };
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

async fn get_league_standings_and_player_history_from_api(ureq_agent: &Agent) -> Result<(Root, HashMap<i64, WelcomePlayers>, EventStatus), Error> {
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
    Ok((league_standings, player_history, event_status))
}
