# Frontend

React + [Vike](https://vike.dev/) (SSR) frontend for the RPG Game. Built with Vite and styled with Tailwind CSS 4.

## Tech Stack

| Library | Version | Purpose |
|---|---|---|
| React | 19 | UI component library |
| Vike | latest | File-system routing + SSR on top of Vite |
| Vite | latest | Build tool and dev server |
| Tailwind CSS | 4 | Utility-first styling |
| Axios | latest | HTTP client (pre-configured in `axiosConfig.tsx`) |
| TypeScript | 6 | Static types |

## Setup

```bash
cd frontend
npm install
npm run dev
```

The dev server listens on port `3000` by default.

## Environment Variables

Copy the example file and fill in your values:

```bash
cp .env.example .env
```

| Variable | Default | Description |
|---|---|---|
| `PORT` | `3000` | Port the Vite/Node server listens on |
| `VITE_SERVER_BASE_URL` | `http://localhost:5000` | Base URL the frontend uses to reach the backend API |

`VITE_SERVER_BASE_URL` is embedded into the client bundle at build time; it must point to wherever the Axum server is running.

## Page / Route Map

| Route | File | Purpose |
|---|---|---|
| `/` | `pages/index/+Page.tsx` | Landing page — redirects authenticated users to `/dashboard` |
| `/login` | `pages/login/+Page.tsx` | Sign-in form |
| `/create_account` | `pages/create_account/+Page.tsx` | New account registration form |
| `/dashboard` | `pages/dashboard/+Page.tsx` | Character list for the logged-in user |
| `/character` | `pages/character/+Page.tsx` | Create a new character (no `?character_id`) or view/edit an existing one |
| `/game_session` | `pages/game_session/+Page.tsx` | Live turn-based combat screen (SSE-driven) |
| `/game_stats` | `pages/game_stats/+Page.tsx` | Post-game summary for a completed or in-progress game |
| `/leaderboard` | `pages/leaderboard/+Page.tsx` | Global leaderboard with multi-criteria sorting |
| `/_error` | `pages/_error/+Page.tsx` | 404 / internal-error fallback page |

## Authentication

Auth is stored in `localStorage` after a successful `/login` or `/create_account` call:

| Key | Value |
|---|---|
| `userId` | UUID v7 string identifying the logged-in user |
| `secret` | Session secret rotated on every login; must be sent with every authenticated request |
| `username` | Display name shown in the UI (not sent to the server after login) |

The Axios interceptor in `axiosConfig.tsx` automatically reads `userId` and `secret` from `localStorage` and appends `Authorization: <userId>:<secret>` to every outgoing request.

## Real-Time Updates (SSE)

The game session page opens a native `EventSource` connection to:

```
GET /session/:game_id/stream
```

Events received:

| Event name | Shape | Meaning |
|---|---|---|
| `snapshot` | `Game` JSON | Initial game state sent immediately on connection |
| *(default)* | `{ TurnResolved: Game }` | A turn resolved; update the board |
| *(default)* | `{ GameOver: Game }` | Game ended naturally |
| *(default)* | `{ GameStopped: Game }` | Game was stopped explicitly |
| *(default)* | `{ GameMessage: string }` | Narrative message from the game engine |
| `lag` | number | Client fell behind the broadcast buffer |
