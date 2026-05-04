# Server

Rust REST API backend built with [Axum](https://github.com/tokio-rs/axum). Runs locally against MongoDB during development and against Google Cloud Firestore in production. The database backend is selected at **compile time** via Cargo feature flags.

## Tech Stack

| Crate | Purpose |
|---|---|
| `axum 0.7` | HTTP routing and request/response handling |
| `tokio 1` | Async runtime |
| `tower` / `tower-http` | Middleware stack (CORS, request tracing) |
| `serde` / `serde_json` | Serialisation — models derive `Serialize`/`Deserialize` |
| `thiserror` / `anyhow` | Structured error types and propagation |
| `tracing` / `tracing-subscriber` | Structured logging, level set via `RUST_LOG` |
| `dotenvy` | Loads `.env` at startup (no-op in production) |
| `uuid` | UUID **v7** IDs for users, characters, and games |
| `bcrypt` | Password hashing |
| `rand` | Random number generation (session secrets, enemy selection) |
| `firestore 0.44` | Production Firestore client (GCP Application Default Credentials) |
| `mongodb 3` | Local development MongoDB client |

## Project Layout

```
server/
├── src/
│   ├── main.rs              # Entry point — loads config, wires state, starts listener
│   ├── config.rs            # Reads all env vars into a Config struct
│   ├── state.rs             # AppState and GameSession (shared handles per live game)
│   ├── error.rs             # AppError enum, mapped to HTTP status codes
│   ├── models/
│   │   ├── mod.rs           # Action enum (Attack / Defend / Heal / None)
│   │   ├── user.rs          # User, CreateUserRequest, LogInRequest/Response
│   │   ├── character.rs     # Character, Stats, CreateCharacterRequest/Response
│   │   └── game.rs          # Game, PlayerState, EnemyState, Health, events, Combatant trait
│   ├── db/
│   │   ├── mod.rs           # Repository traits (UserRepository, GameRepository, CharacterRepository)
│   │   ├── firestore.rs     # FirestoreRepository — production backend (feature-gated)
│   │   └── mongodb.rs       # MongoRepository — local dev backend (feature-gated)
│   ├── routes/
│   │   ├── mod.rs           # create_router() — assembles all routes with CORS and auth middleware
│   │   ├── auth.rs          # POST /create_user, POST /login
│   │   ├── character.rs     # GET /character/all/:user_id, GET /character/:id, POST /character/*
│   │   ├── game.rs          # GET /game/:id, GET /games, POST /game/*, GET /leaderboard
│   │   └── session.rs       # POST/GET /session/:id/*
│   ├── middleware/
│   │   ├── mod.rs
│   │   └── auth.rs          # Authorization header validation middleware
│   ├── game/
│   │   ├── runner.rs        # Turn-based game loop (spawned per session)
│   │   └── watcher.rs       # Inactivity timeout / stop signal handler
│   └── enemys/
│       ├── mod.rs           # Enemy trait, EnemyType enum, factory functions
│       ├── dummy.rs         # Passive placeholder enemy
│       ├── rose_buddies.rs  # Thorns + heal cycle enemy
│       ├── copper_sides.rs  # Charge-up tanky enemy
│       ├── maestro.rs       # Ammo/reload ranged attacker
│       ├── iron_lotus.rs    # Burn-stacking enemy
│       └── bomb.rs          # Countdown detonator enemy
├── .env.example             # Template for local environment variables
├── Dockerfile               # Multi-stage build for the server image
└── Cargo.toml
```

## Local Development Setup

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (stable)
- A running MongoDB instance — use the database-only compose file:
  ```bash
  docker-compose up mongo --build -d
  ```

### 1. Configure environment

```bash
cp .env.example .env
# Edit .env as needed — defaults work out of the box for local dev
```

Key variables (full list in `.env.example`):

| Variable | Default | Description |
|---|---|---|
| `MONGODB_URI` | `mongodb://localhost:27017` | MongoDB connection string |
| `MONGODB_DB_NAME` | `app_db` | Database name |
| `MONGO_ROOT_USERNAME` | `admin` | Docker container root user |
| `MONGO_ROOT_PASSWORD` | `yourpassword` | Docker container root password |
| `FIRESTORE_PROJECT_ID` | — | GCP project ID (only needed for Firestore backend) |
| `HOST` | `0.0.0.0` | Bind address |
| `PORT` | `5000` | Listen port |
| `RUST_LOG` | `server=debug,tower_http=debug` | Log verbosity |

### 2. Select a database backend

The backend is chosen at **compile time** via a Cargo feature flag — there is no runtime environment variable to switch backends:

```bash
# Local development — MongoDB
cargo run --features mongodb

# Production — Firestore
cargo run --features firestore
```

There is no default feature; you must pass exactly one flag.

### 3. Option A - Run Server and MongoDB in Docker

```bash
docker-compose up --build -d
```

### 3. Option B — MongoDB in Docker, server via cargo

```bash
# From the repo root:
docker-compose up mongo --build -d   # start only MongoDB
cd server
cargo run --features mongodb
```

The server listens on `http://0.0.0.0:5000` by default.

## Switching to Firestore (production)

```bash
cargo run --features firestore
```

Ensure `FIRESTORE_PROJECT_ID` is set in `.env` and Application Default Credentials are configured:

```bash
gcloud auth application-default login
# or set GOOGLE_APPLICATION_CREDENTIALS=/path/to/service-account.json
```

## Generating Documentation

```bash
cargo doc --features mongodb --no-deps --open
```

All public items carry `///` doc comments; all modules carry `//!` module-level doc blocks.
