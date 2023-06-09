import React, {useEffect, useState} from 'react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend } from 'recharts';
import {LeagueStanding} from "@/utils/types";

type Props = {
    league_standings: LeagueStanding[];
};

const getPointsGraphData = (league_standings: LeagueStanding[], numEvents: number) => {
    const data = [];
    for (let i = 0; i < numEvents; i++) {
        const eventData = { name: (i+1).toString() };

        for (let standing of league_standings) {
            // @ts-ignore
            eventData[standing.player_name] = standing.events[i].points;
        }

        data.push(eventData);
    }
    return data
}
const getPositionsGraphData = (league_standings: LeagueStanding[], numEvents: number) => {
    const data = [];
    for (let i = 0; i < numEvents; i++) {
        const eventData = { name: (i+1).toString() };

        for (let standing of league_standings) {
            // @ts-ignore
            eventData[standing.player_name] = standing.events[i].position;
        }

        data.push(eventData);
    }
    return data
}
const getOverallRankPercentileGraphData = (league_standings: LeagueStanding[], numEvents: number) => {
    const data = [];
    for (let i = 0; i < numEvents; i++) {
        const eventData = { name: (i+1).toString() };

        for (let standing of league_standings) {
            // @ts-ignore
            eventData[standing.player_name] = standing.events[i].overall_rank_percentile;
        }

        data.push(eventData);
    }
    return data
}

const Graph: React.FC<Props> = ({ league_standings }) => {
    const numEvents = league_standings[0].events.length; // assume all players have the same number of events
    let pointsData = getPointsGraphData(league_standings, numEvents);
    let positionsData = getPositionsGraphData(league_standings, numEvents);
    let overallRankPercentileData = getOverallRankPercentileGraphData(league_standings, numEvents);

    const [selectedData, setSelectedData] = useState('points');
    const [chartData, setChartData] = useState(pointsData);


    useEffect(() => {
        if (selectedData === 'points') {
            setChartData(pointsData);
        } else if (selectedData === 'Overall Rank %') {
            setChartData(overallRankPercentileData);
        } else {
            setChartData(positionsData);
        }
    }, [selectedData]);

    let colors = ['blue', 'red', 'green', 'black']

    return (
        <div className="card">
            <h2>Graph</h2>

            <div>
                <label htmlFor="data-select">Select Data:</label>
                <select id="data-select" value={selectedData} onChange={(e) => setSelectedData(e.target.value)}>
                    <option value="points">Points</option>
                    <option value="positions">Positions</option>
                    <option value="Overall Rank %">Overall Rank %</option>
                </select>
            </div>

            <div style={{ width: '100%', overflowX: 'auto', overflowY: 'hidden'}}>
                <LineChart width={1000} height={300} data={chartData}>
                    <CartesianGrid strokeDasharray="3 3" />
                    <XAxis dataKey="name" />
                    <YAxis />
                    <Tooltip />
                    <Legend />
                    {league_standings.map((standing, index) => (
                        <Line key={standing.player_name} type="monotone" dataKey={standing.player_name} stroke={colors[index]} />
                    ))}
                </LineChart>
            </div>
        </div>
    );
};

export default Graph;
