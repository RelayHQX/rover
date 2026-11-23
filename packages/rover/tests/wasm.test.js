import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import * as WasmTarget from '../dist/wasm.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

test('Optional WASM engine loads and processes emails', () => {
  const wasmPath = path.resolve(__dirname, '../eng/relay_rover.wasm');
  assert.ok(fs.existsSync(wasmPath), 'eng/relay_rover.wasm must exist');

  const wasmBytes = fs.readFileSync(wasmPath);
  const exports = WasmTarget.init(wasmBytes);
  assert.ok(exports, 'WASM engine must initialize with valid binary');

  // Test classify via WASM
  const cat = WasmTarget.classify('Proton Verification Code', 'Your verification code is 307402');
  assert.ok(typeof cat === 'string');
  assert.equal(cat, 'OTP');

  // Test generatePreview via WASM
  const preview = WasmTarget.generatePreview('Proton Verification Code', 'Your verification code is 307402');
  assert.ok(typeof preview === 'string');
  assert.ok(preview.includes('307402'));

  // Test process via WASM
  const processed = WasmTarget.process({
    subject: 'Your order #ORD-12345 confirmed',
    body: 'Total amount: $50.00',
  });
  assert.ok(processed.primary_category);
});
