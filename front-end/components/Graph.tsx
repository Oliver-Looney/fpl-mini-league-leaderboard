import React, {useEffect, useState} from 'react';
import {LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, YAxisProps} from 'recharts';
import {LeagueStanding} from "@/utils/types";

type Props = {
    league_standings: LeagueStanding[];
};

interface EventData {
    name: number;
    [key: string]: number;
}

const getGraphData = (league_standings: LeagueStanding[], numEvents: number, dataType: string): EventData[] => {
    return Array.from({ length: numEvents }, (_, i) => {
        const eventData: EventData = { name: (i + 1) };
        league_standings.forEach((standing) => {
            // @ts-ignore
            eventData[standing.player_name] = standing.events[i][dataType];
        });
        return eventData;
    });
};

const getDataMap = (league_standings: LeagueStanding[], numEvents: number) => {
    const dataTypes = ['points', 'position', 'overall_rank_percentile', 'event_transfers', 'points_on_bench', 'value'];

    return dataTypes.reduce((map, dataType) => {
        map[dataType] = getGraphData(league_standings, numEvents, dataType);
        return map;
    }, {} as Record<string, EventData[]>);
};

const getYAxisProps = (selectedData: string, dataMap: Record<string, EventData[]>, league_standings: LeagueStanding[]): YAxisProps => {
    if (selectedData === 'position' || selectedData === 'overall_rank_percentile') {
        const domain = selectedData === 'position' ? [0, league_standings.length + 1] : [0, 100];
        const ticks = selectedData === 'position' ? [1, 2, 3, 4] : undefined;
        return {
            type: 'number',
            domain,
            reversed: true,
            ticks
        };
    } else if (selectedData === 'value') {
        const maxTeamValue = Math.max(...dataMap['value'].map((data) => Math.max(...Object.values(data))));
        return {
            domain: [95, maxTeamValue + 1]
        };
    } else {
        return {};
    }
};


const Graph: React.FC<Props> = ({ league_standings }) => {
    const numEvents = league_standings[0].events.length; // assume all players have the same number of events
    const dataMap = getDataMap(league_standings, numEvents);
    const [selectedData, setSelectedData] = useState('points');
    const [chartData, setChartData] = useState(dataMap[selectedData]);

    useEffect(() => {
        setChartData(dataMap[selectedData]);
    }, [selectedData]);

    const colors: { [key: string]: string } = {
        'Oliver Looney': 'blue',
        'Declan Mallon': 'red',
        'Daniel Rafferty': 'green',
        'Thomas Matthews': 'black'
    }

    return (
        <div>
            <div>
                <label htmlFor="data-select">Select Data:</label>
                <select id="data-select" value={selectedData} onChange={(e) => setSelectedData(e.target.value)}>
                    <option value="points">Points</option>
                    <option value="position">Positions</option>
                    <option value="event_transfers">Number of Transfers</option>
                    <option value="points_on_bench">Points Left on Bench</option>
                    <option value="value">Team Value</option>
                    <option value="overall_rank_percentile">Overall Rank %</option>
                </select>
            </div>

            <div style={{ width: '100%', overflowX: 'auto', overflowY: 'hidden'}}>
                <LineChart width={1000} height={300} data={chartData}>
                    <CartesianGrid strokeDasharray="3 3" />
                    <XAxis dataKey="name" />
                    <YAxis {...getYAxisProps(selectedData, dataMap, league_standings)} />
                    <Tooltip />
                    <Legend />
                    {league_standings.map((standing) => (
                        <Line key={standing.player_name} type="monotone" dataKey={standing.player_name} stroke={colors[standing.player_name]} />
                    ))}
                </LineChart>
            </div>
        </div>
    );
};

export default Graph;
