import { useState } from "react";
import axios from "axios";

// Function to make the call to the cloud function for getting the game
async function login() {
    let response = await axios.post(
        '', // the URL to the cloud function
        {
            // any args go here, likely username and password
        }
    );
    console.log(response.data);
}

export default function Page() {
    const [username, setUsername] = useState(""); // entered username stored in username
    const [password, setPassword] = useState(""); // entered password stored in password
    const [error, setError] = useState<string | null>(null);

    const handleSubmit = (e: React.SubmitEvent<HTMLFormElement>) => {
        e.preventDefault() // prevents the page from refreshing which could lose the data
        console.log({ username, password})
        if (!username || !password){
            setError("Please enter a username and password");
            return;
        }

        //TODO: login logic would go here, likely call cloud function here

        if (true){ // If the username or password is invalid, likely if (response.status == 401)
            setError("Incorrect username or password");
            return;
        }
    }

    return (
        <div className="min-h-screen bg-gray-950 text-white flex items-center justify-center p-8">
            <div className="w-full max-w-md">
                <h1 className="text-4xl font-bold text-center mb-2">Sign in</h1>

                <form onSubmit={handleSubmit} className="bg-gray-900 rounded-xl border
                    border-gray-700 p-8 flex flex-col gap-6">

                    <div>
                        <label className="text-gray-400 uppercase text-xs font-semibold
                            mb-2 block">Username</label>
                        <input type="text"
                               value={username}
                               onChange={e => setUsername(e.target.value)}
                               className="w-full bg-gray-800 border border-gray-700 rounded-lg
                                    px-4 py-3 text-white focus:outline-none focus:border-yellow-300 transition-colors"
                                    placeholder="Enter your username"/>
                    </div>

                    <div>
                        <label className="text-gray-400 uppercase text-xs font-semibold
                            mb-2 block">Password</label>
                        <input type="password"
                               value={password}
                               onChange={e => setPassword(e.target.value)}
                               className="w-full bg-gray-800 border border-gray-700 rounded-lg
                                    px-4 py-3 text-white focus:outline-none focus:border-yellow-300 transition-colors"
                               placeholder="Enter your username"/>
                    </div>

                    {error && (
                        <p className="text-red-400 text-sm">{error}</p>
                    )}

                    <button type="submit"
                            className="w-full bg-yellow-400 text-gray-950 font-bold py-3
                                rounded-lg hover:bg-yellow-300 transition-colors">Sign in
                    </button>
                </form>
            </div>

            <a href="/" className="fixed bottom-8 left-8 px-6 py-3 bg-gray-800 hover:bg-gray-700
                rounded-lg text-base transition colors">Back
            </a>

            <div className="fixed bottom-8 right-8 flex flex-col items-end gap-2">
                <a href="/create_account" className="px-6 py-3 bg-gray-800 hover:bg-gray-700
                    rounded-lg text-base transition-colors">
                    Create Account
                </a>
            </div>
        </div>
    );
}