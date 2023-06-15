import React from "react";
import {FplMiniLeagueAPIResponse} from "@/utils/types";
import CurrentLeagueTable from "@/components/CurrentLeagueTable";
import Graph from "@/components/Graph";
import CupBrackets from "@/components/CupBrackets";

interface Props {
    league_data: FplMiniLeagueAPIResponse;
}

const CurrentLeagueOutput: React.FC<Props> = ({ league_data }) => {
    const current_league_standings = league_data.league_standings;

    return (
        <div className="card">
            <h2>Current Standings</h2>
            <CurrentLeagueTable current_league_standings={current_league_standings} />
            <br/>
            <Graph league_standings={current_league_standings} />
            <br/>
            {/*<tr className="H2HMatch__StyledMatchRow-suqnjx-5 kZxVAy">*/}
            {/*    <td>38</td>*/}
            {/*    <td className="H2HMatch__MatchesEntry1-suqnjx-6 fSBnCp"><a className="Link-a4a9pd-1 kofttw"*/}
            {/*                                                               href="/entry/563443/event/38"><strong>Fantasy*/}
            {/*        Crew 2</strong>Oliver Looney</a></td>*/}
            {/*    <td className="H2HMatch__MatchesScoreWrap-suqnjx-9 jMorVD">*/}
            {/*        <div className="H2HMatch__MatchesScore-suqnjx-2 lYoGj"><span*/}
            {/*            className="H2HMatch__MatchesScoreTeam-suqnjx-3 gvzamF">39</span><span*/}
            {/*            className="H2HMatch__MatchesScoreTeam-suqnjx-3 gvzamF">46</span></div>*/}
            {/*    </td>*/}
            {/*    <td className="H2HMatch__MatchesEntry2-suqnjx-7 dURqFH"><a className="Link-a4a9pd-1 kofttw"*/}
            {/*                                                               href="/entry/2213457/event/38"><strong>Invincibulls</strong>Daniel*/}
            {/*        Rafferty</a></td>*/}
            {/*</tr>*/}
            <CupBrackets/>
        </div>
    );
};

export default CurrentLeagueOutput;
