# RPG Game | Expansion of Cloud Computing Final Project

A full-stack turn-based RPG web application with a Rust/Axum backend, React/Vike frontend, and dual database support (MongoDB for local development, Firestore for production).

Authors: Ryan Jobson, Sam Plemmons, James Wall

## Stack

| Layer | Technology |
|---|---|
| Frontend | React + Vike (SSR) + Tailwind CSS |
| Backend | Rust + Axum + Tokio |
| Database | MongoDB (local) / Firestore (production) |
| Auth | bcrypt password hashing + session secrets |
| IDs | UUID v7 |
| Serialisation | serde / serde_json |
| Async traits | async-trait |
| Async streams | futures |
| Random | rand |

---

## Prerequisites

**Required:**
- [Rust toolchain](https://rustup.rs/) (stable) — for the backend
- [Node.js](https://nodejs.org/) 22+ — for the frontend

**Optional:**
- [Docker](https://docs.docker.com/get-docker/) and Docker Compose — for containerised database and deployment

---

## Setup

### 1. Configure environment files

Copy the example env files and fill in your values:

```bash
cp server/.env.example server/.env
cp frontend/.env.example frontend/.env
```

Key variables in `server/.env`:

| Variable | Default | Description |
|---|---|---|
| `HOST` | `0.0.0.0` | Address the server binds to |
| `PORT` | `5000` | Backend server port |
| `MONGODB_URI` | `mongodb://localhost:27017` | MongoDB connection URI (mongodb feature) |
| `FIRESTORE_PROJECT_ID` | — | GCP project ID (firestore feature) |

Key variables in `frontend/.env`:

| Variable | Default | Description |
|---|---|---|
| `PORT` | `3000` | Frontend server port |
| `VITE_SERVER_BASE_URL` | `http://localhost:5000` | URL the frontend uses to reach the backend |

### 2. Database backend selection

The database backend is chosen at **compile time** via Cargo features, not at runtime:

```bash
# Local development — MongoDB
cargo run --features mongodb

# Production — Firestore
cargo run --features firestore
```

There is no default feature — you must explicitly select a backend when building.

---

## Docker Compose

There are three compose files, each serving a different purpose.

### Run everything (backend + database)

Starts MongoDB and the Rust server together on a shared Docker network:

```bash
docker compose -f docker-compose.backend.yml up
```

Add `--build` to rebuild images after code changes:

```bash
docker compose -f docker-compose.backend.yml up --build
```

### Run the frontend

```bash
docker compose -f docker-compose.frontend.yml up
```

> When running alongside `docker-compose.backend.yml`, the frontend container reaches the server via the `server` Docker service name (already configured). Both compose files share the `finalproj-network` network.

### Run the database only

Useful when running the backend server directly on the host (e.g. `cargo run`):

```bash
docker compose -f docker-compose.db.yml up
```

### Run everything at once

```bash
docker compose -f docker-compose.db.yml -f docker-compose.backend.yml -f docker-compose.frontend.yml up --build
```

### Stop and clean up

```bash
docker compose -f docker-compose.backend.yml down
# Add -v to also remove the named volume (deletes database data)
docker compose -f docker-compose.backend.yml down -v
```

---

## Local Development (without Docker)

### Backend

```bash
cd server
cargo run --features mongodb
```

Requires `server/.env` with `MONGODB_URI` set and a running MongoDB instance (use `docker-compose.db.yml`).

### Frontend

```bash
cd frontend
npm install
npm run dev
```

Requires `frontend/.env` with `VITE_SERVER_BASE_URL=http://localhost:5000`.

---

## Ports

| Service | Default Port |
|---|---|
| Frontend | 3000 |
| Backend | 5000 |
| MongoDB | 27017 |

---

## API Endpoints

### Authentication

| Method | Path | Auth | Description |
|---|---|---|---|
| POST | `/create_user` | No | Register a new user account |
| POST | `/login` | No | Authenticate and receive a fresh session secret |

### Characters

| Method | Path | Auth | Description |
|---|---|---|---|
| GET | `/character/all/:user_id` | Yes | List all characters belonging to a user |
| GET | `/character/:character_id` | Yes | Fetch a character by ID |
| POST | `/character/update` | Yes | Replace an existing character record |
| POST | `/character/new` | Yes | Create a new character |

### Games

| Method | Path | Auth | Description |
|---|---|---|---|
| GET | `/game/:game_id` | No | Fetch a game record by ID |
| GET | `/games` | No | List all game records |
| POST | `/game/:game_id` | Yes | Replace a game record |
| POST | `/game/new` | Yes | Create a new game |

### Sessions

| Method | Path | Auth | Description |
|---|---|---|---|
| POST | `/session/:game_id` | Yes | Start a live game session |
| POST | `/session/:game_id/stop` | Yes | Stop an active session |
| POST | `/session/:game_id/action` | Yes | Submit a player action |
| GET | `/session/:game_id/stream` | No | Subscribe to SSE game events |

---

## Authentication

Authenticated endpoints require an `Authorization` header in the format:

```
Authorization: <user_id>:<secret>
```

Both `user_id` and `secret` are returned by `/create_user` and `/login`. The secret is rotated on every successful login — clients must update their stored secret after each login call.

---

## Game Sessions

A game session is a live, in-memory combat loop tied to a persisted game record.

1. **Start** — `POST /session/:game_id` loads the game, spawns a runner task and a watcher task, and returns the current game state.
2. **Act** — `POST /session/:game_id/action` sends a JSON [`Action`] to the runner. Valid actions: `{"Attack": <power>}`, `{"Defend": <defense>}`, `{"Heal": <defense>}`, `"None"`.
3. **Stream** — `GET /session/:game_id/stream` opens an SSE connection. The first event is a `"snapshot"` of the current game state; subsequent events carry resolved turns, game-over notifications, and narrative messages.
4. **Stop** — `POST /session/:game_id/stop` cancels the session and persists the final game state.

Sessions time out automatically after **15 minutes** of inactivity.

---

## Enemy Types

| Enemy | HP | Description |
|---|---|---|
| Rose Buddies | 40 | Fast attacker that cycles attack×2 → heal. Thorns deal 1 damage when the player attacks. Speeds up below 20 HP. |
| Copper Sides | 50 | Alternates attack and defend. Gains +1 power each cycle; initial power scales with the current round. |
| Maestro | 30 | Fires 6 shots of escalating power, then reloads (taking extra damage). Final shot deals double power. Speeds up as HP drops. |
| Iron Lotus | 40 | Applies burn damage at the end of every player turn. Burn grows each attack and ramps up faster below half HP. Healing increases burn by 3. |
| Bomb | 1 | Counts down 20 turns, then detonates with an Attack(5) and dies. Attacking it triggers an immediate 20-damage explosion. |

---

## Project Layout

```
server/
├── src/
│   ├── main.rs            # Entry point, startup sequence
│   ├── config.rs          # Environment-based configuration
│   ├── state.rs           # AppState, GameSession
│   ├── error.rs           # AppError enum
│   ├── models/
│   │   ├── mod.rs         # Action enum
│   │   ├── user.rs        # User, CreateUserRequest, LogInRequest/Response
│   │   ├── character.rs   # Character, Stats, CreateCharacterRequest
│   │   └── game.rs        # Game, PlayerState, EnemyState, Health, events
│   ├── db/
│   │   ├── mod.rs         # Repository traits
│   │   ├── firestore.rs   # Firestore implementation (feature-gated)
│   │   └── mongodb.rs     # MongoDB implementation (feature-gated)
│   ├── routes/
│   │   ├── mod.rs         # Router construction
│   │   ├── auth.rs        # /create_user, /login
│   │   ├── character.rs   # /character/*
│   │   ├── game.rs        # /game/*
│   │   └── session.rs     # /session/*
│   ├── middleware/
│   │   ├── mod.rs
│   │   └── auth.rs        # Authorization header validation
│   ├── game/
│   │   ├── runner.rs      # Turn-based game loop
│   │   └── watcher.rs     # Inactivity timeout / stop signal
│   └── enemys/
│       ├── mod.rs         # Enemy trait, EnemyType, factory functions
│       ├── dummy.rs       # Passive placeholder enemy
│       ├── rose_buddies.rs
│       ├── copper_sides.rs
│       ├── maestro.rs
│       ├── iron_lotus.rs
│       └── bomb.rs
frontend/
├── pages/
│   ├── index/             # Landing page (redirects if already logged in)
│   ├── login/             # Sign-in form
│   ├── create_account/    # Registration form
│   ├── dashboard/         # Character list for the logged-in user
│   ├── character/         # Create or view/edit a character; start a new game
│   ├── game_session/      # Live turn-based combat (SSE-driven)
│   ├── game_stats/        # Post-game summary
│   ├── leaderboard/       # Global leaderboard with multi-criteria sorting
│   └── _error/            # 404 / error fallback
├── models/
│   ├── game.tsx           # Game, PlayerState, EnemyState, Character, Action types
│   └── api.tsx            # Request/response DTO types for the backend API
├── assets/                # Enemy and character sprite images
└── axiosConfig.tsx        # Pre-configured Axios client with auth interceptor
Cargo.toml
```
