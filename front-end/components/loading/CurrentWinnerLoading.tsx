import React from "react";
import Skeleton from "@/components/Skeleton";

const CurrentWinner: React.FC = () => {
    return (
        <div className="card">
            <h2>Current Winner:</h2>
            <h3><Skeleton/></h3>
            <h4><Skeleton/></h4>
            <img src="league_trophy.jpg" height="500px" width="300px" alt="league trophy"/>
        </div>
    );
};

export default CurrentWinner;