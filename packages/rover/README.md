# @rhq/rover (Relay Rover)

> **High-Performance WebAssembly Email Intelligence Engine for Real-World Email Clients, React Native, and Relay Mail.**  
> *Zero LLMs. Zero cloud API calls. 100% on-device deterministic understanding.*

[![npm version](https://img.shields.io/badge/npm-v0.1.0-blue.svg)](https://www.npmjs.com/package/@rhq/rover)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-WASM32-purple.svg)](https://webassembly.org/)
[![Throughput](https://img.shields.io/badge/Throughput-4%2C939%20emails%2Fsec-success.svg)](#performance-benchmarks)

---

## The Problem: Emails Are More Than Messages

Traditional email clients treat every message the same: they show sender, subject, and an excerpt of raw text or messy HTML, forcing the user to manually open the email to hunt for verification codes, tracking numbers, flight gates, or payment receipts.

> *"Gmail gives Relay the emails. Rover gives Relay an understanding of those emails."*  
> — [*Meet Rover: The Intelligence Behind Relay Mail*](https://blogs.relaymail.online/blog/rover)

`@rhq/rover` is the production email intelligence engine behind **Relay Mail**. Written in Rust and compiled directly to WebAssembly (`eng/relay_rover.wasm`), Rover runs client-side inside the user's application with microsecond latency, zero operational costs, and total data privacy.

---

## Key Capabilities

- **Dual-Layer Classification:**
  - **Folder Layer (General):** `Primary`, `Updates`, `Social`, `Promotions`, `Forums`.
  - **Action Layer (Purpose):** `Authentication` (OTP), `Transactions` (Payment/Invoice), `Deliveries` (Shopping), `Jobs`, `SecurityAlerts`, `Travel`, `Subscriptions`, `Newsletters`.
- **Urgency & Alert Engine:** Automatically flags critical emails with `is_alert: true` and urgency tiers (`Urgent`, `High`, `Normal`, `Low`) for real-time notification banners and inbox badges.
- **Deep Deterministic Extraction:**
  - **OTPs:** Extract code, service, expiration timeframe, and auth purpose (e.g. `0313`, `Nintendo`, `24 hours`).
  - **Orders & Deliveries:** Extract order ID, merchant, product, amount, tax/GST, order date, and delivery status.
  - **Payments & Receipts:** Extract amount, currency, payer, recipient, transaction ID, date, time, and payment method.
  - **Job Vacancies & Alerts:** Extract company, position, location, job vacancy count, and application URLs.
  - **Social & Alerts:** Extract actor, interaction type, target post URL, or CI/CD build failure details.
- **Semantic Contextual Previews:** Generates clean, informative preview lines (< 100 characters) tailored to the category (e.g., `₹29.00 · Special Airdrop 412 (Google Play)` or `Job Alert · 8 React Native Developer jobs at Webomindapps`).
- **The Rover Intelligence Index:** Computes `timeline_key` identifiers to automatically group related emails (Order Confirmation $\rightarrow$ Payment Receipt $\rightarrow$ Shipment $\rightarrow$ Delivery) into connected purchase and activity timelines.
- **HTML Sanitization & Normalization:** Built-in fast HTML-to-text parser that strips `<style>`, scripts, and tracking markup without third-party dependencies.
- **Blistering Throughput:** Processes over **4,900 emails/second** in WebAssembly and **9,200 emails/second** natively.

---

## Installation

```bash
npm install @rhq/rover
# or with pnpm
pnpm add @rhq/rover
# or with yarn
yarn add @rhq/rover
```

---

## Integrating Into an Email Client Application

### 1. Ingesting Real-World Raw Emails (IMAP, Gmail API, JMAP)

In a production email application, incoming emails contain messy HTML, multi-part bodies, and complex header structures. Rover accepts raw email objects, normalizes headers, sanitizes HTML, and extracts intelligence in one call:

```javascript
import { RelayRover } from '@rhq/rover';

// Example raw email payload from Gmail API / IMAP / backend sync
const rawEmail = {
  id: "msg_18b5ef21a4",
  from: "Google Play <googleplay-noreply@google.com>",
  subject: "Your Google Play Order Receipt from 27 Oct 2026",
  date: "2026-10-27T12:29:12+05:30",
  htmlBody: `
    <div>
      <style>.btn { color: red; }</style>
      <h1>Order number: GPA.3399-0949-2150-45472</h1>
      <p>Special Airdrop 412 (Free Fire MAX x NARUTO) ₹29.00</p>
      <p>Total: ₹29.00 (Includes GST of ₹4.42)</p>
      <p>Payment method: Google Play balance</p>
    </div>
  `
};

// Process email with Rover
const processed = RelayRover.process(rawEmail);

console.log(processed.primary_category); 
// "shopping"

console.log(processed.preview); 
// "₹29.00 · Special Airdrop 412 (Google Play)"

console.log(processed.classification);
// {
//   general_category: "updates",
//   purpose: "shopping",
//   importance: "normal",
//   is_alert: false,
//   security: "none",
//   confidence: 0.95
// }

console.log(processed.extraction.data);
// {
//   store: "Google Play",
//   order_id: "GPA.3399-0949-2150-45472",
//   product: "Special Airdrop 412",
//   amount: "29.00",
//   currency: "INR",
//   tax: "4.42",
//   order_date: "27 Oct 2026",
//   payment_method: "Google Play balance",
//   timeline_key: "order:GPA.3399-0949-2150-45472"
// }
```

---

### 2. High-Speed Batch Ingestion (Initial Sync & Background Fetch)

When your app syncs an inbox folder or executes a background fetch, pass the array of emails to `processBatch()`. The entire batch is processed in WebAssembly memory in a single invocation without round-trips:

```javascript
import { RelayRover } from '@rhq/rover';

// Syncing 100+ emails from the local SQLite store or IMAP cache
async function onInboxSync(incomingMessages) {
  const startTime = performance.now();

  // Process 100 emails in ~20 ms (~4,900 emails/sec)
  const results = RelayRover.processBatch(incomingMessages);

  const duration = (performance.now() - startTime).toFixed(1);
  console.log(`Classified & extracted ${results.length} emails in ${duration} ms`);

  // Persist structured metadata into your SQLite or Realm database
  for (const item of results) {
    await db.emails.update(item.id, {
      category: item.primary_category,
      folder: item.classification.general_category,
      previewText: item.preview,
      importance: item.classification.importance,
      isAlert: item.classification.is_alert,
      timelineKey: item.timeline_key,
      extractedData: JSON.stringify(item.extraction.data),
    });
  }
}
```

---

### 3. Rendering Actionable Inbox Cards & Action Banners

Rover provides structured fields that allow your UI to render rich, actionable elements directly in the inbox feed without opening the email:

#### A. One-Tap OTP Copy Banner
```tsx
import React from 'react';
import { View, Text, TouchableOpacity, Clipboard } from 'react-native';

export function OtpActionBanner({ processedEmail }) {
  if (processedEmail.primary_category !== 'otp') return null;

  const { otp_code, service_name, expiration_time } = processedEmail.extraction.data;
  if (!otp_code) return null;

  return (
    <View style={{ backgroundColor: '#EEF2FF', padding: 12, borderRadius: 8, flexDirection: 'row', alignItems: 'center' }}>
      <View style={{ flex: 1 }}>
        <Text style={{ fontWeight: 'bold', color: '#1E40AF' }}>
          {service_name || 'Authentication'} Verification Code
        </Text>
        <Text style={{ fontSize: 24, letterSpacing: 4, fontWeight: '700', color: '#1E1B4B' }}>
          {otp_code}
        </Text>
        {expiration_time && (
          <Text style={{ fontSize: 12, color: '#6B7280' }}>Expires in {expiration_time}</Text>
        )}
      </View>
      <TouchableOpacity 
        onPress={() => Clipboard.setString(otp_code)}
        style={{ backgroundColor: '#3B82F6', paddingHorizontal: 16, paddingVertical: 8, borderRadius: 6 }}
      >
        <Text style={{ color: '#fff', fontWeight: '600' }}>Copy</Text>
      </TouchableOpacity>
    </View>
  );
}
```

#### B. Delivery Tracking Card
```tsx
export function DeliveryCard({ processedEmail }) {
  if (processedEmail.primary_category !== 'shopping') return null;
  const { store, product, tracking_number, delivery_status } = processedEmail.extraction.data;

  return (
    <View style={{ padding: 12, borderWidth: 1, borderColor: '#E5E7EB', borderRadius: 8 }}>
      <Text style={{ fontSize: 14, color: '#4B5563' }}>{store} · {delivery_status}</Text>
      <Text style={{ fontSize: 16, fontWeight: 'bold' }}>{product}</Text>
      {tracking_number && (
        <TouchableOpacity onPress={() => openTracking(tracking_number)}>
          <Text style={{ color: '#2563EB', marginTop: 4 }}>Track Package: {tracking_number}</Text>
        </TouchableOpacity>
      )}
    </View>
  );
}
```

---

### 4. Connecting Fragmented Emails with the Rover Intelligence Index

Purchases, trips, and projects often scatter multiple emails over days or weeks:
1. *Order Confirmation*
2. *Payment Receipt*
3. *Shipping Update*
4. *Delivered Notification*

Rover provides `RoverIndex` to automatically connect these related communications into unified activity timelines:

```javascript
import { RelayRover, RoverIndex } from '@rhq/rover';

// Array of all emails in the inbox
const processedEmails = RelayRover.processBatch(inboxEmails);

// Group related emails by timeline key
const { timelines, unlinked } = RoverIndex.buildTimelines(processedEmails);

// Inspect continuous order timeline:
console.log(timelines["order:GPA.3399-0949-2150-45472"]);
/*
{
  timelineKey: "order:GPA.3399-0949-2150-45472",
  primaryCategory: "shopping",
  events: [
    { preview: "Order Confirmed · Special Airdrop 412", date: "2026-10-27" },
    { preview: "₹29.00 · Google Play receipt", date: "2026-10-27" },
    { preview: "In Transit · Special Airdrop 412", date: "2026-10-28" }
  ]
}
*/
```

---

### 5. Multi-Folder Inbox Navigation

Use the dual-layer classification to power your email client's primary navigation tabs and sub-category filters:

```typescript
// Filter emails for the "Primary" or "Updates" folder tab
const primaryInbox = allEmails.filter(
  email => email.classification.general_category === 'primary'
);

const updatesTab = allEmails.filter(
  email => email.classification.general_category === 'updates'
);

// Filter by actionable purpose chip in your search bar
const allOrders = allEmails.filter(
  email => email.primary_category === 'shopping'
);

const pendingJobs = allEmails.filter(
  email => email.primary_category === 'job'
);
```

---

## Complete API Reference

### `RelayRover.process(email: EmailRecordInput | string): ProcessedEmail`
Processes an email object or text string. Automatically handles HTML sanitization, header normalization, category detection, typed entity extraction, and contextual preview generation.

### `RelayRover.processBatch(emails: EmailRecordInput[] | string): ProcessedEmail[]`
Processes an array of emails in a single linear WebAssembly memory call. Designed for initial inbox loads and background sync workers.

### `RelayRover.classify(subject: string, body: string): string`
Ultra-fast sub-millisecond purpose category classification without full extraction.

### `RelayRover.generatePreview(subject: string, body: string): string`
Generates a concise contextual summary line (< 100 characters) for push notifications or inbox row subtitles.

### `RoverIndex.buildTimelines(processedEmails: ProcessedEmail[]): TimelineBuildResult`
Groups an array of processed emails into connected timelines (`timelines`) and standalone emails (`unlinked`) using deterministic timeline keys (`order:*`, `payment:*`, `travel:*`, `invoice:*`).

### `cleanEmailBody(email: EmailRecordInput | string): string`
Extracts readable plain-text, strips script and style blocks, unescapes entities, and preserves line breaks.

### `htmlToText(html: string): string`
Fast WASM-powered HTML-to-plain-text sanitizer.

### `normalizeEmail(email: EmailRecordInput): NormalizedEmail`
Standardizes various header formats (`from`, `sender`, `sender_name`, `date`, `received_at`) into a uniform record.

---

## TypeScript Typings & Extraction Schemas

All types are exported from `@rhq/rover`:

```typescript
import type { 
  ProcessedEmail,
  EmailClassification,
  GeneralCategory,
  Importance,
  SecurityClassification,
  PaymentExtraction,
  OtpExtraction,
  JobExtraction,
  ShoppingExtraction,
  SocialExtraction
} from '@rhq/rover';
```

### Extraction Types Summary

- **`OtpExtraction`**: `otp_code`, `service_name`, `expiration_time`, `auth_purpose`, `action_url`
- **`ShoppingExtraction`**: `store`, `order_id`, `product`, `amount`, `currency`, `tax`, `order_date`, `delivery_status`, `tracking_number`, `courier`, `payment_method`, `timeline_key`
- **`PaymentExtraction`**: `amount`, `currency`, `payer`, `recipient`, `transaction_id`, `transaction_date`, `transaction_time`, `payment_method`, `transaction_status`, `timeline_key`
- **`JobExtraction`**: `company`, `position`, `location`, `salary`, `job_count`, `is_job_alert`, `job_url`, `application_deadline`, `interview_date`
- **`SocialExtraction`**: `platform`, `activity_type`, `relevant_entity`, `actor`, `post_url`
- **`SecurityAlertExtraction`**: `alert_type`, `target_resource`, `severity`, `action_required`, `timeline_key`

---

## Performance & Architecture Comparison

| Metric / Feature | Relay Rover (`@rhq/rover`) | Cloud LLM (e.g. GPT-4o / Claude) |
|---|---|---|
| **Latency per Email** | **~200 µs** (0.2 ms) | **1,200 ms – 3,500 ms** |
| **Throughput** | **4,900+ emails/sec** | 2 – 10 emails/sec (rate limited) |
| **Cost per 100,000 Emails** | **$0.00 (Free, runs on-device)** | **$1,500 – $3,000+** in tokens |
| **Network Dependency** | **100% Offline Capable** | Requires active internet & API keys |
| **Privacy & Compliance** | **Zero Data Leaves Device** | Full user email sent to cloud |
| **Determinism** | **100% Consistent (Zero Hallucination)** | Variable responses, model drift |

---

## Compiling & Updating the Engine

To rebuild the WebAssembly binary from Rust source:

```bash
# 1. Compile optimized release target
cargo build --target wasm32-unknown-unknown --release

# 2. Copy compiled binary into package eng/ directory
cp target/wasm32-unknown-unknown/release/relay_rover.wasm packages/rover/eng/relay_rover.wasm

# 3. Run validation tests
node test_rover.js
```

---

## License

MIT © [Relay](https://blogs.relaymail.online)
