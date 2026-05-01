export type Game = {
    id: string;
    complete: boolean;
    win: boolean | null;
    round: number;
    turn: number;
    player_state: PlayerState;
    enemy_state: EnemyState;
    enemies_defeated: Map<string, number>;
};

export type PlayerState = {
    player_id: string;
    character_id: string;
    next_turn: number | null;
    health: Health;
    block: number;
    damage_taken: number;
    damage_healed: number;
    damage_blocked: number;
    damage_dodged: number;
    damage_dealt: number;
};

export type EnemyState = {
    next_turn: number | null;
    state: number;
    health: Health;
    block: number;
    enemy_type: string;
};

export type Health = {
    current: number;
    max: number
};

export type Action =
    { Attack: number }
    | { Defend: number }
    | { Heal: number }
    | "None";

export type Stats = {
    power: number;
    speed: number;
    defense: number;
}

export type Character = {
    id: string;
    owner: string;
    name: string;
    stats: Stats;
    games: string[];
}

export type User = {
    id: string;
    username: string;
    password_hash: string;
    secret: string;
    characters: String[];
}

export type LeaderBoardEntry = {
    player_name: string;
    games_won: number;
    rounds_survived: number;
    damage_dealt: number;
    enemies_defeated: number;
}

export type LeaderBoardRaw = {
    player_name: string;
    win: boolean | null;
    round: number;
    damage_dealt: number;
    enemies_defeated: number;
}
