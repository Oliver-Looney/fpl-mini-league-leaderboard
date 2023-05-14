import React from "react";
import {FplMiniLeagueAPIResponse} from "@/utils/types";
import CurrentLeagueTable from "@/components/CurrentLeagueTable";
import Graph from "@/components/Graph";

interface Props {
    league_data: FplMiniLeagueAPIResponse;
}

const CurrentLeagueOutput: React.FC<Props> = ({ league_data }) => {
    const current_league_standings = league_data.league_standings;

    return (
        <div className="card">
            <h2>Current Standings</h2>
            <CurrentLeagueTable current_league_standings={current_league_standings} />
            <br/>
            <Graph league_standings={current_league_standings} />
        </div>
    );
};

export default CurrentLeagueOutput;
