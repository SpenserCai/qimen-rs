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
assert.throws(() => wasm.calculate({ ...request, day_boundary: { zi_start: null } }));
for (const hour of [1.5, true, Number.NaN, Infinity, null]) {
  assert.throws(() => wasm.calculate({ ...request, hour }));
}
for (const invalid of [null, undefined, [], '2024-02-10']) {
  assert.throws(() => wasm.calculate(invalid));
}
assert.throws(() => wasm.calculateJson('{'));

// User-supplied software screenshot: 2026-09-18 18:15, UTC+08:00.
const reference = { year: 2026, month: 9, day: 18, hour: 18, minute: 15 };
const allRules = {
  hidden_stems: 'duty_door_hour_stem_with_center_fallback',
  strength: 'classical_stars_and_five_elements',
  growth_stages: 'yang_forward_yin_reverse_fire_earth',
  punishments: 'six_instrument_branches',
  tombs: 'growth_stage_fire_earth',
  day_horse: 'day_branch_three_harmony',
  door_pressure: 'door_controls_palace',
};
assert.equal(chart.schema_version, '1.1');
assert.equal(Object.hasOwn(chart, 'extensions'), false);
assert.deepEqual(wasm.calculate({ ...request, extensions: {} }), chart);
assert.deepEqual(wasm.calculate({ ...request, extensions: { day_horse: null } }), chart);
const onlyHorse = wasm.calculate({ ...reference, extensions: { day_horse: allRules.day_horse } });
assert.deepEqual(Object.keys(onlyHorse.extensions), ['day_horse']);
assert.deepEqual(onlyHorse.extensions.day_horse.horse, { branch: 'si', palace: 4 });
assert.deepEqual(onlyHorse.horse, { branch: 'hai', palace: 6 });
const annotatedRequest = { ...reference, extensions: allRules };
const annotated = wasm.calculate(annotatedRequest);
assert.deepEqual(annotated, JSON.parse(wasm.calculateJson(JSON.stringify(annotatedRequest))));
assert.deepEqual(
  Object.fromEntries(Object.entries(annotated.extensions).map(([name, value]) => [name, value.rule])),
  allRules,
);
assert.deepEqual(annotated.extensions.hidden_stems.palaces.map(item => item.stem),
  ['ren', 'xin', 'geng', 'ji', 'wu', 'yi', 'bing', 'ding', 'gui']);
const traditional = wasm.calculate({ ...reference, extensions: { tombs: 'traditional_three_wonders' } });
const instrument = traditional.extensions.tombs.stems.find(item => item.placement.stem === 'geng');
assert.equal(instrument.tomb_branch, null);
assert.equal(instrument.is_in_tomb, null);
for (const extensions of [
  { typo: 'day_branch_three_harmony' }, { day_horse: 'automatic' },
  { day_horse: { day_branch_three_harmony: null } },
  { hidden_stems: true }, [], true, 'all', null,
]) {
  assert.throws(() => wasm.calculate({ ...reference, extensions }));
  assert.throws(() => wasm.calculateJson(JSON.stringify({ ...reference, extensions })));
}
console.log('Wasm object/JSON, optional annotations and error smoke tests passed');
