import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

test('React Native entry point has ZERO Node.js and WASM dependencies', () => {
  const rnCode = fs.readFileSync(path.resolve(__dirname, '../dist/react-native.js'), 'utf-8');

  // Must not import or require Node.js built-ins
  assert.ok(!rnCode.includes("from 'fs'"), "Must not import 'fs'");
  assert.ok(!rnCode.includes("from 'node:fs'"), "Must not import 'node:fs'");
  assert.ok(!rnCode.includes("from 'path'"), "Must not import 'path'");
  assert.ok(!rnCode.includes("from 'node:path'"), "Must not import 'node:path'");
  assert.ok(!rnCode.includes("from 'url'"), "Must not import 'url'");
  assert.ok(!rnCode.includes("from 'node:url'"), "Must not import 'node:url'");
  assert.ok(!rnCode.includes('WebAssembly.Module'), 'Must not require WebAssembly');

  // Check the files exported by index.js that react-native.js depends on
  const indexCode = fs.readFileSync(path.resolve(__dirname, '../dist/index.js'), 'utf-8');
  assert.ok(!indexCode.includes("from 'fs'"), "index.js must not import 'fs'");
  assert.ok(!indexCode.includes("from 'node:fs'"), "index.js must not import 'node:fs'");
  assert.ok(!indexCode.includes("from 'path'"), "index.js must not import 'path'");
  assert.ok(!indexCode.includes("from 'node:path'"), "index.js must not import 'node:path'");
  assert.ok(!indexCode.includes("from 'url'"), "index.js must not import 'url'");
  assert.ok(!indexCode.includes("from 'node:url'"), "index.js must not import 'node:url'");
  assert.ok(!indexCode.includes('WebAssembly'), 'index.js must not import WebAssembly');
});

test('React Native module exports identical API to standard entry point', async () => {
  const RN = await import('../dist/react-native.js');

  assert.equal(typeof RN.classify, 'function');
  assert.equal(typeof RN.generatePreview, 'function');
  assert.equal(typeof RN.processEmail, 'function');
  assert.equal(typeof RN.process, 'function');
  assert.equal(typeof RN.processBatch, 'function');
  assert.equal(typeof RN.processAsync, 'function');
  assert.equal(typeof RN.processBatchAsync, 'function');
  assert.equal(typeof RN.htmlToText, 'function');
  assert.equal(typeof RN.cleanEmailBody, 'function');
  assert.equal(typeof RN.normalizeEmail, 'function');
  assert.equal(typeof RN.extractSummary, 'function');
  assert.equal(typeof RN.toRoverInput, 'function');
  assert.equal(typeof RN.cleanBodyText, 'function');
  assert.equal(typeof RN.processWithRover, 'function');
  assert.equal(typeof RN.processBatchWithRover, 'function');
  assert.equal(typeof RN.attachRoverToFullEmail, 'function');
  assert.equal(typeof RN.RoverIndex, 'object');
  assert.equal(typeof RN.RelayRover, 'object');
  assert.equal(typeof RN.default, 'object');

  // Test execution through react-native entry point
  const result = RN.process({
    subject: 'Proton Verification Code',
    body: 'Code: 307402',
  });
  assert.equal(result.extraction.category, 'Otp');
  assert.equal(result.extraction.data.otp_code, '307402');
});
