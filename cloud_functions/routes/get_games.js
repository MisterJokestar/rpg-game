/**
 * cloud function for getting all games associated with a character
 * @author: Sam Plemmons
 */

/* example json package
  { "characterId": "..." }
*/

const express = require('express');
const db = require('../firebase');

// connects entry point to this route
const router = express.Router();

const HTTP_STATUS = {
    OK: 200,
    BAD_REQUEST: 400,
    SERVER_ERROR: 500
};

router.post('/', async (req, res) => {
    try {
        const { characterId } = req.body;

        // field check
        if (!characterId ) {
            return res.status(HTTP_STATUS.BAD_REQUEST).send('Missing characterId');
        }

        const snapshot = await db.collection('Game').where('player_state.character_id', '==', characterId).get();

        // get all game IDs associated with the inputted character ID
        const gameIds = [];
        snapshot.forEach(doc => {
            gameIds.push(doc.id);
        });

        return res.status(HTTP_STATUS.OK).json({
            gameIds,
            success: true
        });

    } catch (err) {
        return res.status(HTTP_STATUS.SERVER_ERROR).send('Server error');
    }
});

module.exports = router;