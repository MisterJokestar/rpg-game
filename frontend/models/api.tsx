/** Request body for `POST /create_user`. */
export type CreateUserRequest = {
    /** Desired username. Must be unique across all registered accounts. */
    username: string;
    /** Plain-text password. Hashed server-side before storage. */
    password: string;
}

/** Request body for `POST /login`. */
export type LogInRequest = {
    /** Username of the account to authenticate. */
    username: string;
    /** Plain-text password to verify. */
    password: string;
}

/**
 * Response body returned after a successful `POST /login` or
 * `POST /create_user`.
 *
 * Store both values in `localStorage` and pass them as
 * `Authorization: <user_id>:<secret>` on authenticated requests. The secret
 * is rotated on every login — always update the stored value.
 */
export type LogInResponse = {
    /** The authenticated user's unique ID (UUID v7). */
    user_id: string;
    /** Freshly-generated session secret. */
    secret: string;
}

/** Request body for `POST /game/new`. */
export type CreateGameRequest = {
    /** ID of the player starting the game. */
    player_id: string;
    /** ID of the character the player wants to use. */
    character_id: string;
}

/** Request body for `POST /character/new`. */
export type CreateCharacterRequest = {
    /** ID of the owning user. */
    player_id: string;
    /** Display name for the new character. */
    character_name: string;
    /** Initial power stat. */
    power: number;
    /** Initial speed stat. */
    speed: number;
    /** Initial defense stat. */
    defense: number;
}

/**
 * An aggregated leaderboard entry returned by `GET /leaderboard`.
 *
 * Each entry represents one player's cumulative stats across all their games.
 */
export type LeaderBoardEntry = {
    /** Display name of the player. */
    player_name: string;
    /** Total number of games won. */
    wins: number;
    /** Sum of rounds survived across all games. */
    rounds: number;
    /** Total damage dealt to enemies across all games. */
    damage_dealt: number;
    /** Total number of distinct enemies defeated across all games. */
    enemies_defeated: number;
}
