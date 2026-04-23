import { useState, useEffect, useRef } from "react";
import { apiClient } from "../../axiosConfig";
import {Game, Action} from "../../models/game";

const BASE_URL = import.meta.env.VITE_SERVER_BASE_URL ?? "http://localhost:5000";

export default function Page() {
    const gameIdRef = useRef<string | null>(null);
    const esRef = useRef<EventSource | null>(null);

    const [game, setGame] = useState<Game | null>(null);
    const [status, setStatus] = useState<'starting' | 'running' | 'stopped' | 'error'>('starting');
    const [error, setError] = useState<string | null>(null);

    async function sendAction(action: Action) {
      await apiClient.post(`/games/${gameIdRef.current}/actions`, action);
    }

    async function stopGame() {
      esRef.current?.close();
      await apiClient.post(`/games/${gameIdRef.current}/stop`);
      setStatus('stopped');
    }

    useEffect(() => {
      const game_id = new URLSearchParams(window.location.search).get('game_id');
      gameIdRef.current = game_id;

      apiClient.post(`/games/${game_id}`, { })
        .then(() => {
          const es = new EventSource(BASE_URL + `/games/${game_id}/stream`);
          esRef.current = es;

          es.addEventListener('snapshot', (e) => {
            setGame(JSON.parse(e.data));
            setStatus('running');
          });

          es.onmessage = (e) => {
            setGame(JSON.parse(e.data));
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
        apiClient.post(`/games/${game_id}/stop`).catch(() => {});
      };
    }, []);

    if (status === 'starting') return <p>Starting game…</p>;

    if (status === 'error') return (
      <div>
        <p>Error: {error}</p>
        <a href="/">Back</a>
      </div>
    );

    if (status === 'stopped') return (
      <div>
        <p>{game?.win === true ? 'You win!' : game?.win === false ? 'You lose.' : 'Game stopped.'}</p>
        <a href="/">Back</a>
      </div>
    );

    if (!game) return <p>Starting game…</p>;

    const { player_state, enemy_state } = game;

    function promptAction(type: 'Attack' | 'Defend' | 'Heal') {
      const val = prompt(`${type} value:`);
      if (val === null) return;
      sendAction({ [type]: parseInt(val, 10) } as Action);
    }

    return (
      <div>
        <h2>Round {game.round} — Turn {game.turn}</h2>

        <section>
          <h3>Enemy ({enemy_state.enemy_type})</h3>
          <p>HP: {enemy_state.health.current}/{enemy_state.health.max} | Block: {enemy_state.block}</p>
        </section>

        <section>
          <h3>Player</h3>
          <p>HP: {player_state.health.current}/{player_state.health.max} | Block: {player_state.block}</p>
        </section>

        <div>
          <button onClick={() => promptAction('Attack')}>Attack</button>
          <button onClick={() => promptAction('Defend')}>Defend</button>
          <button onClick={() => promptAction('Heal')}>Heal</button>
          <button onClick={() => sendAction("None")}>None</button>
          <button onClick={stopGame}>Stop Game</button>
        </div>
    </div>
  );
}
