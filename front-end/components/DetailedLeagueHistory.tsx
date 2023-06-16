import {LeagueHistory} from "@/utils/types";
import React, {useState} from "react";
import {prev_data} from "@/utils/prev_season_data";
import Graph from "@/components/Graph";
import CupBrackets from "@/components/CupBrackets";

interface Props {
    league_history: LeagueHistory[];
}

const DetailedHistoryTable: React.FC<Props> = ({ league_history }) => {
    const [expandedIndices, setExpandedIndices] = useState<number[]>([]);
    const handleAccordionClick = (index: number) => {
        if (expandedIndices.includes(index)) {
            setExpandedIndices(expandedIndices.filter((i) => i !== index));
        } else {
            setExpandedIndices([...expandedIndices, index]);
        }
    };

    const rank_emojis: String[] = [' 🏆', ' 🥈', ' 🥉'];

    return (
        <div className="card">
            <h2>Detailed History</h2>
            {league_history.map((season, index) => (
                <React.Fragment key={index}>
                    <div className="card">
                    <div
                        className={`${prev_data[season.years] ? ' clickable' : ''}`}
                        onClick={prev_data[season.years] ? () => handleAccordionClick(index) : undefined}
                        style={{ cursor: prev_data[season.years] ? 'pointer' : 'default' }}
                    >
                        <h3 className="post-date">{season.years}</h3>
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
                            {season.standings.map((standing, index) => (
                                <tr key={index}>
                                    <td>{index <= 2 ? rank_emojis[index] : standing.position}</td>
                                    <td>{standing.entry_name}</td>
                                    <td>{standing.player_name.split(' ')[0]}</td>
                                    <td>{standing.points}</td>
                                </tr>
                            ))}
                            </tbody>
                        </table>
                    </div>
                        <div>
                            <br/>
                            {prev_data[season.years] ?
                                expandedIndices.includes(index) ?
                                    <>
                                        <h6>Click table to hide graph and cup</h6>
                                        <Graph league_standings={prev_data[season.years].league}/>
                                        <CupBrackets league_cup={prev_data[season.years].cup}/>
                                    </> :
                                    <h3>Click table to see graph and cup of this season!</h3> :
                                <h5>Unfortunately data from this season was not saved.</h5>
                            }
                        </div>
                    </div>
                    <br/>
                </React.Fragment>
            ))}
        </div>
    );
};

export default DetailedHistoryTable;
