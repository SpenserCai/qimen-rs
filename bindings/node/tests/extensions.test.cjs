'use strict';

const assert = require('node:assert/strict');
const { test } = require('node:test');
const { calculate, calculateJson } = require('../index.cjs');

// User-supplied software screenshot: 2026-09-18 18:15, UTC+08:00.
const request = { year: 2026, month: 9, day: 18, hour: 18, minute: 15 };
const allRules = Object.freeze({
  hidden_stems: 'duty_door_hour_stem_with_center_fallback',
  strength: 'classical_stars_and_five_elements',
  growth_stages: 'yang_forward_yin_reverse_fire_earth',
  punishments: 'six_instrument_branches',
  tombs: 'growth_stage_fire_earth',
  day_horse: 'day_branch_three_harmony',
  door_pressure: 'door_controls_palace',
});

test('default and empty options omit extensions', () => {
  const base = calculate(request);
  assert.equal(base.schema_version, '1.1');
  assert.equal(Object.hasOwn(base, 'extensions'), false);
  assert.deepEqual(calculate({ ...request, extensions: {} }), base);
  assert.deepEqual(calculate({ ...request, extensions: { day_horse: null } }), base);
});

test('day horse is independent of the base hour horse', () => {
  const chart = calculate({ ...request, extensions: { day_horse: allRules.day_horse } });
  assert.deepEqual(chart.horse, { branch: 'hai', palace: 6 });
  assert.deepEqual(chart.extensions.day_horse.horse, { branch: 'si', palace: 4 });
  assert.deepEqual(Object.keys(chart.extensions), ['day_horse']);
});

test('all conventions survive the object and JSON interfaces', () => {
  const extended = Object.freeze({ ...request, extensions: allRules });
  const chart = calculate(extended);
  assert.deepEqual(chart, JSON.parse(calculateJson(JSON.stringify(extended))));
  assert.deepEqual(
    Object.fromEntries(Object.entries(chart.extensions).map(([name, value]) => [name, value.rule])),
    allRules,
  );
  assert.deepEqual(chart.extensions.hidden_stems.palaces.map(item => item.stem),
    ['ren', 'xin', 'geng', 'ji', 'wu', 'yi', 'bing', 'ding', 'gui']);
});

test('three-wonders tombs retain null for instruments outside that convention', () => {
  const chart = calculate({ ...request, extensions: { tombs: 'traditional_three_wonders' } });
  const instrument = chart.extensions.tombs.stems.find(item => item.placement.stem === 'geng');
  assert.equal(instrument.tomb_branch, null);
  assert.equal(instrument.is_in_tomb, null);
});

test('unknown extension names, rules and invalid shapes fail at the Rust boundary', () => {
  for (const extensions of [
    { typo: 'day_branch_three_harmony' }, { day_horse: 'automatic' },
    { day_horse: { day_branch_three_harmony: null } },
    { hidden_stems: true }, [], true, 'all', null,
  ]) {
    assert.throws(() => calculate({ ...request, extensions }), { code: 'InvalidArg' });
  }
});
