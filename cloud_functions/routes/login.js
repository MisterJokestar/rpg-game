/**
 * cloud function for user login
 * @author: Sam Plemmons
 */

/* example json package
    { "username": "testuser", "password": "testpassword" }
*/

const express = require('express');
const bcrypt = require('bcrypt');
const crypto = require('crypto');
const db = require('../firebase');

// connects entry point to this route
const router = express.Router();

const RAND_BYTES = 12;
const FIRST_USER = 0;
const HTTP_STATUS = {
    OK: 200,
    BAD_REQUEST: 400,
    UNAUTHORIZED: 401,
    NOT_FOUND: 404,
    SERVER_ERROR: 500
};

router.post('/', async (req, res) => {
    try {
        // get username and password from json package
        const { username, password } = req.body;

        // check fields
        if (!username || !password) {
            return res.status(HTTP_STATUS.BAD_REQUEST).send('Missing username or password');
        }
        
        // snapshot of user in firestore database
        const snapshot = await db.collection('User').where("username", "==", username).get();
        
        if (snapshot.empty) {
            return res.status(HTTP_STATUS.NOT_FOUND).send('User not found');
        }

        // .docs returns an array even if there is only 1 data entry
        const userDoc = snapshot.docs[FIRST_USER];
        const userData = userDoc.data();

        // check matching passwords
        const passwordMatch = await bcrypt.compare(password, userData.password);
        if (!passwordMatch) {
            return res.status(HTTP_STATUS.UNAUTHORIZED).send('Invalid password');
        }

        // generate new secret value
        const newSecret = crypto.randomBytes(RAND_BYTES).toString('hex');
        await db.collection('User').doc(userDoc.id).update({
            secret: newSecret
        });

        return res.status(HTTP_STATUS.OK).json({
            userId: userDoc.id,
            secret: newSecret,
            success: true
        });

    } catch (err) {
        return res.status(HTTP_STATUS.SERVER_ERROR).send('Server error');
    }
});

module.exports = router;