import {Character, Game} from "../../models/game";
import { useState, useEffect } from "react";
import { apiClient } from "../../axiosConfig";
import { navigate } from "vike/client/router";

/**
 * Game stats page (`/game_stats`).
 *
 * Reads the `?game-id=<id>` query parameter on mount, fetches the game record
 * via `GET /game/:id` and the associated character via
 * `GET /character/:character_id`, then displays a full post-game summary
 * including result, character stats, combat statistics, and enemies defeated.
 *
 * A "Continue Game" button is shown when `game.complete` is `false`, allowing
 * the player to resume an in-progress game.
 */
export default function Page() {

    const [game, setGame] = useState<Game | null>(null);
    const [character, setCharacter] = useState<Character | null>(null);

    useEffect(() => {
        const fetchData = async () => {
            const id = new URLSearchParams(window.location.search).get("game-id");
            if (id) {
                try {
				    let game_response = await apiClient.get(
				        `/game/${id}`
				    );
				    let game_data: Game = game_response.data;
                    setGame(game_data);
                    let character_response = await apiClient.get(
                        `/character/${game_data.player_state.character_id}`
                    );
                    let character_data: Character = character_response.data;
                    setCharacter(character_data);
                } catch (error) {
                    console.log(error);
                }
            }

        }
        fetchData();
    }, []);

    if (!game && !character){
        return(
            <div className="min-h-screen bg-gray-950 text-white flex items-center justify-center">
                <p className="text-gray-400">Loading...</p>
            </div>
        );
    }
    return (

        <div className="min-h-screen bg-gray-950 text-white p-8">
            <h1 className="text-4xl font-bold text-center mb-2">Game Stats</h1>
            {<p className="text-center text-gray-400 mb-8">Game ID: {game?.id}</p>}

            {/* Result */}
            {game?.win !== null && (
                <div className={`text-center text-2xl font-bold mb-8 ${game?.win ? "text-green-400" :
                "text-red-400"}`}>
                    {game?.win ? "Victory" : "Defeat"}
                </div>
            )}

            <div className="max-w-2xl mx-auto flex flex-col gap-6">

                {/* Character */}
                <div className="bg-gray-900 rounded-xl border border-gray-700 p-6">
                    <h2 className="text-gray-400 uppercase text-xs font-semibold mb-4">Character</h2>
                    <p className="text-xl font-bold">{character?.name}</p>
                    <p className="text-gray-400 text-sm mt-1">Round {game?.round} • Turn {game?.turn}</p>
                    <div className="grid grid-cols-3 gap-4 mt-4">
                        <div>
                            <p className="text-gray-400 text-sm">Power</p>
                            <p className="text-xl font-bold text-red-400">{character?.stats.power}</p>
                        </div>
                        <div>
                            <p className="text-gray-400 text-sm">Defense</p>
                            <p className="text-xl font-bold text-blue-400">{character?.stats.defense}</p>
                        </div>
                        <div>
                            <p className="text-gray-400 text-sm">Speed</p>
                            <p className="text-xl font-bold text-green-400">{character?.stats.speed}</p>
                        </div>
                    </div>
                </div>

                {/* Health */}
                <div className="bg-gray-900 rounded-xl border border-gray-700 p-6">
                    <h2 className="text-gray-400 uppercase text-xs font-semibold mb-4">Health</h2>
                    <p className="text-xl font-bold">{game?.player_state.health.current} / {game?.player_state.health.max}</p>
                </div>

                {/* Combat Stats */}
                <div className="bg-gray-900 rounded-xl border border-gray-700 p-6">
                    <h2 className="text-gray-400 uppercase text-xs font-semibold mb-4">Combat Stats</h2>
                    <div className="grid grid-cols-2 gap-4">
                        <div>
                            <p className="text-gray-400 text-sm">Damage Dealt</p>
                            <p className="text-xl font-bold text-red-400">{game?.player_state.damage_dealt.toLocaleString()}</p>
                        </div>
                        <div>
                            <p className="text-gray-400 text-sm">Damage Taken</p>
                            <p className="text-xl font-bold text-orange-400">{game?.player_state.damage_taken.toLocaleString()}</p>
                        </div>
                        <div>
                            <p className="text-gray-400 text-sm">Damage Healed</p>
                            <p className="text-xl font-bold text-green-400">{game?.player_state.damage_healed.toLocaleString()}</p>
                        </div>
                        <div>
                            <p className="text-gray-400 text-sm">Damage Blocked</p>
                            <p className="text-xl font-bold text-blue-400">{game?.player_state.damage_blocked.toLocaleString()}</p>
                        </div>
                        <div>
                            <p className="text-gray-400 text-sm">Damage Dodged</p>
                            <p className="text-xl font-bold text-purple-400">{game?.player_state.damage_dodged.toLocaleString()}</p>
                        </div>
                    </div>
                </div>

                {/* Enemy */}
                <div className="flex gap-6">
                    <div className="bg-gray-900 rounded-xl border border-gray-700 p-6 flex-1 self-start">
                        <h2 className="text-gray-400 uppercase text-xs font-semibold mb-4">Enemy</h2>
                        <div className="grid grid-cols-2 gap-4">
                            <div>
                                <p className="text-gray-400 text-sm">Type</p>
                                <p className="text-xl font-bold">{game?.enemy_state.enemy_type}</p>
                            </div>
                            <div>
                                <p className="text-gray-400 text-sm">Health</p>
                                <p className="text-xl font-bold">{game?.enemy_state.health.current} / {game?.enemy_state.health.max}</p>
                            </div>
                        </div>
                    </div>

                    {game?.enemies_defeated && game?.enemies_defeated.size > 0 &&(
                        <div className="bg-gray-900 rounded-xl border border-gray-700 p-6 flex-1 self-start">
                            <h2 className="text-gray-400 uppercase text-xs font-semibold mb-4">Enemies Defeated</h2>

                            <div className="flex flex-col gap-2 overflow-y-auto max-h-40">
                                {Object.entries(game?.enemies_defeated).map(([enemy, count]) => (
                                    <div key={enemy} className="flex justify-between">
                                        <p className="text-gray-300">{enemy}</p>
                                        <p className="font-bold">x{count}</p>
                                    </div>
                                ))}
                            </div>
                        </div>
                    )}
                </div>
            </div>

            <a onClick={() => navigate("/dashboard")} className="fixed bottom-8 left-8 px-6 py-3 bg-gray-800 hover:bg-gray-700
            rounded-lg text-base transition-colors cursor-pointer"
            >Back</a>

            {!game?.complete && <a onClick={() => navigate(`/game_session?game_id=${game?.id}`)} className="fixed bottom-8 right-8 px-6
            py-3 bg-yellow-400 text-gray-950 font-bold hover:bg-yellow-300 rounded-lg
            text-base transition-colors cursor-pointer"
            >Continue Game</a>}
        </div>
    );
}
