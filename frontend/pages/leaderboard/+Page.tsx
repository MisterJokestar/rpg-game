import {useEffect, useState} from "react";
import {apiClient} from "../../axiosConfig";
import { navigate } from "vike/client/router";
import { LeaderBoardEntry } from "../../models/api";

// the data the leaderboard will be sorted-on
type SortKey = "games_won" | "rounds_survived" | "damage_dealt" | "enemies_defeated";
export default function Page() {

    // tracks which stats are being used for sorting
    const [sortBy, setSortBy] = useState<Record<SortKey, boolean>>({
        games_won: true,
        rounds_survived: false,
        damage_dealt: false,
        enemies_defeated: false,
    });
    // holds the aggregated leaderboard entries received from the cloud function
    const [entries, setEntries] = useState<LeaderBoardEntry[]>([]);

    // fetch and parse the leaderboard on page load
    useEffect(() => {
        const fetchData = async () => {
            try {
                let response = await apiClient.get('/leaderboard');
                let leaderboard: LeaderBoardEntry[] = response.data;
                setEntries(leaderboard);
            } catch (error) {
                console.log(error);
            }
        }
        fetchData();
    }, []);

    console.log(entries);

    // toggles a sort key on | off, ensuring at least one is always checked
    const toggle = (key: SortKey) => {
        setSortBy(prev => {
            const next = { ...prev, [key]: !prev[key] };
            const anyChecked = Object.values(next).some(v => v);
            if (!anyChecked) next[key === "games_won" ? "rounds_survived" : "games_won"] = true;
                return next;
        });
    };

    // calculates a players total score based on whichever sort keys are checked
    const player_score = (p: LeaderBoardEntry) => {
        let total = 0;
        if (sortBy.games_won) total += p.wins;
        if (sortBy.rounds_survived) total += p.rounds;
        if (sortBy.damage_dealt) total += p.damage_dealt;
        if (sortBy.enemies_defeated) total += p.enemies_defeated;
        return total;
    };
    const entries_sorted = [ ...entries].sort((a,b) => player_score(b) - player_score(a));

    // derives the rank from score, players with equal score share a rank
    const getRank = (index: number): number => {
        if (index === 0) return 1;
        const curr = player_score(entries_sorted[index]);
        const prev = player_score(entries_sorted[index-1]);
        return curr === prev ? getRank(index-1) : index+1;
    }

    return (
        <div className="min-h-screen bg-gray-950 text-white p-8">
            <h1 className="text-4xl font-bold text-center mb-2">Leaderboard</h1>
            <p className="text-center text-gray-400 mb-6">Sort by:</p>

            <div className="flex justify-center gap-4 mb-8">
                <label className="flex items-center gap-2 cursor-pointer">
                    <input
                        type="checkbox"
                        checked={sortBy.games_won}
                        onChange={() => toggle("games_won")}
                        className="w-4 h-4 accent-yellow-400"
                    />
                    <span className="text-sm font-medium">Games Won</span>
                </label>
                <label className="flex items-center gap-2 cursor-pointer">
                    <input
                        type="checkbox"
                        checked={sortBy.rounds_survived}
                        onChange={() => toggle("rounds_survived")}
                        className="w-4 h-4 accent-yellow-400"
                    />
                    <span className="text-sm font-medium">Rounds Survived</span>
                </label>
                <label className="flex items-center gap-2 cursor-pointer">
                    <input
                        type="checkbox"
                        checked={sortBy.damage_dealt}
                        onChange={() => toggle("damage_dealt")}
                        className="w-4 h-4 accent-yellow-400"
                    />
                    <span className="text-sm font-medium">Damage Dealt</span>
                </label>
                <label className="flex items-center gap-2 cursor-pointer">
                    <input
                        type="checkbox"
                        checked={sortBy.enemies_defeated}
                        onChange={() => toggle("enemies_defeated")}
                        className="w-4 h-4 accent-yellow-400"
                    />
                    <span className="text-sm font-medium">Enemies Defeated</span>
                </label>
            </div>

            <div className="max-w-4xl mx-auto overflow-hidden rounded-xl border border-gray-700">
                <div className="overflow-y-auto max-h-[60vh]">
                    <table className="w-full text-sm">
                        <thead className="bg-gray-800 text-gray-400 uppercase text-xs sticky top-0">
                            <tr>
                                <th className="px-6 py-4 text-left">Rank</th>
                                <th className="px-6 py-4 text-left">Player</th>
                                <th className="px-6 py-4 text-left">Games Won</th>
                                <th className="px-6 py-4 text-left">Rounds Survived</th>
                                <th className="px-6 py-4 text-left">Damage Dealt</th>
                                <th className="px-6 py-4 text-left">Enemies Defeated</th>
                            </tr>
                        </thead>
                        <tbody>
                        {entries_sorted.map((p, i) =>(
                            <tr
                                key={p.player_name}
                                className={`border-t border-gray-700 ${i % 2 === 0 ? "bg-gray-900" :
                                    "bg-gray-950"} hover:bg-gray-800 transition-colors`}
                            >
                                <td className="px-6 py-4 font-bold text-yellow-400">#{getRank(i)}</td>
                                <td className="px-6 py-4 font-medium">{p.player_name}</td>
                                <td className="px-6 py-4">{p.wins.toLocaleString()}</td>
                                <td className="px-6 py-4">{p.rounds.toLocaleString()}</td>
                                <td className="px-6 py-4">{p.damage_dealt.toLocaleString()}</td>
                                <td className="px-6 py-4">{p.enemies_defeated.toLocaleString()}</td>
                            </tr>
                        ))}
                        </tbody>
                    </table>
                </div>
            </div>
            <a
                onClick={() => navigate(localStorage.getItem("userId") ? "/dashboard" : "/")}
                className="fixed bottom-8 left-8 px-6 py-3 bg-gray-800 hover:bg-gray-700
                rounded-lg text-base transition-colors cursor-pointer"
            >Back</a>
        </div>
    );
}
