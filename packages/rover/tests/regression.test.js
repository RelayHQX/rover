import test from 'node:test';
import assert from 'node:assert/strict';
import {
  RelayRover,
  classify,
  generatePreview,
  processEmail,
  process,
  processBatch,
  processAsync,
  processBatchAsync,
  processWithRover,
  processBatchWithRover,
  attachRoverToFullEmail,
  toRoverInput,
  cleanBodyText,
  htmlToText,
  cleanEmailBody,
  normalizeEmail,
  extractSummary,
  RoverIndex,
  init,
} from '../dist/index.js';

test('1. Proton Verification Code Regression Test', () => {
  const email = {
    id: 'proton_verification_1',
    subject: 'Proton Verification Code',
    body: 'Your Proton verification code is 307402. Valid for 10 minutes.',
    from: 'security@proton.me',
    sender_name: 'Proton',
  };

  const result = process(email);

  // Exact requirements from Section 11:
  // - Category: otp
  // - Extraction category: Otp
  // - OTP code: 307402
  // - Service: Proton
  // - Purpose: Otp
  // - Security: authentication
  assert.equal(result.primary_category, 'otp');
  assert.equal(result.extraction.category, 'Otp');
  assert.equal(result.extraction.data.otp_code, '307402');
  assert.equal(result.extraction.data.service_name, 'Proton');
  assert.equal(result.classification.purpose, 'Otp');
  assert.equal(result.classification.security, 'authentication');
  assert.equal(result.classification.importance, 'urgent');
  assert.equal(result.classification.is_alert, true);

  // Standalone classifier check
  const cat = classify(email.subject, email.body);
  assert.equal(cat, 'Otp');
});

test('2. Shopping Order & Shipment Extraction', () => {
  const email = {
    id: 'shopping_1',
    subject: 'Your order #ORD-987654 is confirmed',
    body: 'Thank you for shopping with Amazon. Total: $49.99. Tracking number: 1Z9999999999999999. Expected delivery soon.',
    from: 'orders@amazon.com',
    sender_name: 'Amazon',
  };

  const result = process(email);
  assert.equal(result.primary_category, 'shopping');
  assert.equal(result.classification.purpose, 'Shopping');
  assert.equal(result.extraction.category, 'Shopping');
  assert.equal(result.extraction.data.order_id, 'ORD-987654');
  assert.equal(result.extraction.data.amount, '49.99');
  assert.equal(result.extraction.data.store, 'Amazon');
  assert.ok(result.preview.includes('$49.99'));
});

test('3. Payment & Money Transfer Extraction', () => {
  const email = {
    id: 'pay_1',
    subject: 'Payment of $25.00 successful',
    body: 'Hey Alex, You have successfully paid $25.00 to CoffeeShop with transaction id TXN_ABC1234567. Your balance is updated.',
    from: 'support@paymentapp.com',
    sender_name: 'PaymentApp',
  };

  const result = process(email);
  assert.equal(result.primary_category, 'payment');
  assert.equal(result.classification.purpose, 'Payment');
  assert.equal(result.extraction.category, 'Payment');
  assert.equal(result.extraction.data.amount, '25.00');
  assert.equal(result.extraction.data.currency, 'USD');
  assert.equal(result.extraction.data.transaction_id, 'TXN_ABC1234567');
  assert.equal(result.extraction.data.transaction_status, 'Successful');
});

test('4. Invoice Extraction', () => {
  const email = {
    id: 'inv_1',
    subject: 'Invoice INV-2026-99 from Cloud Host',
    body: 'Dear customer, your monthly invoice amount due is $120.00. Payment processed on 2026-11-20.',
    from: 'billing@cloudhost.com',
    sender_name: 'Cloud Host',
  };

  const result = process(email);
  assert.equal(result.classification.purpose, 'Invoice');
  assert.equal(result.extraction.category, 'Invoice');
  assert.equal(result.extraction.data.vendor, 'Cloud Host');
  assert.equal(result.extraction.data.invoice_id, 'INV-2026-99');
  assert.equal(result.extraction.data.amount, '120.00');
});

