# Server

Rust REST API backend built with [Axum](https://github.com/tokio-rs/axum). It is designed to run locally against MongoDB during development and against Google Cloud Firestore in production. The database backend is selected at runtime via an environment variable — no code changes needed to switch.

## Tech Stack

| Crate | Purpose |
|---|---|
| `axum 0.7` | HTTP routing and request/response handling |
| `tokio 1` | Async runtime |
| `tower` / `tower-http` | Middleware stack (CORS, request tracing) |
| `serde` / `serde_json` | Serialization — models derive `Serialize`/`Deserialize` |
| `thiserror` / `anyhow` | Structured error types and propagation |
| `tracing` / `tracing-subscriber` | Structured logging, level set via `RUST_LOG` |
| `dotenvy` | Loads `.env` at startup (no-op in production) |
| `uuid` | UUID v4 IDs for users, characters, and games |
| `firestore 0.44` | Production Firestore client (GCP Application Default Credentials) |
| `mongodb 3` | Local development MongoDB client |

## Project Layout

```
server/
├── src/
│   ├── main.rs              # Entry point — loads config, wires state, starts listener
│   ├── config.rs            # Reads all env vars into a Config struct
│   ├── state.rs             # AppState — shared resources injected into route handlers
│   ├── error.rs             # AppError enum, mapped to HTTP status codes
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs          # User, Character, Stats, AddCharacterRequest
│   │   └── game.rs          # Game, PlayerState, EnemyState
│   ├── db/
│   │   ├── mod.rs           # Repository trait (stubbed — see TODOs)
│   │   ├── firestore.rs     # FirestoreRepository (production backend)
│   │   └── mongodb.rs       # MongoRepository (local dev backend)
│   ├── routes/
│   │   ├── mod.rs           # create_router() — assembles all routes
│   │   └── items.rs         # Placeholder route handlers (commented out)
│   └── middleware/
│       ├── mod.rs
│       └── auth.rs          # Auth middleware (currently pass-through)
├── .env.example             # Template for local environment variables
├── docker-compose.yml       # Spins up a local MongoDB container
└── Cargo.toml
```

## Local Development Setup

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (stable)
- [Docker](https://docs.docker.com/get-docker/) (for local MongoDB)

### 1. Configure environment

```bash
cp .env.example .env
# Edit .env as needed — defaults work out of the box for local dev
```

Key variables (full list in `.env.example`):

| Variable | Default | Description |
|---|---|---|
| `DATABASE_BACKEND` | `mongodb` | `mongodb` for local dev, `firestore` for production |
| `MONGODB_URI` | `mongodb://localhost:27017` | MongoDB connection string |
| `MONGODB_DB_NAME` | `app_db` | Database name |
| `MONGO_ROOT_USERNAME` | `admin` | Docker container root user |
| `MONGO_ROOT_PASSWORD` | `yourpassword` | Docker container root password |
| `FIRESTORE_PROJECT_ID` | — | GCP project ID (only needed for Firestore backend) |
| `HOST` | `0.0.0.0` | Bind address |
| `PORT` | `5000` | Listen port |
| `RUST_LOG` | `server=debug,tower_http=debug` | Log verbosity |

### 2. Start MongoDB

```bash
docker compose up -d
```

This starts a MongoDB container at `localhost:27017` with a persistent volume (`mongo-data`).

### 3. Run the server

```bash
cargo run
```

The server listens on `http://0.0.0.0:5000` by default.

### Switching to Firestore (production)

1. Set up [Application Default Credentials](https://cloud.google.com/docs/authentication/provide-credentials-adc) (`gcloud auth application-default login` or a service account key in `GOOGLE_APPLICATION_CREDENTIALS`).
2. In `.env`, set:
   ```
   DATABASE_BACKEND=firestore
   FIRESTORE_PROJECT_ID=your-gcp-project-id
   ```
3. Deploy the Firestore rules and indexes from the `firestore/` folder (see `firestore/README.md`).

## Current State and TODOs

The scaffolding is in place but the application logic is not yet implemented. The major next steps are:

### 1. Define the Repository trait (`src/db/mod.rs`)

Replace the commented-out `Item`-based trait with one that covers `User` and `Game` operations:

```rust
#[async_trait]
pub trait Repository: Send + Sync {
    async fn get_user(&self, id: Uuid) -> Result<Option<User>, AppError>;
    async fn create_user(&self, user: User) -> Result<User, AppError>;
    // ... etc.
}
```

### 2. Implement the backends (`src/db/firestore.rs`, `src/db/mongodb.rs`)

Each file has a `TODO` comment marking where the `Repository` impl goes. Both structs are already constructed and connected at startup.

### 3. Wire the repository into `AppState` (`src/state.rs`)

```rust
pub struct AppState {
    pub db: Arc<dyn Repository>,
}
```

Uncomment and update the backend selection block in `src/main.rs`.

### 4. Add route handlers (`src/routes/items.rs` → rename to `users.rs` / `games.rs`)

The commented-out handlers in `items.rs` show the axum pattern. Replace them with handlers for `User` and `Game` endpoints and register the routes in `src/routes/mod.rs`.

### 5. Implement auth middleware (`src/middleware/auth.rs`)

The middleware currently passes every request through. The `TODO` comment in that file describes the intended approach: extract credentials, validate against the stored `secret` field on `User`, and return `401` on failure.

### 6. Add request/response DTOs

The models in `src/models/` are currently used as both DB documents and HTTP bodies. Consider splitting them into separate request/response types as the API surface grows.
