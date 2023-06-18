import {LeagueHistory} from "@/utils/types";
import React from "react";
import {prev_data} from "@/utils/prev_season_data";

interface Props {
    league_history: LeagueHistory[];
}

const LeagueHistoryTable: React.FC<Props> = ({ league_history }) => {
    const history = league_history.map((season) => ({
        year: season.years,
        player: season.standings[0].player_name,
    }));

    return (
        <div className="card">
            <h2>History</h2>
            <div>
                <table>
                    <thead>
                    <tr>
                        <th>Season</th>
                        <th>League</th>
                        <th>Cup</th>
                    </tr>
                    </thead>
                    <tbody>
                    {history.map((entry, index) => (
                        <tr key={index}>
                            <td>{entry.year}</td>
                            <td>{entry.player.split(" ")[0]}</td>
                            <td>
                                {/*if there is cup data for this season*/}
                                {prev_data[entry.year]?.cup ? (
                                    // display winners name if cup data exists
                                    prev_data[entry.year].cup[prev_data[entry.year].cup.length - 1].seeds[0].teams[0].entry ===
                                    prev_data[entry.year].cup[prev_data[entry.year].cup.length - 1].seeds[0].winner
                                        ? (
                                        prev_data[entry.year].cup[prev_data[entry.year].cup.length - 1].seeds[0].teams[0].player_name.split(" ")[0]
                                    ) : (
                                        prev_data[entry.year].cup[prev_data[entry.year].cup.length - 1].seeds[0].teams[1].player_name.split(" ")[0]
                                    )
                                ) : '---'}
                            </td>
                        </tr>
                    ))}
                    </tbody>
                </table>
            </div>
        </div>
    );
};

export default LeagueHistoryTable;
