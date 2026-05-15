use serde::Serialize;

/// An aggregated leaderboard row, computed from all of a player's game records.
#[derive(Debug, Clone, Serialize)]
pub struct LeaderboardEntry {
    /// Display name of the player.
    pub player_name: String,
    /// Total number of games won.
    pub wins: i64,
    /// Sum of rounds survived across all games.
    pub rounds: i64,
    /// Total damage dealt to enemies across all games.
    pub damage_dealt: i64,
    /// Total number of distinct enemies defeated across all games.
    pub enemies_defeated: i64,
}

impl LeaderboardEntry {
    /// Create a zeroed leaderboard entry for the given player name.
    pub fn new(player_name: String) -> Self {
        LeaderboardEntry {
            player_name,
            wins: 0,
            rounds: 0,
            damage_dealt: 0,
            enemies_defeated: 0
        }
    }
}
