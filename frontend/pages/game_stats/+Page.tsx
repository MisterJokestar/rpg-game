import {GameStats} from "../../models/game";
import { useState, useEffect } from "react";
import axios from "axios";

// Function to make the call to the cloud function for getting the game
async function get_stats(gameId: string) {
    let response = await axios.post(
        'https://cloud-functions-91972588391.us-central1.run.app/getStats',
        {
            gameId: gameId
        }
    );
    console.log(response.data);
    return response.data as GameStats
}

// http://localhost:3000/game_stats?game-id=jlEFef63JiiXV2bBtqwM
export default function Page() {
    //TODO: Currently does not show enemies defeated
    const [gameStats, setGameStats] = useState<GameStats | null>(null);

    useEffect(() => {
        const fetchData = async () => {
            const id = new URLSearchParams(window.location.search).get("game-id");
            if (id) {
                setGameStats(await get_stats(id));
            }

        }
        fetchData();
    }, []);

        const character_name = gameStats?.name;
        const player_stats = gameStats?.stats;
        const win = gameStats?.win;
        const round = gameStats?.round;
        const turn = gameStats?.turn;
        const complete = gameStats?.status;
        const player_state = gameStats?.player_state
        const enemy_state = gameStats?.enemy_state;
        console.log(player_stats?.defense)


    if (!gameStats){
        return(
            <div className="min-h-screen bg-gray-950 text-white flex items-center justify-center">
                <p className="text-gray-400">Loading...</p>
            </div>
        );
    }
    return (

        <div className="min-h-screen bg-gray-950 text-white p-8">
            <h1 className="text-4xl font-bold text-center mb-2">Game Stats</h1>
            {/*<p className="text-center text-gray-400 mb-8">Game ID: {game.id}</p>*/}

            {/* Result */}
            {complete! && (
                <div className={`text-center text-2xl font-bold mb-8 ${win! ? "text-green-400" :
                "text-red-400"}`}>
                    {win ? "Victory" : "Defeat"}
                </div>
            )}

            <div className="max-w-2xl mx-auto flex flex-col gap-6">

                {/* Character */}
                <div className="bg-gray-900 rounded-xl border border-gray-700 p-6">
                    <h2 className="text-gray-400 uppercase text-xs font-semibold mb-4">Character</h2>
                    <p className="text-xl font-bold">{character_name}</p>
                    <p className="text-gray-400 text-sm mt-1">Round {round!} • Turn {turn!}</p>
                    <div className="grid grid-cols-3 gap-4 mt-4">
                        <div>
                            <p className="text-gray-400 text-sm">Power</p>
                            <p className="text-xl font-bold text-red-400">{player_stats?.power}</p>
                        </div>
                        <div>
                            <p className="text-gray-400 text-sm">Defense</p>
                            <p className="text-xl font-bold text-blue-400">{player_stats?.defense}</p>
                        </div>
                        <div>
                            <p className="text-gray-400 text-sm">Speed</p>
                            <p className="text-xl font-bold text-green-400">{player_stats?.speed}</p>
                        </div>
                    </div>
                </div>

                {/* Health */}
                <div className="bg-gray-900 rounded-xl border border-gray-700 p-6">
                    <h2 className="text-gray-400 uppercase text-xs font-semibold mb-4">Health</h2>
                    <p className="text-xl font-bold">{player_state!.health.current} / {player_state!.health.max}</p>
                </div>

                {/* Combat Stats */}
                <div className="bg-gray-900 rounded-xl border border-gray-700 p-6">
                    <h2 className="text-gray-400 uppercase text-xs font-semibold mb-4">Combat Stats</h2>
                    <div className="grid grid-cols-2 gap-4">
                        <div>
                            <p className="text-gray-400 text-sm">Damage Dealt</p>
                            <p className="text-xl font-bold text-red-400">{player_state!.damage_dealt.toLocaleString()}</p>
                        </div>
                        <div>
                            <p className="text-gray-400 text-sm">Damage Taken</p>
                            <p className="text-xl font-bold text-orange-400">{player_state!.damage_taken.toLocaleString()}</p>
                        </div>
                        <div>
                            <p className="text-gray-400 text-sm">Damage Healed</p>
                            <p className="text-xl font-bold text-green-400">{player_state!.damage_healed.toLocaleString()}</p>
                        </div>
                        <div>
                            <p className="text-gray-400 text-sm">Damage Blocked</p>
                            <p className="text-xl font-bold text-blue-400">{player_state!.damage_blocked.toLocaleString()}</p>
                        </div>
                        <div>
                            <p className="text-gray-400 text-sm">Damage Dodged</p>
                            <p className="text-xl font-bold text-purple-400">{player_state!.damage_dodged.toLocaleString()}</p>
                        </div>
                    </div>
                </div>

                {/* Enemy */}
                <div className="bg-gray-900 rounded-xl border border-gray-700 p-6">
                    <h2 className="text-gray-400 uppercase text-xs font-semibold mb-4">Enemy</h2>
                    <div className="grid grid-cols-2 gap-4">
                        <div>
                            <p className="text-gray-400 text-sm">Type</p>
                            <p className="text-xl font-bold">{enemy_state!.type}</p>
                        </div>
                        <div>
                            <p className="text-gray-400 text-sm">Health</p>
                            <p className="text-xl font-bold">{enemy_state!.health.current} / {enemy_state!.health.max}</p>
                        </div>
                    </div>
                </div>
            </div>

            <a href="/character" className="fixed bottom-8 left-8 px-6 py-3 bg-gray-800 hover:bg-gray-700
            rounded-lg text-base transition-colors"
            >Back</a>
        </div>
    );
}