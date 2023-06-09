import {LeagueStanding} from "@/utils/types";
import React from "react";

interface Props {
    current_league_standings: LeagueStanding[];
}

const CurrentLeagueTable: React.FC<Props> = ({ current_league_standings }) => {
    return (<div>
        <table>
            <thead>
            <tr>
                <th>Rank</th>
                <th>Team Name</th>
                <th>Manager</th>
                <th>Current GW</th>
                <th>Total Points</th>
            </tr>
            </thead>
            <tbody>
            {current_league_standings.map((standing) => (
                <tr key={standing.entry_name}>
                    <td>{standing.rank}</td>
                    <td>{standing.entry_name}</td>
                    <td>{standing.player_name.split(' ')[0]}</td>
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