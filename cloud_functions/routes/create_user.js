/**
 * cloud function for creating new user
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

const SALT = 10;
const RAND_BYTES = 12;
const HTTP_STATUS = {
    CREATED: 201,
    BAD_REQUEST: 400,
    CONFLICT: 409,
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
        
        // check if username is already chosen
        const existing = await db.collection('User').where("username", "==", username).get();
        if (!existing.empty) {
            return res.status(HTTP_STATUS.CONFLICT).send('Username already exists');
        }

        // create secret and hash password with salt
        const secret = crypto.randomBytes(RAND_BYTES).toString('hex');
        const salt = await bcrypt.genSalt(SALT);
        const hashedPassword = await bcrypt.hash(password, salt);

        // add new user to firestore
        const docRef = await db.collection('User').add({
            username: username,
            password: hashedPassword,
            secret: secret,
        });
                
        return res.status(HTTP_STATUS.CREATED).json({
            userId: docRef.id,
            secret: secret,
            success: true
        });

    } catch (err) {
        return res.status(HTTP_STATUS.SERVER_ERROR).send('Server error');
    }
});

module.exports = router;