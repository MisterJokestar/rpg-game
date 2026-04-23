/**
 * cloud function for getting leaderboard
 * @author: Sam Plemmons
 */

const express = require('express');
const db = require('../firebase');

// connects entry point to this route
const router = express.Router();

const BASE_ENEMIES_DEFEATED = 0;
const HTTP_STATUS = {
    OK: 200,
    SERVER_ERROR: 500
};

router.post('/', async (req, res) => {
    try {
        const snapshot = await db.collection('Game').get();
        const leaderboard = [];

        for (const doc of snapshot.docs) {
            const game = doc.data();
            const playerId = game.player_state.player_id;
            const userRef = db.collection('User').doc(playerId);
            const userDoc = await userRef.get();

            // skip if user somehow doesn't exist
            if (!userDoc.exists) continue;
            const userData = userDoc.data();

            // get total enemies defeated
            let enemiesDefeated = BASE_ENEMIES_DEFEATED;
            if (game.enemies_defeated !== undefined) {
                enemiesDefeated = Object.values(game.enemies_defeated)
                .reduce((sum, count) => sum + count, BASE_ENEMIES_DEFEATED);
            }

            // will add enemies defeated later once the game collection is updated
            leaderboard.push({
                player_name: userData.username,
                round: game.round,
                turn: game.turn,
                damage_dealt: game.player_state.damage_dealt,
                enemies_defeated: enemiesDefeated
            });
        }

        // auto sort based on damage dealth and enemies defeated
        leaderboard.sort((a, b) => {
            // primary sort
            if (b.damage_dealt !== a.damage_dealt) {
                return b.damage_dealt - a.damage_dealt;
            }
            // secondary sort
            return b.enemies_defeated - a.enemies_defeated;
        });

        return res.status(HTTP_STATUS.OK).json({
            leaderboard,
            success: true
        });

    } catch (err) {
        return res.status(HTTP_STATUS.SERVER_ERROR).send('Server error');
    }
});

module.exports = router;