import React, { FC } from 'react';
import Skeleton from "@/components/Skeleton";

const LeaderBoardLoading: FC = () => {
    return (
        <div className="card">
            <h2>Leaderboard</h2>
            <table>
            <thead>
            <tr>
                <th>Manager</th>
                <th>Wins</th>
            </tr>
            </thead>
            <tbody>
            {[1,2,3].map((value, index) => (
                <tr key={index}>
                    <td><Skeleton/></td>
                    <td><Skeleton/></td>
                </tr>
            ))}
            </tbody>
            </table>
        </div>
    );
};

export default LeaderBoardLoading;