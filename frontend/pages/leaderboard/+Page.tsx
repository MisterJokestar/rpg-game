import {useState} from "react";

// This is just some temp data for development
// Eventually will display gamesWon, roundsPlayed, damageDealt, damageTaken, damageHealed
const players = [
    {username: "SpongeBob", gamesWon: 210, damageDealt: 234901},
    {username: "DevonLong", gamesWon: 352, damageDealt: 18974},
    {username: "gamer123", gamesWon: 98, damageDealt: 12342},
    {username: "weavinBeaver", gamesWon: 0, damageDealt: 0},
    {username: "foobar", gamesWon: 9000, damageDealt: 0},
    {username: "SpongeBob(kinda)", gamesWon: 0, damageDealt: 234901},
];

export default function Page() {
    const [sortBy, setSortBy] = useState({ damageDealt: false, gamesWon: true});
    const toggle = (key: "damageDealt" | "gamesWon") => {
        setSortBy(prev => {
            const next = { ...prev, [key]: !prev[key] };
            if (!next.damageDealt && !next.gamesWon){
                next[key === "damageDealt" ? "gamesWon" : "damageDealt"] = true;
            }
            return next;
        })
    };

    const player_score = (p: typeof players[0]) => {
        let total = 0;
        if (sortBy.damageDealt) total += p.damageDealt;
        if (sortBy.gamesWon) total += p.gamesWon;
        return total;
    };
    const players_sorted = [...players].sort((a,b) => player_score(b) - player_score(a));

    const getRank = (index: number): number => {
        if (index === 0) return 1;
        const curr = player_score(players_sorted[index]);
        const prev = player_score(players_sorted[index-1]);
        return curr === prev ? getRank(index-1) : index+1;
    }

    return (
        <div className="h-screen bg-gray-950 text-white p-8">
            <h1 className="text-4xl font-bold text-center mb-2">Leaderboard</h1>
            <p className="text-center text-gray-400 mb-6">Sort by:</p>

            <div className="flex justify-center gap-4 mb-8">
                <label className="flex items-center gap-2 cursor-pointer">
                    <input
                        type="checkbox"
                        checked={sortBy.gamesWon}
                        onChange={() => toggle("gamesWon")}
                        className="w-4 h-4 accent-yellow-400"
                    />
                    <span className="text-sm font-medium">Games Won</span>
                </label>
                <label className="flex items-center gap-2 cursor-pointer">
                    <input
                        type="checkbox"
                        checked={sortBy.damageDealt}
                        onChange={() => toggle("damageDealt")}
                        className="w-4 h-4 accent-yellow-400"
                    />
                    <span className="text-sm font-medium">Damage Dealt</span>
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
                            <th className="px-6 py-4 text-left">Damage Dealt</th>
                        </tr>
                        </thead>
                        <tbody>
                        {players_sorted.map((p, i) =>(
                            <tr
                                key={p.username}
                                className={`border-t border-gray-700 ${i % 2 === 0 ? "bg-gray-900" :
                                    "bg-gray-950"} hover:bg-gray-800 transition-colors`}
                            >
                                <td className="px-6 py-4 font-bold text-yellow-400">#{getRank(i)}</td>
                                <td className="px-6 py-4 font-medium">{p.username}</td>
                                <td className="px-6 py-4">{p.gamesWon.toLocaleString()}</td>
                                <td className="px-6 py-4">{p.damageDealt.toLocaleString()}</td>
                            </tr>
                        ))}
                        </tbody>
                    </table>
                </div>
            </div>
            <a href="/" className="fixed bottom-8 left-8 px-6 py-3 bg-gray-800 hover:bg-gray-700
                rounded-lg text-base transition-colors"
            >Back</a>
        </div>
    );
}