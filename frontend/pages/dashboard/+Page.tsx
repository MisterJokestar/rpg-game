import { useEffect, useState } from "react"
import { Character } from "../../models/game"
import { cloudFunctions } from "../../axiosConfig";
import { navigate } from "vike/client/router";

export default function Page() {
    const [username, setUsername] = useState<String | null>(null);
    const [characters, setCharacters] = useState<Character[]>([]);

    async function retrieve_data() {
        let user_id = localStorage.getItem("userId");
        let user_name = localStorage.getItem("username");
        let secret = localStorage.getItem("secret");

        if (user_name) {
            setUsername(user_name);
        }

        try {
            if (user_id && secret) {
                let response = await cloudFunctions.post(
                    '/getCharacters',
                    {
                        "userId": user_id,
                        "secret": secret
                    }
                );
                let res_characters: Character[] = response.data.characters;
                setCharacters(res_characters);
            }
        } catch (error) {
            console.log(error);
        }
        
    }

    useEffect(() => {
       retrieve_data(); 
    }, []);

    return (
        <div className="min-h-screen bg-gray-950 text-white flex items-center justify-center p-8">
            <div className="w-full max-w-md">
                <h1 className="text-4xl font-bold text-center mb-2">Hello {username}!</h1>
                <ul className="mt-6 space-y-3">
                    {characters.map((c) => (
                        <li 
                            key={c.characterId}
                            className="bg-gray-800 rounded-xl p-4 flex flex-col gap-2"
                            onClick={() => {navigate(`/character?character_id=${c.characterId}`);}}
                        >
                            <span className="text-xl font-semibold">{c.name}</span>
                            <div className="flex gap-4 text-sm text-gray-300">
                                <span>⚔️ Power: <span className="text-white font-medium">{c.stats.power}</span></span>
                                <span>🛡️ Defense: <span className="text-white font-medium">{c.stats.defense}</span></span>
                                <span>💨 Speed: <span className="text-white font-medium">{c.stats.speed}</span></span>
                            </div>
                        </li>
                    ))}
                    <button className="text-xl bg-gray-800 rounded-xl p-4" onClick={() => {navigate("/character")}}>
                        Create New Character 
                    </button>
                    <button className="text-xl bg-gray-800 rounded-xl p-4 mx-4" onClick={() => {navigate("/leaderboard")}}>
                        View Leaderboard
                    </button>
                </ul>
            </div>
        </div>
    )
}
