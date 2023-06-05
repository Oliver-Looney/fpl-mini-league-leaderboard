import Head from 'next/head'
import {FplMiniLeagueAPIResponse} from "@/utils/types";
import {useEffect, useState} from "react";
import axios from "axios";
import CurrentLeagueOutput from "@/components/CurrentLeagueOutput";
import LeagueHistory from "@/components/LeagueHistory";
import DetailedHistoryTable from "@/components/DetailedLeagueHistory";
import LeaderBoard from "@/components/LeaderBoard";
import CurrentWinner from "@/components/CurrentWinner";
import Header from "@/components/Header";

export default function Home() {
    async function fetchFplMiniLeagueApiData(): Promise<FplMiniLeagueAPIResponse> {
        try {
            const response = await axios.get<FplMiniLeagueAPIResponse>('https://43vewwe578.execute-api.eu-west-1.amazonaws.com/default/fpl-mini-league-api');
            return response.data;
        } catch (error) {
            console.error(error);
            setErrorWhileFetchingDataFlag(true);
            throw error;
        }
    }

    const [apiData, setApiData] = useState<FplMiniLeagueAPIResponse | null>(null);
    const [errorWhileFetchingDataFlag, setErrorWhileFetchingDataFlag] = useState<boolean>(false);

    useEffect(() => {
        async function fetchData() {
            const data = await fetchFplMiniLeagueApiData();
            setApiData(data);
        }

        fetchData();
    }, []);

  return (
    <>
      <Head>
        <title>DG Invitational Leaderboard</title>
        <meta name="description" content="FPL Mini-League Leaderboard" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/league_trophy.jpg" />
      </Head>
        <Header/>
        <div className="container">
        <h1>The David Goggins Invitational</h1>
            {!apiData ?
                errorWhileFetchingDataFlag ?
                    <div>Oh no! An error has occurred.</div> :
                    <div>Loading...</div>
                :
            <div>
                <CurrentLeagueOutput league_data={apiData}/>
                <br/>
                <LeagueHistory league_history={apiData.league_history}/>
                <br/>
                <DetailedHistoryTable league_history={apiData.league_history}/>
                <br/>
                <LeaderBoard data={apiData}/>
                <br/>
                <CurrentWinner league_history={apiData.league_history}/>
            </div>
            }
            <br/>
        </div>
    </>
  )
}
