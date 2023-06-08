import React from "react";
import Skeleton from "@/components/Skeleton";

const LeagueHistoryTableLoading: React.FC = () => {
    return (<div className="card">
            <h2>History</h2>
            <table>
                <thead>
                <tr>
                    <th>Year</th>
                    <th>Manager</th>
                </tr>
                </thead>
                <tbody>
                {[1,2,3].map((value) => (
                    <tr key={value}>
                        <td>{value}</td>
                        <td><Skeleton/></td>
                    </tr>
                ))}
                </tbody>
            </table>
        </div>
    );
};

export default LeagueHistoryTableLoading;