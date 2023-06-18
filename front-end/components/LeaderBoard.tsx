import { FC } from 'react';
import { FplMiniLeagueAPIResponse, Standing } from "@/utils/types";
import {prev_data} from "@/utils/prev_season_data";

interface Props {
    data: FplMiniLeagueAPIResponse;
}

const LeaderBoard: FC<Props> = ({ data }) => {
    const playerStats: Record<string, { leagues: number; cups: number }> = {};

    // Loop through all the standings in league history
    data.league_history.forEach((history) => {
        history.standings.forEach((standing: Standing) => {
            // Increase the leagues count for the player
            if (!playerStats[standing.player_name]) {
                playerStats[standing.player_name] = { leagues: 0, cups: 0 };
            }
            if (standing.position == 1) {
                playerStats[standing.player_name].leagues++;
            }
        });

        // Check if cup data exists for the year
        if (prev_data[history.years] && prev_data[history.years].cup) {
            const winner_entry = prev_data[history.years].cup[prev_data[history.years].cup.length - 1].seeds[0].winner;
            const winner_name: string = prev_data[history.years].cup[prev_data[history.years].cup.length - 1].seeds[0].teams[0].entry == winner_entry ?
                prev_data[history.years].cup[prev_data[history.years].cup.length - 1].seeds[0].teams[0].player_name :
                prev_data[history.years].cup[prev_data[history.years].cup.length - 1].seeds[0].teams[1].player_name;
            // Increase the cups count for the cup winner
            if (!playerStats[winner_name]) {
                playerStats[winner_name] = { leagues: 0, cups: 0 };
            }
            playerStats[winner_name].cups++;
        }
    });

    // Sort the players by number of leagues won in descending order
    const sortedPlayers = Object.entries(playerStats).sort((a, b) => b[1].leagues - a[1].leagues);

    return (
        <div className="card">
            <h2>Leaderboard</h2>
            <table>
                <thead>
                <tr>
                    <th>Manager</th>
                    <th>Leagues</th>
                    <th>Cups</th>
                </tr>
                </thead>
                <tbody>
                {sortedPlayers.map(([player, stats], index) => (
                    <tr key={index}>
                        <td>{player.split(' ')[0]}</td>
                        <td>
                            <div>{stats.leagues}</div>
                            <div>{'🏆'.repeat(stats.leagues) || '---'}</div>
                        </td>
                        <td>
                            <div>{stats.cups}</div>
                            <div>{'🏆 '.repeat(stats.cups) || '---'}</div>
                        </td>
                    </tr>
                ))}
                </tbody>
            </table>
        </div>
    );
};

export default LeaderBoard;
