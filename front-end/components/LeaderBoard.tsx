import { FC } from 'react';
import { FplMiniLeagueAPIResponse, Standing } from "@/utils/types";

interface Props {
    data: FplMiniLeagueAPIResponse;
}

const LeaderBoard: FC<Props> = ({ data }) => {
    const playerWins: Record<string, number> = {};

    // Loop through all the standings in league history
    data.league_history.forEach((history) => {
        history.standings.forEach((standing: Standing) => {
            if (standing.position === 1) {
                const playerName = standing.player_name;
                // Increase the win count for the player
                playerWins[playerName] = (playerWins[playerName] || 0) + 1;
            }
        });
    });

    // Loop through current standings to add new players without any win to playerWins object
    data.league_standings.forEach((standing) => {
        if (!(standing.player_name in playerWins)) {
            playerWins[standing.player_name] = 0;
        }
    });

    // Sort the players by number of wins in descending order
    const sortedPlayers = Object.entries(playerWins).sort((a, b) => b[1] - a[1]);

    return (
        <div className="card">
            <h2>Leaderboard</h2>
            {sortedPlayers.map(([player, wins], index) => (
                <div key={index}>
                    {player.split(' ')[0]}: {wins}
                </div>
            ))}
        </div>
    );
};

export default LeaderBoard;
