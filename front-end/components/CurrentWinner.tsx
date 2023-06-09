import {LeagueHistory} from "@/utils/types";
import React from "react";

interface Props {
    league_history: LeagueHistory[];
}

const CurrentWinner: React.FC<Props> = ({ league_history }) => {
    return (
        <div className="card">
            <h2>Current Winner:</h2>
            <h3>{league_history[0].standings[0].player_name}</h3>
            <h4>{league_history[0].standings[0].entry_name}</h4>
            <img src="league_trophy.jpg" height="250px" width="150px" alt="league trophy"/>
        </div>
    );
};

export default CurrentWinner;
