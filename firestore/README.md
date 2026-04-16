# Firestore

Firebase/Firestore configuration for the production database. This folder is managed with the [Firebase CLI](https://firebase.google.com/docs/cli) and contains all security rules and composite index definitions.

## Tech Stack

| Tool | Purpose |
|---|---|
| [Firebase CLI](https://firebase.google.com/docs/cli) | Deploys rules and indexes to the Firebase project |
| `firestore.rules` | Security rules — controls read/write access per collection |
| `firestore.indexes.json` | Composite index definitions required for multi-field queries |
| `firebase.json` | Project-level config pointing the CLI at the correct rule/index files |

Firestore does not require a schema definition. Collections (`users`, `games`) are created automatically on the first document write from the server.

## File Reference

### `firebase.json`

Tells the Firebase CLI where to find the rules and index files when running `firebase deploy`. No edits should be needed here unless files are renamed.

### `firestore.rules`

Controls who can read and write documents. **Currently locked down to deny all access** — this is intentional while the server-side auth layer is being built. All database access goes through the Rust backend using Application Default Credentials (ADC), which bypasses these client-facing rules. Once client-side access is needed (e.g. a frontend talking directly to Firestore), rules must be updated here.

### `firestore.indexes.json`

Firestore requires composite indexes for any query that filters or sorts on more than one field. The indexes defined here cover the query patterns the server intends to support:

| Collection | Fields Indexed | Use Case |
|---|---|---|
| `games` | `player_state.player_id` + `player_state.character_id` + `complete` | Look up incomplete games for a specific character |
| `games` | `player_state.player_id` + `complete` | Look up all games (complete or not) for a player |
| `games` | `complete` + `win` | Filter for won/lost completed games |

`fieldOverrides` suppresses automatic single-field indexes on `users.password_hash` and `users.secret` — these fields should never be queried directly and excluding them from indexes is a small security and cost improvement.

## Local Development

During local development the server uses MongoDB (see `server/README.md`). You do **not** need to interact with this folder for local dev. Come back here when deploying to GCP.

## Deploying to GCP

### Prerequisites

1. [Firebase CLI](https://firebase.google.com/docs/cli) installed:
   ```bash
   npm install -g firebase-tools
   ```
2. Authenticated:
   ```bash
   firebase login
   ```
3. Firebase project linked — run once from the repo root or this folder:
   ```bash
   firebase use --add
   # Select your GCP project and give it an alias (e.g. "production")
   ```

### Deploy rules and indexes

```bash
cd firestore/
firebase deploy --only firestore
```

This deploys both `firestore.rules` and `firestore.indexes.json` in one command. Index builds run asynchronously on Google's side and may take a few minutes to become active — check the Firebase console under **Firestore > Indexes**.

### Deploy only rules or only indexes

```bash
firebase deploy --only firestore:rules
firebase deploy --only firestore:indexes
```

## Current State and TODOs

### Security rules (`firestore.rules`)

The current blanket-deny rule is a safe placeholder. As the server auth middleware is implemented, update the rules to reflect which collections the backend service account needs access to, and add client rules if the frontend ever queries Firestore directly.

A starting point once auth is in place:

```
rules_version = '2';
service cloud.firestore {
  match /databases/{database}/documents {

    // Only authenticated users can read their own document.
    match /users/{userId} {
      allow read: if request.auth != null && request.auth.uid == userId;
      allow write: if false; // server-side only
    }

    // Game documents are readable by the player who owns them.
    match /games/{gameId} {
      allow read: if request.auth != null
                  && resource.data.player_state.player_id == request.auth.uid;
      allow write: if false; // server-side only
    }
  }
}
```

### Indexes (`firestore.indexes.json`)

Add a new index entry here whenever the server needs a query that Firestore rejects with a "requires an index" error. The Firebase console will provide a direct link to the required index definition when this happens — copy that definition into this file and redeploy rather than creating indexes manually in the console (manual indexes are not tracked in source control).

### Collections

Neither `users` nor `games` collections exist yet — they will be created on the first document write from the server. The data shapes are defined as Rust structs in `server/src/models/`.
