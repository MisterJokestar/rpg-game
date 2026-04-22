import {Game} from "../../models/game";

const characters = [
    {
        id: "char-001",
        name: "Aria",
        stats: { power: 85, speed: 70, defense: 55 },
        games: ["game-001"]
    },
    {
        id: "char-002",
        name: "Rex",
        stats: { power: 92, speed: 60, defense: 78 },
        games: ["game-002"]
    }
];

const user = {
    id: "user-001",
    username: "SpongeBob",
    characters: characters
};

const games = [
    {
        id: "game-001",
        complete: true,
        win: true,
        round: 12,
        turn: 47,
        player_state: {
            player_id: "user-001",
            character_id: "char-001",
            next_turn: null,
            health: { current: 45, max: 100 },
            block: 0,
            damage_taken: 320,
            damage_healed: 80,
            damage_blocked: 150,
            damage_dodged: 60,
            damage_dealt: 874,
        },
        enemy_state: {
            next_turn: null,
            state: 0,
            health: { current: 0, max: 100 },
            block: 0,
            enemy_type: "Dummy"
        }
    },
    {
        id: "game-002",
        complete: true,
        win: false,
        round: 8,
        turn: 31,
        player_state: {
            player_id: "user-001",
            character_id: "char-002",
            next_turn: null,
            health: { current: 0, max: 120 },
            block: 0,
            damage_taken: 510,
            damage_healed: 30,
            damage_blocked: 90,
            damage_dodged: 20,
            damage_dealt: 620,
        },
        enemy_state: {
            next_turn: null,
            state: 0,
            health: { current: 38, max: 100 },
            block: 0,
            enemy_type: "Dummy"
        }
    }
];
import { usePageContext} from "vike-react/usePageContext";
import { useState, useEffect } from "react";
import axios from "axios";

// Function to make the call to the cloud function for getting the game
async function get_game(gameId: string) {
    let response = await axios.post(
        'https://get-game-91972588391.us-central1.run.app',
        {
            gameId: gameId
        }
    );
    console.log(response.data);
    return response.data as Game
}


export default function Page() {
    const [gameStats, setGameStats] = useState<Game | null>(null);

    let tId: string = "eqDLstN542w62urbJDMm";


    useEffect(() => {
        const fetchData = async () => {
            const id = new URLSearchParams(window.location.search).get("game-id");
            if (id) {
                setGameStats(await get_game(id));
            }

        }
        fetchData();
    }, []);

    console.log(gameStats)

    return (
        <div className="min-h-screen bg-gray-950 text-white flex items-center justify-center">
            // <p className="text-gray-400">Under Construction :)</p>
            // </div>
    );
}

    //const game = games.find(g => g.id === gameId);
    //const character = game ? characters.find(
    //    c => c.id === game.player_state.character_id) : null;
    //const player_state = game?.player_state;
    //const enemy_state = game?.enemy_state;

