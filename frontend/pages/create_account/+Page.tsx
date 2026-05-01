import { ChangeEvent, useState } from "react";
import { navigate } from "vike/client/router";
import { apiClient } from "../../axiosConfig";
import { CreateUserRequest, LogInResponse } from "../../models/api";

export default function Page() {
    const [username, setUsername] = useState<string>(""); // entered username stored in username
    const [password, setPassword] = useState<string>(""); // entered password stored in password
    const [loading, setLoading] = useState<boolean>(false);
    const [error, setError] = useState<string | null>(null);

    // I think this needs to be async??????????? <- Yes that is correct.
    const handleSubmit =  async (e: React.SubmitEvent<HTMLFormElement>) => {
        e.preventDefault() // prevents the page from refreshing which could lose the data
        setLoading(true); // prevents the button getting hit twice
        console.log({ username, password})
        if (!username || !password){
            setError("Please enter a username and password");
            return;
        }
        try {
            let request: CreateUserRequest = {
                username: username,
                password: password
            }
            let response = await apiClient.post(
                '/create_user', // the URL to the cloud function
                request
            );
            let data: LogInResponse = response.data;
            localStorage.setItem("secret", data.secret);
            localStorage.setItem("userId", data.user_id);
            localStorage.setItem("username", username);
            navigate("/dashboard"); // redirect to dashboard
        } catch (error) {
            setError("Incorrect username or password");
        } finally {
            setLoading(false);
        }
    }

    function handleUsernameChange(e: ChangeEvent<HTMLInputElement>) {
        let new_username = e.target.value;
        setUsername(new_username);
    }

    function handlePasswordChange(e: ChangeEvent<HTMLInputElement>) {
        let new_password = e.target.value;
        setPassword(new_password);
    }

    return (
        <div className="min-h-screen bg-gray-950 text-white flex items-center justify-center p-8">
            <div className="w-full max-w-md">
                <h1 className="text-4xl font-bold text-center mb-2">Sign up</h1>

                <form onSubmit={handleSubmit} className="bg-gray-900 rounded-xl border
                    border-gray-700 p-8 flex flex-col gap-6">

                    <div>
                        <label className="text-gray-400 uppercase text-xs font-semibold
                            mb-2 block">Username</label>
                        <input type="text"
                               value={username}
                               onChange={handleUsernameChange}
                               className="w-full bg-gray-800 border border-gray-700 rounded-lg
                                    px-4 py-3 text-white focus:outline-none focus:border-yellow-300 transition-colors"
                                    placeholder="Enter your username"/>
                    </div>

                    <div>
                        <label className="text-gray-400 uppercase text-xs font-semibold
                            mb-2 block">Password</label>
                        <input type="password"
                               value={password}
                               onChange={handlePasswordChange}
                               className="w-full bg-gray-800 border border-gray-700 rounded-lg
                                    px-4 py-3 text-white focus:outline-none focus:border-yellow-300 transition-colors"
                               placeholder="Enter your password"/>
                    </div>

                    {error && (
                        <p className="text-red-400 text-sm">{error}</p>
                    )}

                    <button type="submit"
                            className="w-full bg-yellow-400 text-gray-950 font-bold py-3
                                rounded-lg hover:bg-yellow-300 transition-colors"
                            disabled={loading}>
                                {loading ? "Loading" : "Sign up"}
                    </button>
                </form>
            </div>

            <a
                onClick={() => navigate(localStorage.getItem("userId") ? "/dashboard" : "/")}
                className="fixed bottom-8 left-8 px-6 py-3 bg-gray-800 hover:bg-gray-700
                rounded-lg text-base transition-colors cursor-pointer"> Home
            </a>

            <div className="fixed bottom-8 right-8 flex flex-col items-end gap-2">
                <a onClick={() => navigate("/login")} className="px-6 py-3 bg-gray-800 hover:bg-gray-700
                    rounded-lg text-base transition-colors cursor-pointer">
                    Log In
                </a>
            </div>
        </div>
    );
}

