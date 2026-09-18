'use strict';

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const root = path.resolve(__dirname, '..');
const expected = [
  'linux-x64-gnu', 'linux-arm64-gnu', 'darwin-x64', 'darwin-arm64', 'win32-x64-msvc',
];

for (const platform of expected) {
  const artifact = path.join(root, 'npm', platform, `qimen.${platform}.node`);
  assert.ok(fs.existsSync(artifact), `Missing release artifact: ${artifact}`);
  assert.ok(fs.statSync(artifact).size > 0, `Empty release artifact: ${artifact}`);
}
assert.ok(fs.existsSync(path.join(root, 'native.cjs')), 'Missing generated Node loader');
console.log(`Verified ${expected.length} native platform packages and loader`);