//     if (!gameId){
//         return (
//             <div className="min-h-screen bg-gray-950 text-white flex items-center justify-center">
//                 <p className="text-gray-400">No game ID provided.</p>
//             </div>
//         )
//     }
//
//
//     if (!game){
//         return (
//             <div className="h-screen bg-gray-950 text-white flex items-senter justify-center">
//                 <p className="text-gray-400">Game not found.</p>
//             </div>
//         );
//     }
//     if (false) return (
//         <div className="min-h-screen bg-gray-950 text-white p-8">
//             <h1 className="text-4xl font-bold text-center mb-2">Game Stats</h1>
//             <p className="text-center text-gray-400 mb-8">Game ID: {game.id}</p>
//
//             {/* Result */}
//             {game.complete && (
//                 <div className={`text-center text-2xl font-bold mb-8 ${game.win ? "text-green-400" :
//                 "text-red-400"}`}>
//                     {game.win ? "Victory" : "Defeat"}
//                 </div>
//             )}
//
//             <div className="max-w-2xl mx-auto flex flex-col gap-6">
//
//                 {/* Character */}
//                 <div className="bg-gray-900 rounded-xl border border-gray-700 p-6">
//                     <h2 className="text-gray-400 uppercase text-xs font-semibold mb-4">Character</h2>
//                     <p className="text-xl font-bold">{character ? character.name : "Uknown"}</p>
//                     <p className="text-gray-400 text-sm mt-1">Round {game.round} • Turn {game.turn}</p>
//                     {character && (
//                         <div className="grid grid-cols-3 gap-4 mt-4">
//                             <div>
//                                 <p className="text-gray-400 text-sm">Power</p>
//                                 <p className="text-xl font-bold text-red-400">{character.stats.power}</p>
//                             </div>
//                             <div>
//                                 <p className="text-gray-400 text-sm">Defense</p>
//                                 <p className="text-xl font-bold text-blue-400">{character.stats.defense}</p>
//                             </div>
//                             <div>
//                                 <p className="text-gray-400 text-sm">Speed</p>
//                                 <p className="text-xl font-bold text-green-400">{character.stats.speed}</p>
//                             </div>
//                         </div>
//                     )}
//                 </div>
//
//                 {/* Health */}
//                 <div className="bg-gray-900 rounded-xl border border-gray-700 p-6">
//                     <h2 className="text-gray-400 uppercase text-xs font-semibold mb-4">Health</h2>
//                     <p className="text-xl font-bold">{player_state!.health.current} / {player_state!.health.max}</p>
//                 </div>
//
//                 {/* Combat Stats */}
//                 <div className="bg-gray-900 rounded-xl border border-gray-700 p-6">
//                     <h2 className="text-gray-400 uppercase text-xs font-semibold mb-4">Combat Stats</h2>
//                     <div className="grid grid-cols-2 gap-4">
//                         <div>
//                             <p className="text-gray-400 text-sm">Damage Dealt</p>
//                             <p className="text-xl font-bold text-red-400">{player_state!.damage_dealt.toLocaleString()}</p>
//                         </div>
//                         <div>
//                             <p className="text-gray-400 text-sm">Damage Taken</p>
//                             <p className="text-xl font-bold text-orange-400">{player_state!.damage_taken.toLocaleString()}</p>
//                         </div>
//                         <div>
//                             <p className="text-gray-400 text-sm">Damage Healed</p>
//                             <p className="text-xl font-bold text-green-400">{player_state!.damage_healed.toLocaleString()}</p>
//                         </div>
//                         <div>
//                             <p className="text-gray-400 text-sm">Damage Blocked</p>
//                             <p className="text-xl font-bold text-blue-400">{player_state!.damage_blocked.toLocaleString()}</p>
//                         </div>
//                         <div>
//                             <p className="text-gray-400 text-sm">Damage Dodged</p>
//                             <p className="text-xl font-bold text-purple-400">{player_state!.damage_dodged.toLocaleString()}</p>
//                         </div>
//                     </div>
//                 </div>
//
//                 {/* Enemy */}
//                 <div className="bg-gray-900 rounded-xl border border-gray-700 p-6">
//                     <h2 className="text-gray-400 uppercase text-xs font-semibold mb-4">Enemy</h2>
//                     <div className="grid grid-cols-2 gap-4">
//                         <div>
//                             <p className="text-gray-400 text-sm">Type</p>
//                             <p className="text-xl font-bold">{enemy_state!.enemy_type}</p>
//                         </div>
//                         <div>
//                             <p className="text-gray-400 text-sm">Health</p>
//                             <p className="text-xl font-bold">{enemy_state!.health.current} / {enemy_state!.health.max}</p>
//                         </div>
//                     </div>
//                 </div>
//             </div>
//
//             <a href="/" className="fixed bottom-8 left-8 px-6 py-3 bg-gray-800 hover:bg-gray-700
//             rounded-lg text-base transition-colors"
//             >Back</a>
//         </div>
//     );
// }

// May need this later for using cloud fn to get data
//import axios from "axios";
// export default function Page() {
//     async function makeUser() {
//         let response = await axios.post(
//             'https://create-user-91972588391.us-central1.run.app/',
//             {
//                 "username": "testuser",
//                 "password": "password123"
//             }
//         );
//         console.log(response);
//     }