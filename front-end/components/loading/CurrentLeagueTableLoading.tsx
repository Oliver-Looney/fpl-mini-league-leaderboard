import React from "react";
import Skeleton from "@/components/Skeleton";

const CurrentLeagueTableLoading: React.FC = () => {
    return (<div>
            <table>
                <thead>
                <tr>
                    <th>Rank</th>
                    <th>Team Name</th>
                    <th>Manager</th>
                    <th>Current GW</th>
                    <th>Total Points</th>
                </tr>
                </thead>
                <tbody>
                {[1,2,3].map((value) => (
                    <tr key={value}>
                        <td>{value}</td>
                        <td><Skeleton/></td>
                        <td><Skeleton/></td>
                        <td><Skeleton/></td>
                        <td><Skeleton/></td>
                    </tr>
                ))}
                </tbody>
            </table>
        </div>
    );
};

export default CurrentLeagueTableLoading;
