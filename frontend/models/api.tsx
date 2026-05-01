
export type CreateUserRequest = {
    username: string;
    password: string;
}

export type LogInRequest = {
    username: string;
    password: string;
}

export type LogInResponse = {
    user_id: string;
    secret: string;
}

export type CreateGameRequest = {
    player_id: string;
    character_id: string;
}

export type CreateCharacterRequest = {
    player_id: string;
    character_id: string;
    power: number;
    speed: number;
    defense: number;
}

export type LeaderBoardEntry = {
    player_name: string;
    wins: number;
    rounds: number;
    damage_dealt: number;
    enemies_defeated: number;
}
