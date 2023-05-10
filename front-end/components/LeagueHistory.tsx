import {LeagueHistory} from "@/utils/types";
import React from "react";

interface Props {
    league_history: LeagueHistory[];
}

const LeagueHistoryTable: React.FC<Props> = ({ league_history }) => {
    const history = league_history.map((season) => ({
        year: season.years,
        player: season.standings[0].player_name,
    }));

    return (<div className="card">
        <h2>History</h2>
        <table>
            <thead>
            <tr>
                <th>Year</th>
                <th>Player</th>
            </tr>
            </thead>
            <tbody>
            {history.map((entry, index) => (
                <tr key={index}>
                    <td>{entry.year}</td>
                    <td>{entry.player}</td>
                </tr>
            ))}
            </tbody>
        </table>
        </div>
    );
};

export default LeagueHistoryTable;
