/**
 * cloud functions for getting character and game stats
 * @author: Sam Plemmons
 */

/* example json package
  { "gameId": "..." }
*/

const express = require('express');
const db = require('../firebase');

// connects entry point to this route
const router = express.Router();

const HTTP_STATUS = {
    OK: 200,
    BAD_REQUEST: 400,
    NOT_FOUND: 404,
    SERVER_ERROR: 500
};

router.post('/', async (req, res) => {
    try {
        const { gameId } = req.body;
        
        // gameId field
        if (!gameId) {
            return res.status(HTTP_STATUS.BAD_REQUEST).send('Missing gameId');
        }

        const gameRef = db.collection('Game').doc(gameId);
        const gameDoc = await gameRef.get();

        // check if game is real
        if (!gameDoc.exists) {
            return res.status(HTTP_STATUS.NOT_FOUND).send('Game not found');
        }

        const gameData = gameDoc.data();
        const playerId = gameData.player_state.player_id;
        const characterId = gameData.player_state.character_id;
        const characterRef = db.collection('User').doc(playerId).collection('Characters').doc(characterId);
        const characterDoc = await characterRef.get();

        // check if character exists in game (they should...)
        if (!characterDoc.exists) {
            return res.status(HTTP_STATUS.NOT_FOUND).send('Character not found');
        }

        const characterData = characterDoc.data();

        // this is to prevent returning the ids
        // if you don't wanna worry about it, just change player_state in the return .json
        const returnPlayerState = {
            health: gameData.player_state.health,
            damage_taken: gameData.player_state.damage_taken,
            damage_healed: gameData.player_state.damage_healed,
            damage_dodged: gameData.player_state.damage_dodged,
            damage_dealt: gameData.player_state.damage_dealt,
            damage_blocked: gameData.player_state.damage_blocked
        };

        const returnEnemyState = {
            type: gameData.enemy_state.enemy_type,
            health: gameData.enemy_state.health
        };
            
        return res.status(HTTP_STATUS.OK).json({
            name: characterData.name,
            stats: characterData.stats,
            win: gameData.win,
            round: gameData.round,
            turn: gameData.turn,
            player_state: returnPlayerState, // gameData.player_state
            enemy_state: returnEnemyState, // gameData.enemy_state
            status: true
        });

    } catch (err) {
        return res.status(HTTP_STATUS.SERVER_ERROR).send('Server error');
    }
});

module.exports = router;