'use strict';

const sharp = require('sharp');
const fs = require('fs');

const consoleLog = console.log;
console.log = () => undefined;

/**
 *
 * @param {String} resp
 */
const processExitResponse = (resp) => {
    consoleLog(resp);
    process.exit();
};

const storageFile = (() => {
    let toReturn = false;
    for (const value of process.argv) {
        if (toReturn) {
            return value;
        }
        if (value === '--file') {
            toReturn = true;
        }
    }

    processExitResponse('no file provided to command line');
    return '';
})();


const deleteFileIfExist = (storageFile) => {
    try {
        fs.unlinkSync(storageFile);
    } catch (e) {
    }
};

const newStorageFile = storageFile + '.webp';

sharp(storageFile)
    .toFile(newStorageFile, (error) => {
        deleteFileIfExist(storageFile);
        if (error) {
            processExitResponse(error.toString());
        }
        processExitResponse('done');
    });
