import {Bracket, IRenderSeedProps, IRoundProps, Seed, SeedItem, SeedTeam} from 'react-brackets';
import React from "react";

interface Props {
    league_cup: IRoundProps[];
}

// const CustomTitle = ({seed, breakpoint, roundIndex, seedIndex}: IRenderSeedProps) => {
//     return <div>
//         <h1>{seed.title}</h1>
//         <h3>Gameweek</h3>
//     </div>
// };

const CustomSeed = ({seed, breakpoint}: IRenderSeedProps) => {
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
                        fontWeight: seed.teams[value].entry == seed.winner ? 'bold' : 'normal',
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
const CupBrackets: React.FC<Props> = ({ league_cup }) => {
    return <div style={{ width: '100%', overflowX: 'auto', overflowY: 'hidden'}}>
        <Bracket mobileBreakpoint={757} rounds={league_cup} renderSeedComponent={CustomSeed} swipeableProps={{ enableMouseEvents: true, animateHeight: true }}/>
    </div>
};

export default CupBrackets;
