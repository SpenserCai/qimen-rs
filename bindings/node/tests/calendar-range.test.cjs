'use strict';

const assert = require('node:assert/strict');
const { test } = require('node:test');
const { calculate } = require('../index.cjs');

test('Gregorian range endpoints and the 1582 dates survive native loading', () => {
  // Independent Gregorian ordinal differences from the 2000-01-07 Jia-Zi anchor.
  for (const [year, month, day, expected] of [
    [1, 1, 1, 15], [4, 2, 29, 29], [24, 1, 28, 42],
    [1070, 1, 1, 39], [1582, 10, 5, 0],
    [1582, 10, 14, 9], [2101, 1, 1, 44], [9999, 12, 31, 53],
  ]) {
    const chart = calculate({ year, month, day, hour: 12 });
    assert.equal(chart.calendar.four_pillars.day.index, expected);
    assert.equal(chart.palaces.length, 9);
  }
  for (const year of [0, 10000]) {
    assert.throws(() => calculate({ year, month: 1, day: 1, hour: 0 }), { code: 'InvalidArg' });
  }
});
