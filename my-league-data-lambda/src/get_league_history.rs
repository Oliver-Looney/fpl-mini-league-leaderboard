use std::collections::HashMap;
use lambda_runtime::Error;
use chrono::Datelike;
use crate::constants::START_YEAR_OF_MINI_LEAGUE_HISTORY;
use crate::event_status::EventStatus;
use crate::league_standings::Root;
use crate::player::WelcomePlayers;
use crate::result_struct::{DetailedSeason, Season};

pub async fn get_result_seasons(player_history: &HashMap<i64, WelcomePlayers>, league_standings: &Root, event_status: &EventStatus) -> Result<Vec<Season>, Error> {
    let mut start =  chrono::Local::now().year() - 2;
    let mut end =  chrono::Local::now().year()%100 - 1;
    let mut result: Vec<Season> = Vec::new();

    add_just_finished_season_results_if_needed(&mut result, &(start+1), &(end+1), &player_history, league_standings, event_status);

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

fn add_just_finished_season_results_if_needed(result: &mut Vec<Season>, first_year: &i32, second_year: &i32, player_history: &HashMap<i64, WelcomePlayers>, league_standings: &Root, event_status: &EventStatus) {
    let current_gameweek = player_history[&league_standings.standings.results[0].entry].current.len();

    if current_gameweek == 38 && event_status.leagues == "Updated" {
        let mut new_season: Season = Season {
            years: format!("{}/{}",first_year, second_year),
            standings: Vec::new()
        };
        for player in &league_standings.standings.results {
            new_season.standings.push( DetailedSeason {
                player_name: player.player_name.clone(),
                rank: player.rank,
                points: player.total,
                entry_name: player.entry_name.clone(),
                position: 0,
            });
        }

        sort_seasons_by_points(&mut new_season.standings);
        result.push(new_season);
    }
}

fn sort_seasons_by_points(standings: &mut Vec<DetailedSeason>) {
    standings.sort_by(|a, b| b.points.cmp(&a.points));
    for (i, season) in standings.iter_mut().enumerate() {
        season.position = i as i64 + 1;
    }
}
