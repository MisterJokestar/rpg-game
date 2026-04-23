import { useState } from "react";
import axios from "axios";

// Function to make the call to the cloud function for getting the game
async function login(username: String, password: String) {
    let response = await axios.post(
        'https://cloud-functions-91972588391.us-central1.run.app/login', // the URL to the cloud function
        {
            "username": username,
            "password": password
        });
    return response;
}

export default function Page() {
    const [username, setUsername] = useState(""); // entered username stored in username
    const [password, setPassword] = useState(""); // entered password stored in password
    const [error, setError] = useState<string | null>(null);

    // I think this needs to be async???????????
    const handleSubmit =  async (e: React.SubmitEvent<HTMLFormElement>) => {
        e.preventDefault() // prevents the page from refreshing which could lose the data
        console.log({ username, password})
        if (!username || !password){
            setError("Please enter a username and password");
            return;
        }

        const response = await login(username, password);
        console.log(response.data);

        if (response.success) {
            /* returned json package
            return res.status(HTTP_STATUS.OK).json({
                userId: userDoc.id,
                secret: newSecret,
                success: true
            });
            */
            // save secret value and user id
            response.secret;
            response.userId;
        } else
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
                               placeholder="Enter your password"/>
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