import React from "react";
import CurrentLeagueTableLoading from "@/components/loading/CurrentLeagueTableLoading";
import GraphLoading from "@/components/loading/GraphLoading";

const CurrentLeagueOutputLoading: React.FC = () => {

    return (
        <div className="card">
            <h2>Current Standings</h2>
            <CurrentLeagueTableLoading/>
            <br/>
            <GraphLoading/>
        </div>
    );
};

export default CurrentLeagueOutputLoading;