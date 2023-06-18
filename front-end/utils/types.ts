import {IRoundProps} from "react-brackets";

interface FplMiniLeagueAPIResponse {
    league_history:   LeagueHistory[];
    league_standings: LeagueStanding[];
    league_cup: IRoundProps[];
}

interface LeagueHistory {
    standings: Standing[];
    years:     string;
}

interface Standing {
    entry_name:  string;
    player_name: string;
    points:      number;
    position:    number;
    rank:        number;
}

interface LeagueStanding {
    entry_name:  string;
    event_total: number;
    events:      EventGW[];
    last_rank:   number;
    player_name: string;
    rank:        number;
    rank_sort:   number;
    total:       number;
}

interface EventGW {
    event:                   number;
    overall_rank:            number;
    overall_rank_percentile: number;
    points:                  number;
    rank:                    number;
    rank_percentile:         number;
    total_points:            number;
    position:                number;
    value:                   number;
    event_transfers:         number;
    points_on_bench:         number;
}

export type {
    FplMiniLeagueAPIResponse,
    LeagueStanding,
    LeagueHistory,
    Standing,
    EventGW
}
