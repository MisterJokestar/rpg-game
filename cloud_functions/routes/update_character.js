/**
 * cloud function for updating character
 * @author: Sam Plemmons
 */

/* example json package
    { "userId": "...", "secret": "...", "characterId": "...", "gameId": "...", "defense": 999, "power": 999, "speed": 999 }
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
        // check fields
        const { userId, secret, characterId, gameId, defense, power, speed } = req.body;
        if (!userId || !secret || !characterId) {
            return res.status(HTTP_STATUS.BAD_REQUEST).send('Missing required fields');
        }

        const userRef = db.collection('User').doc(userId);
        const userDoc = await userRef.get();

        // check if user is real
        if (!userDoc.exists) {
            return res.status(HTTP_STATUS.NOT_FOUND).send('User not found');
        }

        const userData = userDoc.data();

        // check secret value (user authentication)
        if (userData.secret !== secret) {
            return res.status(HTTP_STATUS.UNAUTHORIZED).send('Invalid credentials');
        }

        const characterRef = userRef.collection('Characters').doc(characterId);
        const characterDoc = await characterRef.get();

        // check for valid character
        if (!characterDoc.exists) {
            return res.status(HTTP_STATUS.NOT_FOUND).send('Character not found');
        }

        // set updates for character
        const updates = {};

        // updates stats
        if (defense !== undefined) {
            updates['stats.defense'] = defense;
        }
        if (power !== undefined) {
            updates['stats.power'] = power;
        }
        if (speed !== undefined) {
             updates['stats.speed'] = speed;
        }
        if (Object.keys(updates).length !== 0) {
            await characterRef.update(updates);
        }

        // update games
        if (gameId !== undefined) {
            await characterRef.update({
                games: FieldValue.arrayUnion(gameId)
            });
        }

        return res.status(HTTP_STATUS.OK).json({
            characterId: characterRef.id,
            success: true
        });

    } catch (err) {
        return res.status(HTTP_STATUS.SERVER_ERROR).send('Server error');
    }
});

module.exports = router;