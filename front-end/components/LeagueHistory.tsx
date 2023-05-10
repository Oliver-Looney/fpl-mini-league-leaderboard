import {LeagueHistory} from "@/utils/types";
import React from "react";

interface Props {
    data: LeagueHistory[];
}

const LeagueHistoryTable: React.FC<Props> = ({ data }) => {
    const history = data.map((season) => ({
        year: season.years,
        player: season.standings[0].player_name,
    }));

    return (<>
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
        </>
    );
};

export default LeagueHistoryTable;
