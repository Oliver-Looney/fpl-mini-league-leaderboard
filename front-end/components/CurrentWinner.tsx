import {LeagueHistory} from "@/utils/types";
import React from "react";

interface Props {
    data: LeagueHistory[];
}

const CurrentWinner: React.FC<Props> = ({ data }) => {
    return (
        <div>
            <h2>Current Winner:</h2>
            <p>{data[0].standings[0].player_name}</p>
            <img src="league_trophy.jpg" height="500px" width="300px" alt="league trophy"/>
        </div>
    );
};

export default CurrentWinner;
