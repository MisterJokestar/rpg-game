# Cloud Final Project

A full-stack web application with a Rust/Axum backend, React/Vike frontend, and MongoDB database. Supports local development via MongoDB and production deployment via Firestore.

## Stack

- **Frontend**: React + Vike (SSR) + Tailwind CSS
- **Backend**: Rust + Axum
- **Database**: MongoDB (local) / Firestore (production)

---

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) and Docker Compose

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
| `PORT` | `5000` | Backend server port |
| `DATABASE_BACKEND` | `mongodb` | `mongodb` for local dev, `firestore` for prod |
| `MONGO_ROOT_USERNAME` | `admin` | MongoDB root user |
| `MONGO_ROOT_PASSWORD` | — | MongoDB root password |
| `MONGO_PORT` | `27017` | MongoDB port |
| `MONGODB_DB_NAME` | `app_db` | Database name |
| `FIRESTORE_PROJECT_ID` | — | GCP project ID (production only) |

Key variables in `frontend/.env`:

| Variable | Default | Description |
|---|---|---|
| `PORT` | `3000` | Frontend server port |
| `VITE_SERVER_BASE_URL` | `http://localhost:5000` | URL the frontend uses to reach the backend |

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
cargo run
```

Requires `server/.env` with `DATABASE_BACKEND=mongodb` and a running MongoDB instance (use `docker-compose.db.yml`).

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
