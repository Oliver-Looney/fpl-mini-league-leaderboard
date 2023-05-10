import {LeagueStanding} from "@/utils/types";
import React from "react";

interface Props {
    current_league_standings: LeagueStanding[];
}

const CurrentLeagueTable: React.FC<Props> = ({ current_league_standings }) => {

    return (<div className="card">
        <h2>Current Standings</h2>
        <table>
            <thead>
            <tr>
                <th>Rank</th>
                <th>Team Name</th>
                <th>Player Name</th>
                <th>Event Total</th>
                <th>Total Points</th>
            </tr>
            </thead>
            <tbody>
            {current_league_standings.map((standing, index) => (
                <tr key={standing.entry_name}>
                    <td>{standing.rank}</td>
                    <td>{standing.entry_name}</td>
                    <td>{standing.player_name}</td>
                    <td>{standing.event_total}</td>
                    <td>{standing.total}</td>
                </tr>
            ))}
            </tbody>
        </table>
        </div>
    );
};

export default CurrentLeagueTable;