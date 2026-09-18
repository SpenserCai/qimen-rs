'use strict';

const assert = require('node:assert/strict');
const path = require('node:path');
const wasm = require(path.resolve(process.argv[2] || path.join(__dirname, '../pkg-node/qimen_wasm.js')));

const request = { year: 2024, month: 2, day: 10, hour: 12 };
const chart = wasm.calculate(request);
assert.equal(chart.palaces.length, 9);
assert.deepEqual(chart, JSON.parse(wasm.calculateJson(JSON.stringify(request))));
assert.throws(() => wasm.calculate({ ...request, day: 30 }));
assert.throws(() => wasm.calculate({ ...request, typo: 1 }), /unknown field/);
assert.throws(() => wasm.calculate({ ...request, utc_offset_minute: 0 }), /unknown field/);
for (const hour of [1.5, true, Number.NaN, Infinity, null]) {
  assert.throws(() => wasm.calculate({ ...request, hour }));
}
for (const invalid of [null, undefined, [], '2024-02-10']) {
  assert.throws(() => wasm.calculate(invalid));
}
assert.throws(() => wasm.calculateJson('{'));
console.log('Wasm object/JSON contract and error smoke tests passed');
