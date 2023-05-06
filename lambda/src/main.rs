#[path = "models/league_standings.rs"] mod league_standings;
#[path = "models/result_struct.rs"] mod result_struct;
mod constants;

use reqwest::Client;
use league_standings::{Root};
use result_struct::{Output};
use crate::constants::MY_FRIEND_LEAGUE_ID;
use crate::result_struct::PlayerLeaderboardPosition;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let reqwest_client = Client::new();
    let league_standings: Root = reqwest_client.get(format!("https://fantasy.premierleague.com/api/leagues-classic/{}/standings/",MY_FRIEND_LEAGUE_ID)).send().await?.json().await?;
    // println!("{:#?}", league_standings);
    let mut output_result: Output;
    output_result.leaderboard = write_overall_leaderboard();
    // println!("{:#?}", output_result);
    Ok(())
}

fn write_overall_leaderboard() -> Vec<PlayerLeaderboardPosition> {
    return vec![
        PlayerLeaderboardPosition {
            player_name: "Oliver".parse().unwrap(),
            total_wins: 1
        },
        PlayerLeaderboardPosition {
            player_name: "Daniel".parse().unwrap(),
            total_wins: 1
        },
        PlayerLeaderboardPosition {
            player_name: "Declan".parse().unwrap(),
            total_wins: 0
        },
        PlayerLeaderboardPosition {
            player_name: "Thomas".parse().unwrap(),
            total_wins: 0
        },
    ];
}


