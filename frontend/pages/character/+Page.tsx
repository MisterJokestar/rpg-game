import { useEffect, useRef, useState } from "react";
import { Character } from "../../models/game";
import { apiClient } from "../../axiosConfig";
import characterImg from "../../assets/Character.png";
import { navigate } from "vike/client/router";
import { CreateCharacterRequest, CreateGameRequest } from "../../models/api";

export default function Page() {
    const characterIdRef = useRef<String | null>(null);
    const [character, setCharacter] = useState<Character | undefined>(undefined);
    const [totalPoints, setTotalPoints] = useState<number>(0);
    const [unusedPoints, setUnusedPoints] = useState<number>(0);
    const [newCharacter, setNewCharacter] = useState<boolean>(false);

    async function retrieve_data() {
        let user_id = localStorage.getItem("userId");
        let secret = localStorage.getItem("secret");
        let character_id = new URLSearchParams(window.location.search).get('character_id');
        
        if (character_id) {
            characterIdRef.current = character_id;
            await grab_character(user_id, secret);
        } else {
            setNewCharacter(true);
            setTotalPoints(18);
            setUnusedPoints(15);
            let new_character: Character = {
                id: "",
                owner: user_id ? user_id : "",
                name: "",
                stats: {
                    power: 1,
                    speed: 1,
                    defense: 1
                },
                games: []
            }
            setCharacter(new_character);
        }
    }

    useEffect(() => {
        retrieve_data();
    }, []);

    async function grab_character(user_id: string | null, secret: string | null) {
        try {
            if (user_id && secret) {
                let response = await apiClient.get(
                    `/character/${characterIdRef.current}`
                );
                let res_character: Character = response.data.character;
                setCharacter(res_character);
                let tp: number = res_character.stats.speed + res_character.stats.power + res_character.stats.defense;
                setTotalPoints(tp);
            }
        } catch (error) {
            console.log(error);
        }
    }

    async function new_character() {
        let user_id = localStorage.getItem("userId");
        let secret = localStorage.getItem("secret");

        try {
            if (user_id && secret && character) {
                let request: CreateCharacterRequest = {
            	    player_id: user_id,
            	    character_name: character.name,
            	    power: character.stats.power,
            	    speed: character.stats.speed,
            	    defense: character.stats.defense
                }
                let response = await apiClient.post(
                    '/character/new',
                    request
                );
                characterIdRef.current = response.data.character_id;
                navigate("/dashboard");
            } else {
                console.log("ERROR: Missing User Id, Secret, or Character.");
            }
        } catch (error) {
            console.log(error);
        }
    }

    async function update_character() {
        let user_id = localStorage.getItem("userId");
        let secret = localStorage.getItem("secret");

        try {
            if (user_id && secret) {
                await apiClient.post(
                    '/character/update',
                    character
                );
            }
        } catch (error) {
            console.log(error);
        }
    }

    async function create_game() {
        let user_id = localStorage.getItem("userId");
        let secret = localStorage.getItem("secret");

        try {
            if (user_id && secret && character) {
                let request: CreateGameRequest = {
                    player_id: user_id,
                    character_id: character.name
                }
                let response = await apiClient.post(
                    '/game/new',
                    request
                );
                let game_id = response.data.game_id;
                navigate(`/game_session?game_id=${game_id}`);
            }
        } catch (error) {
            console.log(error);
        }

    }

    function adjustStat(stat: 'power' | 'speed' | 'defense', delta: number) {
        if (!character) return;
        const current = character.stats[stat];
        if (current + delta < 1) return;
        if (delta > 0 && unusedPoints <= 0) return;
        setCharacter({ ...character, stats: { ...character.stats, [stat]: current + delta } });
        setUnusedPoints(p => p - delta);
    }

    return (
      <div className="min-h-screen bg-gray-950 text-white flex flex-col items-center justify-center p-8 gap-10">
          <div className="w-full max-w-2xl">
              <h1 className="text-4xl font-bold text-center mb-8">
                  {newCharacter ? "Create Character" : "Edit Character"}
              </h1>

              {/* Name + Hero image + Stats row */}
              <div className="flex gap-8 items-start bg-gray-900 rounded-2xl p-6">

                  {/* Left: hero image */}
                  <div className="flex-shrink-0 flex flex-col items-center gap-3">
                      <img src={characterImg} alt="Hero" className="w-40 h-40 object-contain rounded-xl bg-gray-800 p-2" />
                      <div className="text-sm text-gray-400">
                          Points remaining: <span className="text-yellow-400 font-bold">{unusedPoints}</span>
                      </div>
                  </div>

                  {/* Right: name + stats */}
                  <div className="flex flex-col gap-5 flex-grow">
                      {/* Name */}
                      <div>
                          <label className="text-sm text-gray-400 mb-1 block">Name</label>
                          <input
                              className="w-full bg-gray-800 rounded-lg px-4 py-2 text-white focus:outline-none focus:ring-2 focus:ring-indigo-500"
                              value={character?.name ?? ""}
                              onChange={e => character && setCharacter({ ...character, name: e.target.value })}
                              placeholder="Enter name..."
                          />
                      </div>

                      {/* Stats */}
                      {(['power', 'speed', 'defense'] as const).map(stat => (
                          <div key={stat} className="flex items-center gap-4">
                              <span className="w-20 capitalize text-gray-300">
                                  {stat === 'power' ? '⚔️' : stat === 'speed' ? '💨' : '🛡️'} {stat}
                              </span>
                              <button
                                  onClick={() => adjustStat(stat, -1)}
                                  disabled={!character || character.stats[stat] <= 1}
                                  className="w-8 h-8 rounded-lg bg-gray-700 hover:bg-gray-600 disabled:opacity-30 font-bold text-lg"
                              >−</button>
                              <span className="w-8 text-center font-bold text-xl">
                                  {character?.stats[stat] ?? 1}
                              </span>
                              <button
                                  onClick={() => adjustStat(stat, 1)}
                                  disabled={unusedPoints <= 0}
                                  className="w-8 h-8 rounded-lg bg-gray-700 hover:bg-gray-600 disabled:opacity-30 font-bold text-lg"
                              >+</button>
                              {/* Visual bar */}
                              <div className="flex-grow bg-gray-800 rounded-full h-2">
                                  <div
                                      className="bg-indigo-500 h-2 rounded-full transition-all"
                                      style={{ width: `${((character?.stats[stat] ?? 1) / totalPoints) * 100}%` }}
                                  />
                              </div>
                          </div>
                      ))}

                      {/* Submit */}
                      <button
                          disabled={unusedPoints > 0}
                          onClick={newCharacter ? new_character : update_character}
                          className="mt-2 w-full py-3 bg-indigo-600 hover:bg-indigo-500 rounded-xl font-semibold text-lg transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
                      >
                          {newCharacter ? "Create Character" : "Save Changes"}
                      </button>
                  </div>
              </div>

              {/* Games list */}
              {!newCharacter && (
                  <div className="mt-8">
                      <div className="flex items-center justify-between mb-4">
                          <h2 className="text-2xl font-bold">Games</h2>
                          <button
                              onClick={create_game}
                              className="bg-green-700 hover:bg-green-600 px-4 py-2 rounded-xl font-semibold transition-colors"
                          >
                              + New Game
                          </button>
                      </div>
                      {character?.games.length === 0 ? (
                          <p className="text-gray-500 text-center py-6">No games yet. Start one!</p>
                      ) : (
                          <ul className="space-y-3">
                              {character?.games.map(gameId => (
                                  <li
                                      key={gameId}
                                      onClick={() => navigate(`/game_stats?game-id=${gameId}`)}
                                      className="bg-gray-800 hover:bg-gray-700 rounded-xl px-5 py-4 cursor-pointer transition-colors"
                                  >
                                      <span className="text-gray-400 text-sm">Game ID: </span>
                                      <span className="font-mono text-sm">{gameId}</span>
                                  </li>
                              ))}
                          </ul>
                      )}
                  </div>
              )}
          </div>
      </div>
  );
}
