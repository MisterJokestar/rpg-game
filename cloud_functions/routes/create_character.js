/**
 * cloud function for creating new character
 * @author: Sam Plemmons
 */

/* example json package
    { "userId": "...", "secret": "...", "name": "little guy", "defense": 10, "power": 10, "speed": 10 }
*/

const express = require('express');
const db = require('../firebase');

// connects entry point to this route
const router = express.Router();

const HTTP_STATUS = {
    CREATED: 201,
    BAD_REQUEST: 400,
    UNAUTHORIZED: 401,
    NOT_FOUND: 404,
    SERVER_ERROR: 500
};

router.post('/', async (req, res) => {
    try {
        // check fields
        const { userId, secret, name, defense, power, speed } = req.body;
        if (!userId || !secret || !name) {
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

        const newCharacter = {
            name: name,
            stats: {
                defense: defense,
                power: power,
                speed: speed
            },
            games: []
        };

        // set stats
        const characterRef = userRef.collection('Characters').doc();
        await characterRef.set(newCharacter);

        return res.status(HTTP_STATUS.CREATED).json({
            characterId: characterRef.id,
            success: true
        });

    } catch (err) {
        return res.status(HTTP_STATUS.SERVER_ERROR).send('Server error');
    }
});

module.exports = router;