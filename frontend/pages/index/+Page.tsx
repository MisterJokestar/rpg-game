import { navigate } from "vike/client/router";

export default function Page() {
    return (
        <div className="min-h-screen bg-gray-950 text-white flex flex-col items-center justify-center p-8">
            <h1 className="text-4xl font-bold text-center mb-2">Welcome</h1>
            <p className="text-center text-gray-400 mb-8">Ready to play?</p>

            <div className="flex flex-col items-center gap-4">
                <div className="flex gap-4">
                    <a onClick={() => navigate("/login")} className="px-8 py-4 bg-yellow-400 text-gray-950 font-bold
                        rounded-lg hover:bg-yellow-300 transition-colors cursor-pointer">Login</a>
                    <a onClick={() => navigate("/create_account")} className="px-8 py-4 bg-yellow-400 text-gray-950 font-bold
                        rounded-lg hover:bg-yellow-300 transition-colors cursor-pointer">Sign Up</a>
                </div>
                <a onClick={() => navigate("/leaderboard")} className="px-6 py-3 bg-gray-800 hover:bg-gray-700
                    rounded-lg text-base transition-colors cursor-pointer">Leaderboard</a>
            </div>
        </div>
    );
}