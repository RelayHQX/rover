# @rhq/rover (Relay Rover)

> **Clean, Lightweight, TypeScript-First Email Intelligence Engine for React Native, Expo, Node.js, and Relay Mail.**  
> *Zero LLMs · Zero Cloud APIs · 100% On-Device Deterministic Understanding · Zero Native Dependencies*

[![npm version](https://img.shields.io/badge/npm-v0.1.5-blue.svg)](https://www.npmjs.com/package/@rhq/rover)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.0+-3178C6.svg)](https://www.typescriptlang.org/)
[![React Native](https://img.shields.io/badge/React%20Native-Hermes%20Direct-61DAFB.svg)](https://reactnative.dev)
[![Expo](https://img.shields.io/badge/Expo-Pure%20JS-000020.svg)](https://expo.dev)

---

## 1. Project Overview

Traditional email clients treat every message as generic unstructured text: they show only the sender, subject, and a raw HTML or text snippet, forcing users to open messages to find verification codes, tracking numbers, flight gates, or receipts.

`@rhq/rover` is the production email intelligence engine behind **Relay Mail**. Rover transforms raw emails into structured, typed intelligence on-device in microseconds.

Rover features a **TypeScript-First Architecture**:
- **Pure TypeScript / JavaScript** for React Native and Expo (Hermes) with **zero native module compilation**, zero Gradle/CocoaPods linking, zero JNI, zero JSI, and zero Rust/C++/Kotlin/Swift dependencies.
- **Separate Optional WASM Target** (`@rhq/rover/wasm`) for environments that explicitly require WebAssembly execution.

---

## 2. Features

- ⚡ **Zero Native Dependencies**: React Native and Expo execute Rover directly through standard JavaScript. No `pod install`, no NDK, no C++ compilation, and zero native module setup.
- 🔐 **OTP & Verification Codes**: High-precision extraction of one-time passwords, verification codes, expiration windows, and service identities (Proton, Nintendo, Google, Apple, GitHub, etc.).
- 📦 **Commerce & Order Tracking**: Instant detection of orders, tracking IDs, delivery status (`Delivered`, `Shipped`, `Processing`), item names, and totals.
- 💳 **Payments & Banking**: Extracts transaction IDs, amounts, currencies (USD, INR, EUR, GBP), payers, recipients, and settlement statuses.
- 🧾 **Invoices & Billing**: Extracts vendor names, invoice IDs, and amounts due.
- 🔄 **Subscriptions**: Detects recurring subscriptions, renewals, and membership updates.
- ✈️ **Travel & Bookings**: Detects flight itineraries, hotel reservations, and PNR booking codes.
- 🛠️ **Developer Notifications**: Identifies package publications, deployments, and build failures (`urgent` importance and `build_failure` security flags).
- 🚨 **Security Alerts**: Flags new logins, password changes, and account alerts with `high` importance.
- 👥 **Social Activity**: Extracts social platform reactions, comments, posts, and actor names (LinkedIn, Twitter, etc.).
- 👁️ **Smart Previews**: Generates human-readable preview lines (< 100 chars) tailored to each category.
- 🗂️ **Rover Index Timelines**: Groups related email events (e.g. order -> shipment -> delivery) into connected deterministic timelines.
- 🛡️ **100% Private & Local**: Zero email bytes or metadata ever leave the user's device.

---

## 3. Installation

Install `@rhq/rover` via your preferred package manager:

```bash
# npm
npm install @rhq/rover

# yarn
yarn add @rhq/rover

# pnpm
pnpm add @rhq/rover

# bun
bun add @rhq/rover
```

> **React Native & Expo Notice**: No native linking, CocoaPods installation, or Expo config plugins are required. `@rhq/rover` works out of the box in Expo Go, custom dev clients, and bare React Native applications.

---

## 4. Quick Start

```typescript
import { RelayRover } from '@rhq/rover';

const email = {
  id: 'msg_101',
  subject: 'Proton Verification Code',
  body: 'Your Proton verification code is 307402. Valid for 10 minutes.',
  from: 'security@proton.me',
  sender_name: 'Proton',
};

const result = RelayRover.process(email);

console.log(result.primary_category);
// => 'otp'

console.log(result.extraction.data);
// => {
//      otp_code: '307402',
//      service_name: 'Proton',
//      expiration_time: '10 minutes',
//      auth_purpose: 'Email Verification'
//    }

console.log(result.preview);
// => 'Proton verification code: 307402 (Expires in 10 minutes)'
```

---

## 5. React Native & Expo Usage

React Native bundlers (Metro) automatically resolve the package via the `"react-native"` export condition, executing pure TypeScript/JavaScript directly in the Hermes runtime with zero Node.js built-ins (`fs`, `path`, `url`).

```typescript
import React, { useMemo } from 'react';
import { View, Text, FlatList } from 'react-native';
import { processWithRover, attachRoverToFullEmail } from '@rhq/rover';

export function InboxItem({ message }) {
  const processed = useMemo(() => processWithRover(message), [message]);

  return (
    <View style={{ padding: 12 }}>
      <Text style={{ fontWeight: 'bold' }}>{processed.senderName}</Text>
      <Text>{processed.preview}</Text>
      {processed.extraction.category === 'Otp' && (
        <Text style={{ color: 'blue' }}>
          Code: {processed.extraction.data.otp_code}
        </Text>
      )}
    </View>
  );
}
```

---

## 6. Node.js Usage

`@rhq/rover` is distributed as standard ES modules with complete TypeScript declarations:

```typescript
import { RelayRover, processBatch } from '@rhq/rover';

const emails = [
  { subject: 'Your payment of $15.00 was successful', body: 'Transaction ID: TXN_881920' },
  { subject: 'Your order #ORD-4491 is confirmed', body: 'Shipped via Courier' }
];

const processed = processBatch(emails);
console.log(processed.map(p => ({ category: p.primary_category, preview: p.preview })));
```

---

## 7. TypeScript API Documentation

### Primary Engine Methods

#### `RelayRover.process(email: EmailRecordInput | string): ProcessedEmail`
Processes a single email record object or plain-text string. Automatically cleans HTML, extracts structured metadata, classifies intent, and attaches clean text.

#### `RelayRover.processBatch(emails: EmailRecordInput[] | string): ProcessedEmail[]`
Batch-processes multiple emails in a single pass. Accepts an array of email objects or a serialized JSON string.

#### `RelayRover.classify(subject: string, body: string): string`
Fast category detection returning the primary category name (e.g. `'Otp'`, `'Shopping'`, `'Payment'`, `'Social'`, `'Job'`, `'Invoice'`, `'General'`).

#### `RelayRover.generatePreview(subject: string, body: string): string`
Generates a concise, high-signal contextual preview line (< 100 characters).

#### `RelayRover.processAsync(email: EmailRecordInput | string): Promise<ProcessedEmail>`
Asynchronously processes a single email off the main execution frame.

#### `RelayRover.processBatchAsync(emails: EmailRecordInput[] | string): Promise<ProcessedEmail[]>`
Asynchronously batch-processes emails without blocking user interactions.

### Relay Mail Helpers

#### `toRoverInput(email: FullEmail | InboxMessage): EmailRecordInput`
Converts Relay Mail's `FullEmail` (RFC-style with `html`/`text`) or `InboxMessage` (lightweight list entry) into a normalized Rover input.

#### `processWithRover(email: FullEmail | InboxMessage): ProcessedEmail`
Directly processes a `FullEmail` or `InboxMessage`.

#### `processBatchWithRover(emails: (FullEmail | InboxMessage)[]): ProcessedEmail[]`
Batch processes an array of `FullEmail` or `InboxMessage` records.

#### `attachRoverToFullEmail(email: FullEmail): FullEmail`
Processes the email and attaches the computed `rover` result directly onto the `FullEmail` object.

### Content & Utility Functions

#### `htmlToText(html: string): string`
Strips HTML tags, script/style blocks, unescapes entities (`&nbsp;`, `&euro;`, `&amp;`), and normalizes whitespace into readable text.

#### `cleanEmailBody(email: EmailRecordInput | string): string`
Extracts and normalizes clean message body text from multipart or HTML email records.

#### `extractSummary(email: EmailRecordInput | string, maxLength?: number): string`
Extracts a clean snippet truncated to `maxLength` (defaults to 100 characters).

#### `RoverIndex.buildTimelines(processedEmails: ProcessedEmail[]): TimelineBuildResult`
Groups processed emails by their deterministic `timeline_key` into connected purchase, project, and activity timelines.

---

## 8. Input Data Format

Rover accepts multiple input formats:

### `EmailRecordInput`
```typescript
export interface EmailRecordInput {
  id?: string;
  sender?: string;
  sender_name?: string;
  from?: any;
  subject?: string;
  body?: string;
  textBody?: string;
  htmlBody?: string;
  content?: string;
  date?: string;
  received_at?: string;
  headers?: any;
}
```

### `FullEmail` (Relay Mail)
```typescript
export interface FullEmail {
  id: string;
  from?: any;
  to?: any;
  subject?: string;
  date?: string;
  timestamp?: number;
  internalDate?: string;
  html?: string;
  text?: string;
  snippet?: string;
  headers?: any;
  rover?: ProcessedEmail;
  [key: string]: any;
}
```

---

## 9. Output Format

Rover returns a strongly-typed `ProcessedEmail` object:

```typescript
export interface ProcessedEmail<T = any> {
  id: string;
  primary_category: string;
  matched_categories: string[];
  classification?: EmailClassification;
  extraction: {
    category: string;
    data: T;
  };
  preview: string;
  timeline_key?: string;
  cleanBody?: string;
  sender?: string;
  senderName?: string;
}

export interface EmailClassification {
  general_category: RoverGeneralCategory;  // 'primary' | 'social' | 'promotions' | 'updates' | 'forums' | 'spam'
  purpose: RoverCategory;                 // 'Otp' | 'Shopping' | 'Payment' | 'Invoice' | ...
  importance: RoverImportance;            // 'urgent' | 'high' | 'normal' | 'low'
  is_alert: boolean;
  security: RoverSecurityClassification;  // 'authentication' | 'password_reset' | 'security_alert' | 'none' | ...
  confidence: number;
  factors: ConfidenceFactors;
  secondary_purposes: RoverCategory[];
}
```

### Extraction Data Examples

- **`Otp`**: `{ otp_code: '307402', service_name: 'Proton', expiration_time: '10 minutes', auth_purpose: 'Email Verification' }`
- **`Shopping`**: `{ store: 'Amazon', order_id: 'ORD-987654', amount: '49.99', delivery_status: 'Shipped', tracking_number: '1Z999...' }`
- **`Payment`**: `{ amount: '25.00', currency: 'USD', recipient: 'CoffeeShop', transaction_id: 'TXN_ABC123', transaction_status: 'Successful' }`
- **`Invoice`**: `{ vendor: 'Cloud Host', invoice_id: 'INV-2026-99', amount: '120.00', due_date: null }`
- **`Developer`**: `{ package: '@rhq/rover', version: '0.1.5', status: 'failed', service: 'GitHub Actions' }`

---

## 10. Batch Processing

Rover is designed for high-throughput sync operations. You can process hundreds of messages in a single call:

```typescript
import { RelayRover } from '@rhq/rover';

// Synchronous batch
const results = RelayRover.processBatch(inboxEmails);

// Asynchronous batch (off the UI frame)
const asyncResults = await RelayRover.processBatchAsync(inboxEmails);
```

---

## 11. Optional WASM Distribution Target

For Web or backend environments that explicitly request WebAssembly execution, Rover maintains a separate, verified WASM entry point:

```typescript
import { RelayRoverWasm, init } from '@rhq/rover/wasm';

// Optional: initialize with custom bytes or rely on bundled eng/relay_rover.wasm
init();

const result = RelayRoverWasm.process({
  subject: 'Your order is confirmed',
  body: 'Order #ORD-12345 total: $50.00',
});
```

*Note: The default `@rhq/rover` entry point and the React Native entry point `@rhq/rover/react-native` do not require or load WebAssembly.*

---

## 12. Supported Platforms

| Platform | Runtime | Engine Type | Native Linking |
| :--- | :--- | :--- | :--- |
| **React Native** | Hermes / JSC | Pure TypeScript | **None** (Zero setup) |
| **Expo** | Expo Go / Dev Client | Pure TypeScript | **None** (Zero setup) |
| **Node.js** | Node.js 18+ (ESM) | Pure TypeScript | **None** |
| **Web / Browsers** | Chrome, Safari, Firefox | Pure TypeScript | **None** |
| **Optional WASM** | Web / Node.js | WebAssembly (Rust) | In-Memory WASM |

---

## 13. Privacy and Local Processing

All message parsing, regex evaluation, HTML sanitization, and entity extraction occur strictly in-memory on the host device.
- **Zero Cloud LLM calls**
- **Zero telemetry or external network requests**
- **Zero persistence of decrypted email contents**

---

## 14. Development Instructions

Clone the repository and install dependencies:

```bash
git clone https://github.com/relayhq/rover.git
cd rover/packages/rover
npm install
```

---

## 15. Build Instructions

Compile the TypeScript source code into `dist/`:

```bash
npm run build
```

This generates:
- `dist/index.js` & `dist/index.d.ts` (Standard ESM entry point)
- `dist/react-native.js` & `dist/react-native.d.ts` (React Native / Hermes entry point)
- `dist/wasm.js` & `dist/wasm.d.ts` (Optional WASM distribution)

---

## 16. Testing Instructions

Run the complete test suite:

```bash
npm test
```

The test suite validates:
- Proton verification code regression test (`Subject: Proton Verification Code`, code `307402`, purpose `Otp`, security `authentication`).
- Extraction accuracy across shopping, payments, invoices, developer notifications, security alerts, and general fallbacks.
- Parity across the real-world 5-category email dataset (`sample_5_types.json`).
- Strict zero-dependency execution in React Native (no `fs`, `path`, `url`, `WebAssembly`).
- Optional WebAssembly entry point execution.

---

## 17. Package Structure

```text
packages/rover/
├── dist/                      # Compiled JS, type definitions (.d.ts), and source maps
│   ├── core/                  # Extractor, classifier, normalizer, preview, timeline
│   ├── types/                 # Exported TypeScript interfaces
│   ├── utils/                 # HTML and string sanitizers
│   ├── index.js               # Main package entry point
│   ├── react-native.js        # Pure TS/JS entry point for React Native / Hermes
│   └── wasm.js                # Optional WASM entry point
├── eng/
│   └── relay_rover.wasm       # Precompiled WebAssembly binary for @rhq/rover/wasm
├── src/                       # TypeScript source files
│   ├── core/
│   ├── extractors/
│   ├── types/
│   ├── utils/
│   ├── index.ts
│   ├── react-native.ts
│   └── wasm.ts
├── tests/                     # Test suites (regression, react-native, samples, wasm)
├── package.json               # Package configuration & export conditions
├── tsconfig.json              # TypeScript build configuration
├── README.md                  # Documentation
├── LICENSE                    # MIT License
├── .gitignore
└── .npmignore
```

---

## 18. License

MIT © [Relay Team](https://relaymail.online)
