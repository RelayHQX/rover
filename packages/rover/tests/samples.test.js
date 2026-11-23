import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { process, classify, generatePreview } from '../dist/index.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const samplePath = path.resolve(__dirname, '../../../sample_5_types.json');

test('Sample 5 Types Real-World Email Parity', () => {
  const sample = JSON.parse(fs.readFileSync(samplePath, 'utf-8'));
  assert.equal(sample.length, 5);

  // 1. LinkedIn -> Social
  const linkedin = process(sample[0]);
  assert.equal(linkedin.classification.purpose, 'Social');
  assert.equal(linkedin.extraction.category, 'Social');
  assert.equal(linkedin.extraction.data.platform, 'LinkedIn');
  assert.ok(linkedin.preview.includes('LinkedIn'));

  // 2. Indeed -> Job
  const indeed = process(sample[1]);
  assert.equal(indeed.classification.purpose, 'Job');
  assert.equal(indeed.extraction.category, 'Job');
  assert.ok(indeed.preview.includes('Job Alert'));

  // 3. Nintendo -> Otp
  const nintendo = process(sample[2]);
  assert.equal(nintendo.classification.purpose, 'Otp');
  assert.equal(nintendo.extraction.category, 'Otp');
  assert.equal(nintendo.extraction.data.otp_code, '0313');
  assert.equal(nintendo.extraction.data.service_name, 'Nintendo');

  // 4. FamApp -> Payment
  const famapp = process(sample[3]);
  assert.equal(famapp.classification.purpose, 'Payment');
  assert.equal(famapp.extraction.category, 'Payment');
  assert.equal(famapp.extraction.data.amount, '10.0');
  assert.equal(famapp.extraction.data.currency, 'INR');
  assert.equal(famapp.extraction.data.transaction_id, 'FMPIB6973726141');
  assert.ok(famapp.preview.includes('₹10.0 paid to DigiGold'));

  // 5. Google Play -> Shopping
  const googleplay = process(sample[4]);
  assert.equal(googleplay.classification.purpose, 'Shopping');
  assert.equal(googleplay.extraction.category, 'Shopping');
  assert.equal(googleplay.extraction.data.order_id, 'GPA.3399-0949-2150-45472');
  assert.equal(googleplay.extraction.data.amount, '29.00');
  assert.ok(googleplay.preview.includes('₹29.00'));
});
