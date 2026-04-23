/**
 * cloud function for creating new game
 * @author: Sam Plemmons
 */

/* example json package
  { "userId": "...", "secret": "...", "characterId": "..." }
*/

const express = require('express');
const db = require('../firebase');

// connects entry point to this route
const router = express.Router();

const DEFAULT_VALUE = 0;
const HTTP_STATUS = {
    CREATED: 201,
    BAD_REQUEST: 400,
    UNAUTHORIZED: 401,
    NOT_FOUND: 404,
    SERVER_ERROR: 500
};

router.post('/', async (req, res) => {
    try {
        // required fields
        const { userId, secret, characterId } = req.body;
        if (!userId || !secret || !characterId) {
        return res.status(HTTP_STATUS.BAD_REQUEST).send('Missing required fields');
        }

        const userRef = db.collection('User').doc(userId);
        const userDoc = await userRef.get();

        // check user
        if (!userDoc.exists) {
        return res.status(HTTP_STATUS.NOT_FOUND).send('User not found');
        }

        const userData = userDoc.data();

        // validate user
        if (userData.secret !== secret) {
        return res.status(HTTP_STATUS.UNAUTHORIZED).send('Invalid credentials');
        }

        const characterRef = userRef.collection('Characters').doc(characterId);
        const characterDoc = await characterRef.get();

        // check character
        if (!characterDoc.exists) {
        return res.status(HTTP_STATUS.NOT_FOUND).send('Character not found');
        }

        const gameRef = db.collection('Game').doc();

        const newGame = {
        complete: false,
        win: null,
        round: DEFAULT_VALUE,
        turn: DEFAULT_VALUE,

        enemies_defeated: {},

        player_state: {
            player_id: userId,
            character_id: characterId,
            next_turn: null,
            health: {
                current: DEFAULT_VALUE,
                max: DEFAULT_VALUE
            },
            block: DEFAULT_VALUE,
            damage_taken: DEFAULT_VALUE,
            damage_healed: DEFAULT_VALUE,
            damage_blocked: DEFAULT_VALUE,
            damage_dodged: DEFAULT_VALUE,
            damage_dealt: DEFAULT_VALUE
        },

        enemy_state: {
            next_turn: null,
            state: DEFAULT_VALUE,
            health: {
                current: DEFAULT_VALUE,
                max: DEFAULT_VALUE
            },
            block: DEFAULT_VALUE,
            enemy_type: "little guy"
        }
        };

        // update game database
        await gameRef.set(newGame);

        return res.status(HTTP_STATUS.CREATED).json({
        gameId: gameRef.id,
        success: true
        });


    } catch (err) {
        return res.status(HTTP_STATUS.SERVER_ERROR).send('Server error');
    }
});

module.exports = router;