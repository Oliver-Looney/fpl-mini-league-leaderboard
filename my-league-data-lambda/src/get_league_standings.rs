use std::collections::HashMap;
use lambda_runtime::Error;
use ureq::Agent;
use crate::constants::NUMBER_OF_PLAYERS;
use crate::event_status::EventStatus;
use crate::league_standings::Root;
use crate::live_event_data::LiveEventData;
use crate::manager_picks::ManagerPicks;
use crate::player::WelcomePlayers;
use crate::result_struct::{EventHistory, PlayerPositions};

pub async fn get_current_league_standings(league_standings: &Root, player_history: &HashMap<i64, WelcomePlayers>, event_status: &EventStatus, ureq_agent: &Agent) ->  Result<Vec<PlayerPositions>, Error>{
    let mut result: Vec<PlayerPositions> = Vec::new();
    let current_gameweek = player_history[&league_standings.standings.results[0].entry].current.len();

    let mut live_gameweek_data:Option<LiveEventData> = None;
    if is_league_data_out_of_sync(event_status)? {
        live_gameweek_data = Some(serde_json::from_str(&ureq_agent.get(&*format!("https://fantasy.premierleague.com/api/event/{}/live/", current_gameweek)).call()?.into_string()?)?);
    }

    for player in &league_standings.standings.results {
        let (event_points, total_points) = get_player_current_points(&player_history[&player.entry], current_gameweek, &live_gameweek_data, &player.entry, ureq_agent)?;

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
    result = set_positions_in_mini_league_for_each_event(result, current_gameweek)?;

    result.iter_mut().for_each(|player_result| {
        let last_event_index = player_result.events.len() - 1;
        player_result.events[last_event_index].points = player_result.event_total;
        player_result.events[last_event_index].total_points = player_result.total;
    });
    Ok(result)
}

fn is_league_data_out_of_sync(event_status: &EventStatus) -> Result<bool, Error>{
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
                position: 0,
                event_transfers: player_history.current[i].event_transfers,
                points_on_bench: player_history.current[i].points_on_bench,
                value: player_history.current[i].value as f64 / 10 as f64,
                // to do idea, get data from this endpoint:
                // https://fantasy.premierleague.com/api/entry/563443/event/2/picks/
            }
        );
    }
    events
}

fn set_positions_in_mini_league_for_each_event(mut result: Vec<PlayerPositions>, current_gameweek: usize) -> Result<Vec<PlayerPositions>, Error> {
    let mut event_index: usize = 0;
    while event_index < current_gameweek {
        let mut points = Vec::new();
        result.iter().for_each(|player_result| {
            points.push(player_result.events[event_index].total_points);
        });
        points.sort_by(|a, b| b.cmp(a));
        for player_result in &mut result {
            let event = &mut player_result.events[event_index];
            let rank = points.iter().position(|&x| x == event.total_points)
                            .map(|i| i + 1).unwrap_or(0); // Get the index of the player's total points and add 1 to get the rank
            event.position = rank as i64;
        }
        event_index += 1;
    }
    Ok(result)
}

fn get_player_current_points(history: &WelcomePlayers, current_gameweek: usize, live_gameweek_data: &Option<LiveEventData>, player_entry: &i64, ureq_agent: &Agent) -> Result<(i64, i64), Error> {
    match live_gameweek_data {
        Some(live_data) => {// get manager's entry's picks
            let manager_picks: ManagerPicks = serde_json::from_str(&ureq_agent.get(&*format!("https://fantasy.premierleague.com/api/entry/{}/event/{}/picks/", player_entry, current_gameweek)).call()?.into_string()?)?;
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
