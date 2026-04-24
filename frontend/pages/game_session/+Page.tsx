import { useState, useEffect, useRef } from "react";
import { apiClient, cloudFunctions } from "../../axiosConfig";
import { Game, Action, Character } from "../../models/game";
import characterImg from "../../assets/Character.png";
import copperSidesImg from "../../assets/CopperSides.png";
import roseBuddiesImg from "../../assets/RoseBuddies.png";

const BASE_URL = import.meta.env.VITE_SERVER_BASE_URL ?? "http://localhost:5000";

function getEnemyImage(enemy_type: string): string {
  if (enemy_type === 'CopperSides') return copperSidesImg;
  if (enemy_type === 'RoseBuddies') return roseBuddiesImg;
  return characterImg;
}

function isPlayerTurn(game: Game): boolean {
  const pt = game.player_state.next_turn;
  const et = game.enemy_state.next_turn;
  if (pt === null || et === null) return true;
  return pt <= et;
}

function HealthBar({ current, max }: { current: number; max: number }) {
  const pct = max > 0 ? (current / max) * 100 : 0;
  const color = pct > 50 ? 'bg-green-500' : pct > 25 ? 'bg-yellow-500' : 'bg-red-500';
  return (
    <div className="w-full">
      <div className="flex justify-between text-sm text-gray-400 mb-1">
        <span>{current}</span>
        <span>{max}</span>
      </div>
      <div className="w-full bg-gray-700 rounded-full h-3">
        <div
          className={`${color} h-3 rounded-full transition-all duration-500`}
          style={{ width: `${pct}%` }}
        />
      </div>
    </div>
  );
}

