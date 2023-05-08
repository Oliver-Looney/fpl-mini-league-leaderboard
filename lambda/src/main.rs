#[path = "models/league_standings.rs"] mod league_standings;
#[path = "models/event_status.rs"] mod event_status;
#[path = "models/player.rs"] mod player;
#[path = "models/result_struct.rs"] mod result_struct;
mod constants;

use std::collections::HashMap;
use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;
use chrono::{Datelike, Local};
use reqwest::Client;
use league_standings::{Root};
use result_struct::{Output};
use crate::constants::MY_FRIEND_LEAGUE_ID;
use crate::player::WelcomePlayers;
use crate::result_struct::{DetailedSeason, PlayerPositions, Season};
use lambda_runtime::{Error, LambdaEvent};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error>{
    let lambda_handler = lambda_runtime::service_fn(handler);
    lambda_runtime::run(lambda_handler).await?;
    Ok(())
}

async fn handler(_lambda_event: LambdaEvent<ApiGatewayProxyRequest>) -> Result<ApiGatewayProxyResponse, Error> {
    let (league_standings, player_history): (Root, HashMap<i64, WelcomePlayers>) = get_league_standings_and_player_history_from_api().await?;

    let output_result = Output {
        league_standings: get_current_league_standings(&league_standings, &player_history)?,
        league_history: get_result_seasons(&player_history, &league_standings)?
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

async fn get_league_standings_and_player_history_from_api() -> Result<(Root, HashMap<i64, WelcomePlayers>), Error> {
    let reqwest_client = Client::new();
    let league_standings: Root = reqwest_client.get(format!("https://fantasy.premierleague.com/api/leagues-classic/{}/standings/", MY_FRIEND_LEAGUE_ID)).send().await?.json().await?;

    let mut player_history: HashMap<i64, WelcomePlayers> = HashMap::new();
    for player in &league_standings.standings.results {
        player_history.insert(player.entry, reqwest_client.get(format!("https://fantasy.premierleague.com/api/entry/{}/history/", player.entry)).send().await?.json().await?);
    }
    Ok((league_standings, player_history))
}

fn get_current_league_standings(league_standings: &Root, player_history: &HashMap<i64, WelcomePlayers>) ->  Result<Vec<PlayerPositions>, Error>{
    let mut result: Vec<PlayerPositions> = Vec::new();
    for player in &league_standings.standings.results {
        let history: &WelcomePlayers = &player_history[&player.entry];
        let current_gameweek = history.current.len()-1;
        result.push(PlayerPositions {
            event_total: history.current[current_gameweek]["points"].unwrap(),
            player_name: player.player_name.clone(),
            rank: player.rank,
            last_rank: player.last_rank,
            rank_sort: player.rank_sort,
            total:  history.current[current_gameweek]["total_points"].unwrap(),
            entry_name: player.entry_name.clone(),
        });
    }
    result = sort_by_total_points(result);
    Ok(result)
}

fn get_result_seasons(player_history: &HashMap<i64, WelcomePlayers>, league_standings: &Root) -> Result<Vec<Season>, Error> {
    let mut start =  Local::now().year() - 2;
    let mut end =  Local::now().year()%100 - 1;
    let mut result: Vec<Season> = Vec::new();

    while start >= 2020 {
        let fpl_year = format!("{}/{}",start,end);
        start -=1;
        end -=1;

        let mut new_season: Season = Season {
            years: fpl_year.clone(),
            standings: get_past_season_standings(&fpl_year, player_history, league_standings)
        };
        sort_seasons_by_points(&mut new_season.standings);

        result.push(new_season);
    }
    Ok(result)
}

fn get_past_season_standings(years: &String, player_history: &HashMap<i64, WelcomePlayers>, league_standings: &Root) -> Vec<DetailedSeason> {
    let mut standings : Vec<DetailedSeason> = Vec::new();

    for player in &league_standings.standings.results {
        let mut player_result = DetailedSeason {
            entry_name: player.entry_name.clone(),
            player_name: player.player_name.clone(),
            points: 0,
            rank: 0,
            position: 0
        };

        for players_past_season in &player_history[&player.entry].past {
            if players_past_season.season_name != years.as_str() {continue}
            player_result.points = players_past_season.total_points;
            player_result.rank = players_past_season.rank;
            standings.push(player_result);
            break;
        }
    }
    standings
}

fn sort_by_total_points(mut league_standings: Vec<PlayerPositions>) -> Vec<PlayerPositions> {
    league_standings.sort_by_key(|player| std::cmp::Reverse(player.total));
    let mut rank = 1;
    let mut last_total = league_standings[0].total;
    for player in league_standings.iter_mut() {
        if player.total < last_total {
            rank += 1;
        }
        player.rank = rank;
        player.last_rank = player.rank;
        last_total = player.total;
    }
    league_standings
}

fn sort_seasons_by_points(standings: &mut Vec<DetailedSeason>) {
    standings.sort_by(|a, b| b.points.cmp(&a.points));
    for (i, season) in standings.iter_mut().enumerate() {
        season.position = i as i64 + 1;
    }
}
