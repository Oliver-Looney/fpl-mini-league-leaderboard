import {FplMiniLeagueAPIResponse} from "@/utils/types";
import React from "react";

interface Props {
    data: FplMiniLeagueAPIResponse;
}

const CurrentLeagueTable: React.FC<Props> = ({ data }) => {
    const currentStandings = data.league_standings;

    return (
        <table>
            <thead>
            <tr>
                <th>Rank</th>
                <th>Team Name</th>
                <th>Player Name</th>
                <th>Total Points</th>
            </tr>
            </thead>
            <tbody>
            {currentStandings.map((standing, index) => (
                <tr key={standing.entry_name}>
                    <td>{standing.rank}</td>
                    <td>{standing.entry_name}</td>
                    <td>{standing.player_name}</td>
                    <td>{standing.total}</td>
                </tr>
            ))}
            </tbody>
        </table>
    );
};

export default CurrentLeagueTable;