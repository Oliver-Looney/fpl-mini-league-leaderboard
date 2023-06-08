import React from "react";
import Skeleton from "@/components/Skeleton";

const GraphLoading: React.FC = () => {

    return (
        <div className="card">
            <h2>Graph</h2>
            <div>
                <label htmlFor="data-select">Select Data:</label>
                <Skeleton/>
            </div>
            <Skeleton/>
        </div>
    );
};

export default GraphLoading;