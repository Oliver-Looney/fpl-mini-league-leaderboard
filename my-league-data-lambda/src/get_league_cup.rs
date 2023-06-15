use std::collections::HashMap;
use lambda_runtime::Error;
use crate::league_standings_cup::LeagueStandingsCupAPI;
use crate::result_struct::{CupSeedMatches, CupTeamData, Rounds};

pub async fn get_league_cup_matches(league_cup: &Option<LeagueStandingsCupAPI>) -> Result<Vec<Rounds>, Error> {
    let rounds: Vec<Rounds> = match league_cup {
        Some(league_cup) => {
            let mut round_map: HashMap<String, Rounds> = HashMap::new();

            for cup_match in &league_cup.results {
                let round_title = &cup_match.knockout_name;

                let match1 = CupSeedMatches {
                    id: cup_match.id,
                    date: format!("Gameweek: {}", cup_match.event),
                    winner: cup_match.winner,
                    team1: CupTeamData {
                        team_name: cup_match.entry_1_name.clone(),
                        player_name: cup_match.entry_1_player_name.clone(),
                        points: cup_match.entry_1_points,
                        entry: cup_match.entry_1_entry,
                    },
                    team2: CupTeamData {
                        team_name: cup_match.entry_2_name.clone(),
                        player_name: cup_match.entry_2_player_name.clone(),
                        points: cup_match.entry_2_points,
                        entry: cup_match.entry_2_entry,
                    },
                };

                round_map
                    .entry(round_title.clone())
                    .or_insert_with(|| Rounds {
                        title: round_title.clone(),
                        matches: Vec::new(),
                    })
                    .matches
                    .push(match1);
            }
            round_map.into_values().collect()
        }
        None => Vec::new(),
    };
    Ok(rounds)
}
