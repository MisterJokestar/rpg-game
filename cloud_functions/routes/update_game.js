/**
 * cloud function for updating a game
 * @author: Sam Plemmons
 */

/* you don't need to input all of this, just the values that need to be updated
    {
        "gameId": "game_id",

        "complete": true,
        "win": true,
        "round": 99,
        "turn": 99,

        "player_state": {
            "next_turn": 100,
            "health": {
                "current": 99,
                "max": 100
            },
            "block": 999,
            "damage_taken": 1,
            "damage_healed": 999,
            "damage_blocked": 999,
            "damage_dodged": 999,
            "damage_dealt": 999
        },

        "enemy_state": {
            "next_turn": 100,
            "state": 1,
            "health": {
                "current": 1,
                "max": 2
            },
            "block": 3,
            "enemy_type": "chump"
        },

        "enemies_defeated": {
            "Dummy": 5,
            "The Red Fraud": 1
        }
    }
*/

const express = require('express');
const db = require('../firebase');

// connects entry point to this route
const router = express.Router();

const HTTP_STATUS = {
    OK: 200,
    BAD_REQUEST: 400,
    UNAUTHORIZED: 401,
    NOT_FOUND: 404,
    SERVER_ERROR: 500
};

router.post('/', async (req, res) => {
    try {
        const {
            gameId,
            complete,
            win,
            round,
            turn,
            player_state, // map of all player state stats
            enemy_state, // enemy state stats
            enemies_defeated // map of all enemies defeated
        } = req.body;

        if (!gameId) {
            return res.status(HTTP_STATUS.BAD_REQUEST).send('Missing required fields');
        }

        const gameRef = db.collection('Game').doc(gameId);
        const gameDoc = await gameRef.get();

        // check if game exists
        if (!gameDoc.exists) {
            return res.status(HTTP_STATUS.NOT_FOUND).send('Game not found');
        }

        // updates to general game stats
        const updates = {};
        if (complete !== undefined) updates.complete = complete;
        if (win !== undefined) updates.win = win;
        if (round !== undefined) updates.round = round;
        if (turn !== undefined) updates.turn = turn;

        // player state updates
        if (player_state !== undefined) {
            if (player_state.next_turn !== undefined)
            updates['player_state.next_turn'] = player_state.next_turn;

            if (player_state.block !== undefined)
            updates['player_state.block'] = player_state.block;

            if (player_state.damage_taken !== undefined)
            updates['player_state.damage_taken'] = player_state.damage_taken;

            if (player_state.damage_healed !== undefined)
            updates['player_state.damage_healed'] = player_state.damage_healed;

            if (player_state.damage_blocked !== undefined)
            updates['player_state.damage_blocked'] = player_state.damage_blocked;

            if (player_state.damage_dodged !== undefined)
            updates['player_state.damage_dodged'] = player_state.damage_dodged;

            if (player_state.damage_dealt !== undefined)
            updates['player_state.damage_dealt'] = player_state.damage_dealt;

            if (player_state.health !== undefined) {
                if (player_state.health.current !== undefined)
                updates['player_state.health.current'] = player_state.health.current;

                if (player_state.health.max !== undefined)
                updates['player_state.health.max'] = player_state.health.max;
            }
        }

        // enemy state updates
        if (enemy_state !== undefined) {
            if (enemy_state.next_turn !== undefined)
            updates['enemy_state.next_turn'] = enemy_state.next_turn;

            if (enemy_state.state !== undefined)
            updates['enemy_state.state'] = enemy_state.state;

            if (enemy_state.block !== undefined)
            updates['enemy_state.block'] = enemy_state.block;

            if (enemy_state.enemy_type !== undefined)
            updates['enemy_state.enemy_type'] = enemy_state.enemy_type;

            if (enemy_state.health !== undefined) {
            if (enemy_state.health.current !== undefined)
                updates['enemy_state.health.current'] = enemy_state.health.current;

            if (enemy_state.health.max !== undefined)
                updates['enemy_state.health.max'] = enemy_state.health.max;
            }
        }

        // update enemies defeated
        if (enemies_defeated !== undefined) {
            Object.keys(enemies_defeated).forEach(enemy => {
            updates[`enemies_defeated.${enemy}`] = enemies_defeated[enemy];
            });
        }

        await gameRef.update(updates);

        return res.status(HTTP_STATUS.OK).json({
            gameId: gameRef.id,
            success: true
        });

    } catch (err) {
        return res.status(HTTP_STATUS.SERVER_ERROR).send('Server error');
    }
});

module.exports = router;