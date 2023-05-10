import {LeagueHistory} from "@/utils/types";
import React from "react";

interface Props {
    league_history: LeagueHistory[];
}

const DetailedHistoryTable: React.FC<Props> = ({ league_history }) => {
    return (
        <div className="card">
            <h2>Detailed History</h2>
            {league_history.map((season, index) => (
                <div key={index} className="card">
                    <h3>{season.years}</h3>
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
                        {season.standings.map((standing, index) => (
                            <tr key={index}>
                                <td>{standing.position}</td>
                                <td>{standing.entry_name}</td>
                                <td>{standing.player_name}</td>
                                <td>{standing.points}</td>
                            </tr>
                        ))}
                        </tbody>
                    </table>
                    <br/>
                </div>
            ))}
        </div>
    );
};

export default DetailedHistoryTable;
