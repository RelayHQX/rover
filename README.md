# Relay Rover Monorepo

> **Clean, Lightweight, TypeScript-First Email Intelligence Engine for Relay Mail, React Native, Expo, and Web.**  
> *Zero LLMs · Zero Cloud APIs · 100% On-Device Deterministic Understanding · Zero Native Dependencies*

[![npm version](https://img.shields.io/badge/npm-v0.1.5-blue.svg)](https://www.npmjs.com/package/@rhq/rover)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.0+-3178C6.svg)](https://www.typescriptlang.org/)
[![React Native](https://img.shields.io/badge/React%20Native-Hermes%20Direct-61DAFB.svg)](https://reactnative.dev)
[![Expo](https://img.shields.io/badge/Expo-Pure%20JS-000020.svg)](https://expo.dev)

---

## What is Relay Rover?

Most email applications treat emails as generic unstructured text, forcing users to open messages to find verification codes, tracking numbers, flight gates, or receipts.

**Rover** is the deterministic email intelligence engine behind [Relay Mail](https://blogs.relaymail.online/blog/rover). Rover parses and extracts structured metadata on-device without cloud LLMs, network latency, or recurring API costs.

### Architecture Overview

Rover adopts a **TypeScript-First Architecture**:
- **Pure TypeScript / JavaScript** for React Native and Expo (Hermes) with **zero native module compilation**, zero Gradle/CocoaPods linking, zero JNI, zero JSI, and zero Rust/C++/Kotlin/Swift dependencies.
- **Precompiled WebAssembly Engine** (`@rhq/rover/wasm`) retained for web and Node.js environments requiring WASM execution.
- **Rust Core** (`relay-rover`) retained cleanly for compilation to WASM, with all UniFFI and obsolete native binding scaffolding removed.

```text
Incoming Email ──► Pure TypeScript Engine (Hermes / Node) ──► Purpose Category + Dual-Layer Classification
                                                         ──► Deep Typed Entity Extraction (OTP, Order, Txn)
                                                         ──► Semantic Preview Line (< 100 chars)
                                                         ──► Timeline Key Generation
```

---

## Monorepo Layout

```text
rover/
├── packages/
│   └── rover/                 # @rhq/rover npm package (TypeScript-First architecture)
│       ├── dist/              # Compiled JavaScript, .d.ts declarations, and source maps
│       ├── eng/               # WebAssembly binaries (eng/relay_rover.wasm)
│       ├── src/               # Pure TypeScript source code
│       │   ├── core/          # Normalizer, extractor, classifier, preview, timeline
│       │   ├── types/         # Strongly-typed models (FullEmail, InboxMessage, ProcessedEmail)
│       │   ├── utils/         # HTML sanitization and entity decoding
│       │   ├── index.ts       # Main package entry point
│       │   ├── react-native.ts# Dedicated pure JS entry point for React Native
│       │   └── wasm.ts        # Optional WASM entry point
│       ├── tests/             # Regression, parity, and React Native import tests
│       └── package.json       # Configured with "react-native", "main", and "./wasm" exports
├── src/                       # Rust crate for WASM builds (relay-rover)
├── tests/                     # Cross-platform parity test suite
├── Cargo.toml                 # Rust crate manifest (cdylib, rlib)
├── sample_5_types.json        # Real-world 5-category evaluation dataset
└── README.md
```

---

## Quick Start with `@rhq/rover`

```bash
npm install @rhq/rover
```

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

console.log(result.primary_category); // 'otp'
console.log(result.extraction.data.otp_code); // '307402'
console.log(result.preview); // 'Proton verification code: 307402 (Expires in 10 minutes)'
```

---

## Validation & Test Commands

```bash
# 1. Build TypeScript package
cd packages/rover && npm run build

# 2. Run TypeScript test suite (100% pass)
npm test

# 3. Dry-run npm pack
npm pack --dry-run

# 4. Run Rust unit tests
cd ../.. && cargo test --quiet
```

---

## License

MIT © [Relay Team](https://relaymail.online)
