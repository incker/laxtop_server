'use strict';

const consoleLog = console.log;
console.log = () => undefined;

const processExitError = (code, message) => {
    processExitResponse({
        // 'auth/id-token-expired'
        // 'auth/argument-error'
        code: code || '',
        message: message || '',
    });
};

const processExitResponse = (obj) => {
    consoleLog(JSON.stringify(obj));
    process.exit()
};

const idToken = (() => {
    let toReturn = false;
    for (const value of process.argv) {
        if (toReturn) {
            return value;
        }
        if (value === '--token') {
            toReturn = true;
        }
    }
    processExitError('no_token', 'no token provided to command line');
    return '';
})();


const admin = require('firebase-admin');

admin.initializeApp({
    credential: admin.credential.cert(require('../../firebase_private_key.json')),
    databaseURL: 'https://laxtop-ce594.firebaseio.com'
});


admin.auth().verifyIdToken(idToken)
    .then((decodedToken) => {
        processExitResponse({
            uid: decodedToken.uid,
            phone: decodedToken.phone_number || '',
        });
    }).catch((error) => {
    processExitError(error.code || '', error.message || '');
})
    .then(() => {
        process.exit()
    });


/*
# run:
node src/main.js
node carbon - для разработки (docker)
node alpine - для продакшена (docker)
*/


/* INFO
Setup admin SDK
https://firebase.google.com/docs/admin/setup/

Verify id tokens
https://firebase.google.com/docs/auth/admin/verify-id-tokens
 */
