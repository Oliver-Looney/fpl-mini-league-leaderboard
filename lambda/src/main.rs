#[path = "models/league_standings.rs"] mod league_standings;
#[path = "models/event_status.rs"] mod event_status;
#[path = "models/player.rs"] mod player;
#[path = "models/result_struct.rs"] mod result_struct;
mod constants;

use std::collections::HashMap;
use chrono::{Datelike, Local};
use reqwest::Client;
use league_standings::{Root};
use result_struct::{Output};
use crate::constants::MY_FRIEND_LEAGUE_ID;
use crate::player::WelcomePlayers;
use crate::result_struct::{DetailedSeason, PlayerPositions, Season};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let reqwest_client = Client::new();
    let league_standings: Root = reqwest_client.get(format!("https://fantasy.premierleague.com/api/leagues-classic/{}/standings/", MY_FRIEND_LEAGUE_ID)).send().await?.json().await?;

    let mut player_history: HashMap<i64, WelcomePlayers>= HashMap::new();
    for player in &league_standings.standings.results {
        player_history.insert(player.entry, reqwest_client.get(format!("https://fantasy.premierleague.com/api/entry/{}/history/", player.entry)).send().await?.json().await?);
    }

    for player in &league_standings.standings.results {
        println!("{}", player.player_name);
        println!("{:#?}", player_history[&player.entry].past);
    }

    let league_history = get_result_seasons(&player_history, &league_standings);
    let league_standings = get_current_league_standings(league_standings, &player_history)?;

    let mut output_result = Output {
        league_standings,
        league_history
    };
    println!("{:#?}\n\n\n", output_result);
    Ok(())
}

fn get_current_league_standings(league_standings: Root, player_history: &HashMap<i64, WelcomePlayers>) ->  Result<Vec<PlayerPositions>, Box<dyn std::error::Error>>{
    let mut result: Vec<PlayerPositions> = Vec::new();
    for player in league_standings.standings.results {
        let history: &WelcomePlayers = &player_history[&player.entry];
        let current_gameweek = history.current.len()-1;
        result.push(PlayerPositions {
            event_total: history.current[current_gameweek]["points"].unwrap(),
            player_name: player.player_name,
            rank: player.rank,
            last_rank: player.last_rank,
            rank_sort: player.rank_sort,
            total:  history.current[current_gameweek]["total_points"].unwrap(),
            entry_name: player.entry_name,
        });
    }
    result = sort_by_total_points(result);
    Ok(result)
}

fn get_result_seasons(player_history: &HashMap<i64, WelcomePlayers>, league_standings: &Root) -> Vec<Season> {
    let curr_year_digits = Local::now().year()%100;
    let mut start = curr_year_digits - 2;
    let mut end = curr_year_digits - 1;
    let mut result: Vec<Season> = Vec::new();

    while start >= 20 {
        let fpl_year = format!("{}/{}",start,end);
        start -=1;
        end -=1;

        let mut new_season: Season = Season {
            years: fpl_year.clone(),
            standings: get_past_season_standings(&fpl_year, player_history, league_standings)
        };

        result.push(new_season);
    }
    println!("{:#?}",result);
    result
}

fn get_past_season_standings(years: &String, player_history: &HashMap<i64, WelcomePlayers>, league_standings: &Root) -> Vec<DetailedSeason> {
    let mut standings : Vec<DetailedSeason> = Vec::new();

    for player in &league_standings.standings.results {
        let player_result = DetailedSeason {
            entry_name: player.entry_name.clone(),
            player_name: player.player_name.clone(),
            points: 0,
            rank: 0,
            position: 0
        };

        standings.push(player_result);

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
