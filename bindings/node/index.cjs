'use strict';

const { calculateJson } = require('./native.cjs');

function calculate(request) {
  if (request === null || typeof request !== 'object' || Array.isArray(request)) {
    throw new TypeError('request must be an object containing year, month, day, hour');
  }
  return JSON.parse(calculateJson(JSON.stringify(request)));
}

module.exports = { calculate, calculateJson };
