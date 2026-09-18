'use strict';

const assert = require('node:assert/strict');
const { test } = require('node:test');
const { calculate, calculateJson } = require('../index.cjs');

const request = { year: 2024, month: 2, day: 10, hour: 12 };

test('object and canonical JSON APIs agree', () => {
  const chart = calculate(request);
  assert.deepEqual(chart, JSON.parse(calculateJson(JSON.stringify(request))));
  assert.equal(chart.palaces.length, 9);
});

test('ES modules and CommonJS expose identical behavior', async () => {
  const esm = await import('../index.mjs');
  assert.deepEqual(esm.calculate(request), calculate(request));
});

test('invalid dates, unknown fields and fractions are rejected by Rust', () => {
  for (const invalid of [
    { ...request, day: 30 }, { ...request, typo: 1 },
    { ...request, hour: 1.5 }, { ...request, hour: true },
    { ...request, hour: Number.NaN }, { ...request, hour: Infinity },
  ]) {
    assert.throws(() => calculate(invalid), { code: 'InvalidArg' });
  }
});

test('malformed JSON and non-object requests fail clearly', () => {
  assert.throws(() => calculateJson('{'), { code: 'InvalidArg' });
  for (const invalid of [null, undefined, 1, '2024-02-10', []]) {
    assert.throws(() => calculate(invalid), TypeError);
  }
});

test('calls are deterministic and do not mutate requests', () => {
  const frozen = Object.freeze({ ...request });
  assert.deepEqual(calculate(frozen), calculate(frozen));
  assert.deepEqual(frozen, request);
});