export default function Page() {
  const gameIdRef = useRef<string | null>(null);
  const esRef = useRef<EventSource | null>(null);
  const stoppedRef = useRef<boolean>(false);

  const [game, setGame] = useState<Game | null>(null);
  const [character, setCharacter] = useState<Character | null>(null);
  const [status, setStatus] = useState<'starting' | 'running' | 'stopped' | 'error'>('starting');
  const [error, setError] = useState<string | null>(null);
  const [message, setMessage] = useState<string | null>(null);
  const [actionPending, setActionPending] = useState<boolean>(false);
  const messageTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  async function sendAction(action: Action) {
    setActionPending(true);
    await apiClient.post(`/games/${gameIdRef.current}/actions`, action);
  }

  async function stopGame() {
    stoppedRef.current = true;
    esRef.current?.close();
    await apiClient.post(`/games/${gameIdRef.current}/stop`);
    setStatus('stopped');
  }

  useEffect(() => {
    const game_id = new URLSearchParams(window.location.search).get('game_id');
    gameIdRef.current = game_id;

    apiClient.post(`/games/${game_id}`, {})
      .then(() => {
        const es = new EventSource(BASE_URL + `/games/${game_id}/stream`);
        esRef.current = es;

        es.addEventListener('snapshot', (e) => {
          const snapshotGame: Game = JSON.parse(e.data);
          setGame(snapshotGame);
          setStatus('running');

          const userId = localStorage.getItem("userId");
          const secret = localStorage.getItem("secret");
          const characterId = snapshotGame.player_state.character_id;
          if (userId && secret && characterId) {
            cloudFunctions.post('/getCharacter', { userId, secret, characterId })
              .then(res => setCharacter(res.data.character))
              .catch(err => console.warn('Failed to fetch character:', err));
          }
        });

        es.onmessage = (e) => {
          const payload = JSON.parse(e.data);
          if (payload.TurnResolved) {
            setGame(payload.TurnResolved);
            setActionPending(false);
          } else if (payload.GameOver) {
            setGame(payload.GameOver);
            setActionPending(false);
            setStatus('stopped');
            esRef.current?.close();
          } else if (payload.GameStopped) {
            setGame(payload.GameStopped);
            setActionPending(false);
            setStatus('stopped');
            esRef.current?.close();
          } else if (payload.GameMessage) {
            if (messageTimerRef.current) clearTimeout(messageTimerRef.current);
            setMessage(payload.GameMessage);
            messageTimerRef.current = setTimeout(() => setMessage(null), 2500);
          }
        };

        es.addEventListener('lag', (e) => {
          console.warn('lag event', e.data);
        });

        es.onerror = () => {
          setStatus('error');
          setError('SSE connection error');
        };
      })
      .catch((err) => {
        setStatus('error');
        setError(err.message ?? 'Failed to start game');
      });

    return () => {
      esRef.current?.close();
      if (!stoppedRef.current) {
        apiClient.post(`/games/${game_id}/stop`).catch(() => {});
      }
      if (messageTimerRef.current) clearTimeout(messageTimerRef.current);
    };
  }, []);

  if (status === 'starting' || !game) {
    return (
      <div className="min-h-screen bg-gray-950 text-white flex items-center justify-center">
        <p className="text-2xl text-gray-400 animate-pulse">Starting game…</p>
      </div>
    );
  }

  if (status === 'error') {
    return (
      <div className="min-h-screen bg-gray-950 text-white flex items-center justify-center p-8">
        <div className="bg-gray-900 rounded-2xl p-8 max-w-md w-full text-center space-y-4">
          <p className="text-red-400 text-xl font-semibold">Error</p>
          <p className="text-gray-300">{error}</p>
          <a href="/" className="inline-block mt-4 px-6 py-2 bg-gray-700 hover:bg-gray-600 rounded-xl transition-colors">
            Back
          </a>
        </div>
      </div>
    );
  }

  if (status === 'stopped') {
    const won = game.win === true;
    const lost = game.win === false;
    const enemiesDefeated = game.enemies_defeated instanceof Map
      ? Array.from(game.enemies_defeated.entries())
      : Object.entries(game.enemies_defeated as unknown as Record<string, number>);

    return (
      <div className="min-h-screen bg-gray-950 text-white flex items-center justify-center p-8">
        <div className="bg-gray-900 rounded-2xl p-10 max-w-md w-full text-center space-y-6">
          <div className="text-6xl">{won ? '🏆' : lost ? '💀' : '🛑'}</div>
          <h1 className="text-3xl font-bold">
            {won ? 'Victory!' : lost ? 'Defeated' : 'Game Stopped'}
          </h1>
          <div className="space-y-2 text-gray-300">
            <p>Round survived: <span className="text-white font-bold">{game.round}</span></p>
            {enemiesDefeated.length > 0 && (
              <div>
                <p className="text-sm text-gray-400 mb-1">Enemies defeated:</p>
                {enemiesDefeated.map(([type, count]) => (
                  <p key={type} className="text-sm">
                    {type}: <span className="text-white font-bold">{count}</span>
                  </p>
                ))}
              </div>
            )}
          </div>
          <a href="/" className="inline-block px-8 py-3 bg-indigo-600 hover:bg-indigo-500 rounded-xl font-semibold transition-colors">
            Back to Character
          </a>
        </div>
      </div>
    );
  }

  const { player_state, enemy_state } = game;
  const power = character?.stats.power ?? 1;
  const defense = character?.stats.defense ?? 1;
  const canAct = !actionPending && isPlayerTurn(game) && status === 'running';

  return (
    <div className="min-h-screen bg-gray-950 text-white flex flex-col items-center p-6 gap-6">

      {/* Header */}
      <div className="w-full max-w-3xl text-center">
        <h2 className="text-2xl font-bold text-gray-200">
          Round {game.round} — Turn {game.turn}
        </h2>
      </div>

      {/* Battle Arena */}
      <div className="w-full max-w-3xl bg-gray-900 rounded-2xl p-6">
        <div className="flex items-center justify-around gap-4">

          {/* Player Panel */}
          <div className="flex flex-col items-center gap-3 w-40">
            <span className="text-sm text-gray-400 font-semibold uppercase tracking-wide">Player</span>
            <img
              src={characterImg}
              alt="Player"
              className="w-32 h-32 object-contain rounded-xl bg-gray-800 p-2"
            />
            <HealthBar current={player_state.health.current} max={player_state.health.max} />
            {player_state.block > 0 && (
              <span className="bg-gray-700 text-blue-300 rounded-full px-3 py-1 text-sm">
                🛡 {player_state.block} block
              </span>
            )}
          </div>

          {/* Message Banner */}
          <div className="flex-1 flex items-center justify-center min-h-16">
            <div
              className={`text-yellow-400 font-bold text-xl text-center transition-opacity duration-700 ${message ? 'opacity-100' : 'opacity-0'}`}
            >
              {message ?? '\u00A0'}
            </div>
          </div>

          {/* Enemy Panel */}
          <div className="flex flex-col items-center gap-3 w-40">
            <span className="text-sm text-gray-400 font-semibold uppercase tracking-wide">
              {enemy_state.enemy_type}
            </span>
            <img
              src={getEnemyImage(enemy_state.enemy_type)}
              alt={enemy_state.enemy_type}
              className="w-32 h-32 object-contain rounded-xl bg-gray-800 p-2"
            />
            <HealthBar current={enemy_state.health.current} max={enemy_state.health.max} />
            {enemy_state.block > 0 && (
              <span className="bg-gray-700 text-blue-300 rounded-full px-3 py-1 text-sm">
                🛡 {enemy_state.block} block
              </span>
            )}
          </div>
        </div>

        {/* Turn indicator */}
        {actionPending && (
          <p className="text-center text-gray-400 text-sm mt-4 animate-pulse">Waiting for turn resolution…</p>
        )}
        {!actionPending && !isPlayerTurn(game) && status === 'running' && (
          <p className="text-center text-gray-400 text-sm mt-4 animate-pulse">Enemy is acting…</p>
        )}
      </div>

      {/* Action Buttons */}
      <div className="w-full max-w-3xl">
        <div className="grid grid-cols-2 gap-3">
          <button
            onClick={() => sendAction({ Attack: power })}
            disabled={!canAct}
            className="py-4 bg-red-700 hover:bg-red-600 rounded-xl font-semibold text-lg transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
          >
            ⚔️ Attack <span className="text-sm text-red-300">({power})</span>
          </button>
          <button
            onClick={() => sendAction({ Defend: defense })}
            disabled={!canAct}
            className="py-4 bg-blue-700 hover:bg-blue-600 rounded-xl font-semibold text-lg transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
          >
            🛡️ Defend <span className="text-sm text-blue-300">({defense})</span>
          </button>
          <button
            onClick={() => sendAction({ Heal: defense })}
            disabled={!canAct}
            className="py-4 bg-green-700 hover:bg-green-600 rounded-xl font-semibold text-lg transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
          >
            💚 Heal <span className="text-sm text-green-300">({defense})</span>
          </button>
          <button
            onClick={() => sendAction("None")}
            disabled={!canAct}
            className="py-4 bg-gray-700 hover:bg-gray-600 rounded-xl font-semibold text-lg transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
          >
            ⏸ None
          </button>
        </div>
      </div>

      {/* Stop Game */}
      <button
        onClick={stopGame}
        className="text-sm text-gray-500 hover:text-red-400 transition-colors underline"
      >
        Stop Game
      </button>
    </div>
  );
}
