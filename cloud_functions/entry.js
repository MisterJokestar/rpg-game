/**
 * entry point of cloud project that routes to all cloud functions
 * @author: Sam Plemmons
 */

// URL: https://cloud-functions-91972588391.us-central1.run.app

const functions = require('firebase-functions');
const express = require('express');
const cors = require('cors');

// file routes for cloud functions
const loginRoute = require('./routes/login');
const createUserRoute = require('./routes/create_user');
const createCharacterRoute = require('./routes/create_character');
const updateCharacterRoute = require('./routes/update_character');
const createGameRoute = require('./routes/create_game');
const updateGameRoute = require('./routes/update_game');
const getCharactersRoute = require('./routes/get_characters');
const getGamesRoute = require('./routes/get_games');
const getStatsRoute = require('./routes/get_stats');
const getLeaderboardRoute = require('./routes/get_leaderboard');

const app = express();

// cors is for these functions to work with Axios
app.use(cors({ origin: true }));
app.use(express.json());

// connecting routes to cloud project
app.use('/login', loginRoute);
app.use('/createUser', createUserRoute);
app.use('/createCharacter', createCharacterRoute);
app.use('/updateCharacter', updateCharacterRoute);
app.use('/createGame', createGameRoute);
app.use('/updateGame', updateGameRoute);
app.use('/getCharacters', getCharactersRoute);
app.use('/getGames', getGamesRoute);
app.use('/getStats', getStatsRoute);
app.use('/getLeaderboard', getLeaderboardRoute);

exports.entryPoint = functions.https.onRequest(app);
