export default function Page() {
    return (
        <div className="min-h-screen bg-gray-950 text-white flex flex-col items-center justify-center p-8">
            <h1 className="text-4xl font-bold text-center mb-2">Welcome</h1>
            <p className="text-center text-gray-400 mb-8">Ready to play?</p>

            <a href="/login" className="px-8 py-4 bg-yellow-400 text-gray-950 font-bold
            rounded-lg hover:bg-yellow-300 transition-colors">Login</a>
        </div>
    );
}