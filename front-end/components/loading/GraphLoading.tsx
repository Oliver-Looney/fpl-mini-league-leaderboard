import React from "react";
import Skeleton from "@/components/Skeleton";

const GraphLoading: React.FC = () => {

    return (
        <div>
            <h2>Graph</h2>
            <div>
                <label htmlFor="data-select">Select Data:</label>
                {[0,1,2,3,4,5,6,7,8,9].map(key =>
                    <Skeleton key={key}/>
                )}
            </div>
            <Skeleton/>
        </div>
    );
};

export default GraphLoading;