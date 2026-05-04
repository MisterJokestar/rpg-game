/** A complete game record returned by the backend. */
export type Game = {
    /** Unique game identifier (UUID v7). */
    id: string;
    /** Whether the game has ended (player died or won). */
    complete: boolean;
    /** `true` = player won, `false` = player lost, `null` = in progress. */
    win: boolean | null;
    /** Current round number (incremented each time an enemy is defeated). */
    round: number;
    /** Monotonically-increasing turn counter used by the turn-order system. */
    turn: number;
    /** Snapshot of the player's in-game combat state. */
    player_state: PlayerState;
    /** Snapshot of the current enemy's in-game combat state. */
    enemy_state: EnemyState;
    /**
     * Running tally of how many of each enemy type the player has defeated.
     * Keyed by the enemy type name string (e.g. `"RoseBuddies"`).
     */
    enemies_defeated: Map<string, number>;
};

/** Live combat state for the player during a game session. */
export type PlayerState = {
    /** ID of the owning user. */
    player_id: string;
    /** ID of the character being used in this game. */
    character_id: string;
    /**
     * The turn value on which the player will next act, or `null` if not yet
     * calculated for this round.
     */
    next_turn: number | null;
    /** Current and maximum HP. */
    health: Health;
    /** Damage the player will absorb before taking HP loss this turn. */
    block: number;
    /** Cumulative HP lost over the entire game. */
    damage_taken: number;
    /** Cumulative HP restored over the entire game. */
    damage_healed: number;
    /** Cumulative damage prevented by blocking over the entire game. */
    damage_blocked: number;
    /** Cumulative damage avoided through dodge mechanics over the entire game. */
    damage_dodged: number;
    /** Cumulative damage dealt to enemies over the entire game. */
    damage_dealt: number;
};

/** Live combat state for the current enemy during a game session. */
export type EnemyState = {
    /**
     * The turn value on which the enemy will next act, or `null` if not yet
     * calculated.
     */
    next_turn: number | null;
    /**
     * Enemy-specific internal state counter used to implement multi-turn
     * behaviour patterns (e.g. attack/defend cycles, ammo count).
     */
    state: number;
    /** Current and maximum HP. */
    health: Health;
    /** Damage the enemy will absorb before taking HP loss this turn. */
    block: number;
    /**
     * Which enemy type this state belongs to (e.g. `"RoseBuddies"`,
     * `"CopperSides"`, `"Maestro"`, `"IronLotus"`, `"Bomb"`).
     */
    enemy_type: string;
};

/** Current and maximum hit points for a combatant. */
export type Health = {
    /** Current HP. Clamped to `[0, max]` by the combat engine. */
    current: number;
    /** Maximum HP. */
    max: number
};

/**
 * A single combat action sent to `POST /session/:game_id/action`.
 *
 * - `{ Attack: power }` — deal damage scaled from the power stat.
 * - `{ Defend: defense }` — set a block value scaled from the defense stat.
 * - `{ Heal: defense }` — restore HP scaled from the defense stat.
 * - `"None"` — pass the turn without acting.
 */
export type Action =
    { Attack: number }
    | { Defend: number }
    | { Heal: number }
    | "None";

/** Combat statistics for a character. All values scale game-engine outcomes. */
export type Stats = {
    /** Scales attack damage output. */
    power: number;
    /** Determines how frequently the character acts (lower interval = faster). */
    speed: number;
    /** Scales block and heal amounts. */
    defense: number;
}

/** A player character returned by the backend. */
export type Character = {
    /** Unique character identifier (UUID v7). */
    id: string;
    /** ID of the user who owns this character. */
    owner: string;
    /** Display name chosen by the player. */
    name: string;
    /** Combat statistics that influence game-engine calculations. */
    stats: Stats;
    /** IDs of all games this character has been used in. */
    games: string[];
}

/** A registered user account returned by the backend. */
export type User = {
    /** Unique user identifier (UUID v7). */
    id: string;
    /** Human-readable username. */
    username: string;
    /** bcrypt hash of the user's password (not used client-side). */
    password_hash: string;
    /** Session secret — sent in the `Authorization` header. */
    secret: string;
    /** IDs of all characters owned by this user. */
    characters: String[];
}

/**
 * An aggregated leaderboard row (used by `frontend/models/game.tsx`).
 * @deprecated Use {@link LeaderBoardEntry} from `models/api.tsx` for the
 * live leaderboard page; this type is kept for historical reference.
 */
export type LeaderBoardEntry = {
    /** Display name of the player. */
    player_name: string;
    /** Total games won. */
    games_won: number;
    /** Total rounds survived across all games. */
    rounds_survived: number;
    /** Total damage dealt across all games. */
    damage_dealt: number;
    /** Total enemies defeated across all games. */
    enemies_defeated: number;
}

/**
 * Raw per-game leaderboard row before aggregation.
 * @deprecated Not currently used; kept for reference.
 */
export type LeaderBoardRaw = {
    /** Display name of the player. */
    player_name: string;
    /** `true` if the game was won, `false` if lost, `null` if in progress. */
    win: boolean | null;
    /** Number of rounds survived in this game. */
    round: number;
    /** Damage dealt in this game. */
    damage_dealt: number;
    /** Number of enemies defeated in this game. */
    enemies_defeated: number;
}
