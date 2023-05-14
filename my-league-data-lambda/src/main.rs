mod constants;
mod league_standings;
mod player;
mod result_struct;
mod event_status;
mod live_event_data;
mod manager_picks;

use std::collections::HashMap;
use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::chrono;
use aws_lambda_events::encodings::Body;
use chrono::{Datelike};
use league_standings::{Root};
use result_struct::{Output};
use crate::constants::{MY_FRIEND_LEAGUE_ID, NUMBER_OF_PLAYERS, START_YEAR_OF_MINI_LEAGUE_HISTORY};
use crate::player::WelcomePlayers;
use crate::result_struct::{DetailedSeason, EventHistory, PlayerPositions, Season};
use lambda_runtime::{Error, LambdaEvent};
use serde_json::json;
use crate::event_status::EventStatus;
use crate::live_event_data::{LiveEventData};
use crate::manager_picks::ManagerPicks;

// FOR LOCAL TESTING

// #[tokio::main]
// async fn main() -> Result<(), Error>{
//     let (league_standings, player_history): (Root, HashMap<i64, WelcomePlayers>) = get_league_standings_and_player_history_from_api().await?;
//
//     let output_result = Output {
//         league_standings: get_current_league_standings(&league_standings, &player_history)?,
//         league_history: get_result_seasons(&player_history, &league_standings)?
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
    let body = ureq::get(&format!("https://fantasy.premierleague.com/api/leagues-classic/{}/standings/", MY_FRIEND_LEAGUE_ID))
        .call()?
        .into_string()?;
    let league_standings: Root = serde_json::from_str(&body)?;

    let mut player_history: HashMap<i64, WelcomePlayers> = HashMap::new();
    for player in &league_standings.standings.results {
        let body = ureq::get(&format!("https://fantasy.premierleague.com/api/entry/{}/history/", player.entry)).call()?.into_string()?;
        let player_from_fpl: WelcomePlayers = serde_json::from_str(&body)?;
        player_history.insert(
            player.entry,
            player_from_fpl
        );
    }
    Ok((league_standings, player_history))
}

fn is_league_data_out_of_sync() -> Result<bool, Error>{
    let event_status: EventStatus = serde_json::from_str(&ureq::get("https://fantasy.premierleague.com/api/event-status/").call()?.into_string()?)?;
    if event_status.leagues == "Updated" {
        return Ok(false);
    }
    let today = chrono::Local::now().format("%Y-%m-%d").to_string(); // get today's date in YYYY-MM-DD format
    // AND leagues == "Updating" ??? -> need to check what it is before and after updating
    // leagues = "", "updating", "updated"
    for status in &event_status.status {
        if status.date == today {
            return Ok(true); // return true if today's date is found in the status vector
        }
    }
    Ok(false) // return false if today's date is not found in the status vector
}

fn get_player_current_points(history: &WelcomePlayers, current_gameweek: usize, live_gameweek_data: &Option<LiveEventData>, player_entry: &i64) -> Result<(i64, i64), Error> {
    match live_gameweek_data {
        Some(live_data) => {// get manager's entry's picks
            let manager_picks: ManagerPicks = serde_json::from_str(&ureq::get(&*format!("https://fantasy.premierleague.com/api/entry/{}/event/{}/picks/", player_entry, current_gameweek)).call()?.into_string()?)?;
            let mut sum_of_points: i64 = 0;

            for pick in manager_picks.picks {
                sum_of_points += (&live_data.elements[pick.element as usize - 1].stats.total_points) * pick.multiplier;
            }
            Ok((sum_of_points,history.current[current_gameweek-1].total_points - history.current[current_gameweek-1].points + sum_of_points))
        },
        None => {
            println!("Live gameweek data is None. main.rs, get_player_current_points");
            return Ok((history.current[current_gameweek-1].points, history.current[current_gameweek-1].total_points))
        }
    }
}

fn get_current_league_standings(league_standings: &Root, player_history: &HashMap<i64, WelcomePlayers>) ->  Result<Vec<PlayerPositions>, Error>{
    let mut result: Vec<PlayerPositions> = Vec::new();
    let current_gameweek = player_history[&league_standings.standings.results[0].entry].current.len();

    let mut live_gameweek_data:Option<LiveEventData> = None;
    if is_league_data_out_of_sync()? {
        live_gameweek_data = Some(serde_json::from_str(&ureq::get(&*format!("https://fantasy.premierleague.com/api/event/{}/live/", current_gameweek)).call()?.into_string()?)?);
    }

    for player in &league_standings.standings.results {
        let (event_points, total_points) = get_player_current_points(&player_history[&player.entry], current_gameweek, &live_gameweek_data, &player.entry)?;

        result.push(PlayerPositions {
            event_total: event_points,
            player_name: player.player_name.clone(),
            rank: player.rank,
            last_rank: player.last_rank,
            rank_sort: player.rank_sort,
            total: total_points,
            entry_name: player.entry_name.clone(),
            events: get_current_league_event_history(&player_history[&player.entry], current_gameweek)
        });
    }
    result = sort_by_total_points(result);
    Ok(result)
}

fn get_current_league_event_history(player_history: &WelcomePlayers, current_gameweek: usize) -> Vec<EventHistory> {
    let mut events: Vec<EventHistory> = Vec::new();
    for i in 0.. current_gameweek {
        let rank: i64 = player_history.current[i].rank.unwrap_or_else(|| {
            if i > 1 {
                player_history.current[i - 1].rank.unwrap_or(1)
            } else {
                1
            }
        });

        let overall_rank: i64 = player_history.current[i].overall_rank.unwrap_or_else(|| {
            if i > 1 {
                player_history.current[i - 1].overall_rank.unwrap_or(1)
            } else {
                1
            }
        });
        events.push(
            EventHistory {
                event: player_history.current[i].event,
                points: player_history.current[i].points,
                total_points: player_history.current[i].total_points,
                rank,
                overall_rank,
                rank_percentile: (rank as f64 / NUMBER_OF_PLAYERS as f64) * 100.0,
                overall_rank_percentile: (overall_rank as f64 / NUMBER_OF_PLAYERS as f64) * 100.0,
            }
        );
    }
    events
}

fn get_result_seasons(player_history: &HashMap<i64, WelcomePlayers>, league_standings: &Root) -> Result<Vec<Season>, Error> {
    let mut start =  chrono::Local::now().year() - 2;
    let mut end =  chrono::Local::now().year()%100 - 1;
    let mut result: Vec<Season> = Vec::new();

    while start >= START_YEAR_OF_MINI_LEAGUE_HISTORY {
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
