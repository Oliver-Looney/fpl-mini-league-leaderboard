import {Bracket, IRenderSeedProps, IRoundProps, Seed, SeedItem, SeedTeam} from 'react-brackets';
import React from "react";

const rounds: IRoundProps[] = [
    {
        title: 'Semi Finals',
        seeds: [
            {
                id: 1,
                date: "Gameweek 37",
                teams: [
                    { name: 'Oliver Looney', player_name: "Boat Crew 2", points: 23, entry: 563443 },
                    { name: 'Team B', "points": 14 }
                ],
                winner: "Oliver Looney"
            },
            {
                id: 2,
                date: "Gameweek 37",
                teams: [{ name: 'Team C', points: 65 }, { name: 'Team D', points: 43 }],
                winner: "Team C"
            },
        ],
    },
    {
        title: 'Final', // from api
        seeds: [
            {
                id: 3, // from api
                date: "Gameweek 38", // from api or calc ?
                teams: [{ name: 'Team A', points: 55 }, { name: 'Team C', points: 33 }],
                winner: "Team A" // from api
            },
        ],
    },
];

// const CustomTitle = ({seed, breakpoint, roundIndex, seedIndex}: IRenderSeedProps) => {
//     return <div>
//         <h1>{seed.title}</h1>
//         <h3>Gameweek</h3>
//     </div>
// };

const CustomSeed = ({seed, breakpoint, roundIndex, seedIndex}: IRenderSeedProps) => {
    // breakpoint passed to Bracket component
    // to check if mobile view is triggered or not

    // mobileBreakpoint is required to be passed down to a seed
    return (
        <Seed mobileBreakpoint={breakpoint} style={{ fontSize: 12 }}>
            <SeedItem>
                <div>
                    {[0,1].map((value, index) => (
                        <SeedTeam key={index}
                        style={{
                        whiteSpace: 'pre-line',
                        fontWeight: seed.teams[value].name == seed.winner ? 'bold' : 'normal',
                        display: 'flex',
                        justifyContent: 'center',
                        alignItems: 'center'
                    }}
                >
                    {seed.teams[value]?.name || 'NO TEAM'}
                    {/*{'\n'}*/}{' -  '}
                    {seed.teams[value]?.player_name || 'NO PLAYER'}
                    {'\n'}
                    {seed.teams[value]?.points || 'NO POINTS'}
                </SeedTeam>
                    ))}
                </div>
            </SeedItem>
            <p>{seed.date}</p>
        </Seed>
    );
};
const CupBrackets = () => {
    return <Bracket rounds={rounds} renderSeedComponent={CustomSeed}/>;
};

export default CupBrackets;
