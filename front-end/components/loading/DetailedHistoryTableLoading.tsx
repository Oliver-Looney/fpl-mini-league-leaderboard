import React from "react";
import Skeleton from "@/components/Skeleton";
const getCurrentYear = (): number => {
    const currentDate = new Date();
    return currentDate.getFullYear();
};

const DetailedHistoryTableLoading: React.FC = () => {
    return (
        <div className="card">
            <h2>Detailed History</h2>
            {[1,2,3].map((value, index) => (
                <div key={index} className="card">
                    <h3 className="post-date">{getCurrentYear()-index}/{getCurrentYear()%100 - index}</h3>
                    <table>
                        <thead>
                        <tr>
                            <th>Rank</th>
                            <th>Team Name</th>
                            <th>Manager</th>
                            <th>Total Points</th>
                        </tr>
                        </thead>
                        <tbody>
                        {[1,2,3].map((value, index) => (
                            <tr key={index}>
                                <td>{value}</td>
                                <td><Skeleton/></td>
                                <td><Skeleton/></td>
                                <td><Skeleton/></td>
                            </tr>
                        ))}
                        </tbody>
                    </table>
                    <br/>
                </div>
            ))}
        </div>
    );
};

export default DetailedHistoryTableLoading;