test('5. Developer Notification Extraction', () => {
  const email = {
    id: 'dev_1',
    subject: 'Build failed: @rhq/rover v0.1.5 release pipeline',
    body: 'GitHub Actions workflow build failed on commit a1b2c3d.',
    from: 'notifications@github.com',
    sender_name: 'GitHub Actions',
  };

  const result = process(email);
  assert.equal(result.classification.purpose, 'Developer');
  assert.equal(result.classification.importance, 'urgent');
  assert.equal(result.classification.is_alert, true);
  assert.equal(result.classification.security, 'build_failure');
  assert.equal(result.extraction.category, 'Developer');
  assert.equal(result.extraction.data.status, 'failed');
  assert.equal(result.extraction.data.package, '@rhq/rover');
  assert.equal(result.extraction.data.version, '0.1.5');
});

test('6. Security Alert Extraction', () => {
  const email = {
    id: 'sec_1',
    subject: 'Security alert: New login from Linux in Tokyo',
    body: 'We detected a new sign-in to your account. If this was not you, please secure your account immediately.',
    from: 'security@example.com',
    sender_name: 'Account Security',
  };

  const result = process(email);
  assert.equal(result.classification.purpose, 'SecurityAlert');
  assert.equal(result.classification.importance, 'high');
  assert.equal(result.classification.is_alert, true);
  assert.equal(result.classification.security, 'security_alert');
  assert.equal(result.extraction.category, 'SecurityAlert');
});

test('7. General Email Fallback', () => {
  const email = {
    id: 'gen_1',
    subject: 'Coffee tomorrow at 10am?',
    body: 'Hey, let us grab a coffee tomorrow morning and catch up.',
    from: 'friend@example.com',
    sender_name: 'Friend',
  };

  const result = process(email);
  assert.equal(result.classification.purpose, 'General');
  assert.equal(result.classification.importance, 'normal');
  assert.equal(result.classification.is_alert, false);
  assert.equal(result.classification.security, 'none');
  assert.equal(result.extraction.category, 'General');
});

test('8. Batch Processing & Async Processing', async () => {
  const emails = [
    { subject: 'Proton Verification Code', body: 'code: 123456' },
    { subject: 'Your order #112233 confirmed', body: 'Amount $10.00' },
  ];

  const batchResults = processBatch(emails);
  assert.equal(batchResults.length, 2);
  assert.equal(batchResults[0].extraction.category, 'Otp');
  assert.equal(batchResults[1].extraction.category, 'Shopping');

  const asyncSingle = await processAsync(emails[0]);
  assert.equal(asyncSingle.extraction.category, 'Otp');

  const asyncBatch = await processBatchAsync(emails);
  assert.equal(asyncBatch.length, 2);
});

test('9. Input Validation & Error Handling', () => {
  assert.throws(() => process(null), TypeError);
  assert.throws(() => process(undefined), TypeError);
  assert.throws(() => process(42), TypeError);

  // Empty strings are handled gracefully
  const emptyRes = process('');
  assert.equal(emptyRes.classification.purpose, 'General');
  assert.equal(emptyRes.cleanBody, '');

  const emptyClassify = classify('', '');
  assert.equal(emptyClassify, 'General');

  const emptyPreview = generatePreview('', '');
  assert.equal(emptyPreview, '');
});

test('10. Relay Mail Integration Methods (FullEmail / InboxMessage)', () => {
  const fullEmail = {
    id: 'full_1',
    from: 'service@company.com',
    subject: 'Proton Verification Code',
    text: 'Your code is 307402',
    date: '2026-11-23T12:00:00Z',
  };

  const attached = attachRoverToFullEmail(fullEmail);
  assert.ok(attached.rover);
  assert.equal(attached.rover.extraction.data.otp_code, '307402');

  const roverInput = toRoverInput(fullEmail);
  assert.equal(roverInput.id, 'full_1');
  assert.equal(roverInput.subject, 'Proton Verification Code');

  const batch = processBatchWithRover([fullEmail]);
  assert.equal(batch.length, 1);
  assert.equal(batch[0].extraction.data.otp_code, '307402');
});

test('11. Timeline Grouping (RoverIndex.buildTimelines)', () => {
  const emails = [
    { id: '1', primary_category: 'shopping', timeline_key: 'order:123', preview: 'Ordered' },
    { id: '2', primary_category: 'shopping', timeline_key: 'order:123', preview: 'Shipped' },
    { id: '3', primary_category: 'general', preview: 'Unlinked item' },
  ];

  const { timelines, unlinked } = RoverIndex.buildTimelines(emails);
  assert.ok(timelines['order:123']);
  assert.equal(timelines['order:123'].events.length, 2);
  assert.equal(unlinked.length, 1);
});
