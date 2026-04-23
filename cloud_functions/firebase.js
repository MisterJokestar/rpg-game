/**
 * initializes firestore database object for functions to use
 * @author: Sam Plemmons
 */

const { initializeApp } = require('firebase-admin/app');
const { getFirestore } = require('firebase-admin/firestore');

// this should onlt be called one time
initializeApp();
const db = getFirestore();

module.exports = db;