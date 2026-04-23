/**
 * cloud functions for getting all characters from a user
 * @author: Sam Plemmons
 */

/* example json package
  { "userId": "...", "secret": "..." }
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
        // check fields
        const { userId, secret } = req.body;
        if (!userId || !secret) {
            return res.status(HTTP_STATUS.BAD_REQUEST).send('Missing userId');
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

        // reference to user's characters
        const characterRef = userRef.collection('Characters');
        const snapshot  = await characterRef.get();
        const characters = [];

        // get each character ID and name
        snapshot.forEach(doc => {
            const data = doc.data();
            characters.push({
                characterId: doc.id,
                name: data.name
            });
        });

        return res.status(HTTP_STATUS.OK).json({
            characters,
            success: true
        });

    } catch (err) {
        return res.status(HTTP_STATUS.SERVER_ERROR).send('Server error');
    }
});

module.exports = router;