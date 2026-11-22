# Relay Rover Engine

> **High-Performance, Zero-LLM Deterministic Email Intelligence Engine for Relay Mail, React Native (Hermes), Expo, Web, and Edge Services.**  
> *100% Deterministic · Sub-Millisecond Latency · On-Device · Zero Breaking Changes*  

[![Blog](https://img.shields.io/badge/Blog-Meet%20Rover%3A%20The%20Intelligence%20Behind%20Relay%20Mail-8A2BE2.svg)](https://blogs.relaymail.online/blog/rover)
[![React Native](https://img.shields.io/badge/React%20Native-Hermes%20%2B%20JSI-61DAFB.svg)](https://reactnative.dev)
[![Expo](https://img.shields.io/badge/Expo-Modules%20API-000020.svg)](https://expo.dev)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-WASM32-purple.svg)](https://webassembly.org/)
[![Category Accuracy](https://img.shields.io/badge/category%20accuracy-100%25-success.svg)](#performance-highlights)
[![Throughput](https://img.shields.io/badge/throughput-9%2C200%2B%20emails%2Fsec-blueviolet.svg)](#performance-highlights)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

---

## What is Relay Rover?

Most email applications treat emails as generic unstructured text, forcing users to open messages to find verification codes, tracking numbers, flight gates, or receipts.

**Rover** is the deterministic email intelligence engine behind [Relay Mail](https://blogs.relaymail.online/blog/rover). Built in Rust and compiled for both **WebAssembly (Web/Node)** and **Native JSI + UniFFI (React Native Hermes on Android & iOS)**, Rover understands emails locally on-device without cloud LLMs, network latency, or recurring API costs.

```text
Incoming Email ──► RegexSet DFA (1 Pass) ──► Purpose Category + Dual-Layer Classification
                                          ──► Deep Typed Entity Extraction (OTP, Order, Txn)
                                          ──► Semantic Preview Line (< 100 chars)
                                          ──► Rover Intelligence Index (Timeline Key)
```

---

## 🏛 Cross-Platform Architecture

Rover maintains **a single Rust core as the absolute source of truth** across all platforms:

```
                       ┌─────────────────────────────────────────────────────────────┐
                       │                     @rhq/rover API                          │
                       │    (index.js for Web/Node, index.native.js for RN/Hermes)   │
                       └──────────────────────────────┬──────────────────────────────┘
                                                      │
                       ┌──────────────────────────────┴──────────────────────────────┐
                       │               Platform Resolution / Adapter                 │
                       │             (Zero Node built-ins on RN path)                │
                       └──────────────┬───────────────────────────────┬──────────────┘
                                      │                               │
                 Web & Node.js        │                  React Native / Expo
            ┌─────────────────────────┴─────────┐        ┌────────────┴──────────────────────┐
            │       Linear WebAssembly          │        │      JSI Native Bridge            │
            │   (WASM Instance via linear mem)  │        │   (global.__RELAY_ROVER_NATIVE__) │
            └─────────────────┬─────────────────┘        │   (Expo Module: RelayRover)       │
                              │                          └────────────┬──────────────────────┘
                              │                                       │
                              │                        ┌──────────────┴──────────────┐
                              │                        │                             │
                              │                 Android (JNI/JSI)               iOS (JSI)
                              │                 - RoverModule.kt (Expo)         - RoverModule.swift (Expo)
                              │                 - RoverJsi.cpp (Hermes JSI)     - RoverJsi.cpp (Hermes JSI)
                              │                 - UniFFI Kotlin Bindings        - UniFFI Swift Bindings
                              │                        │                             │
                              │                        └──────────────┬──────────────┘
                              │                                       │
                              ▼                                       ▼
                       ┌─────────────────────────────────────────────────────────────┐
                       │                   Shared Rust Core (Rover)                  │
                       │     - Deterministic Email Classification (RegexSet)         │
                       │     - Entity Extraction (Date, Amount, OTP, Identifiers)    │
                       │     - Contextual Preview Generation                         │
                       │     - Multithreaded Parallel Batch Processing (Rayon)       │
                       │     - HTML to Text Sanitization                             │
                       │     - Memory Management & C / UniFFI FFI Symbols            │
                       └─────────────────────────────────────────────────────────────┘
```

1. **Shared Rust Core**: Zero duplicated business logic in Kotlin, Swift, or TypeScript. All classification and extraction run from the Rust engine.
2. **Web & Node.js**: Direct in-memory linear WebAssembly execution (`eng/relay_rover.wasm`).
3. **Android (Expo & React Native)**:
   - Compiled for `arm64-v8a`, `x86_64`, `armeabi-v7a` using Android NDK.
   - UniFFI Kotlin bindings (`uniffi.relay_rover`).
   - Direct C++ JSI host object (`RoverHostObject`) binding onto Hermes runtime (`globalThis.__RELAY_ROVER_NATIVE__`).
   - Kotlin Expo Module (`com.relayhq.rover.RoverModule`).
4. **iOS (Expo & React Native)**:
   - Compiled for `aarch64-apple-ios` and `aarch64-apple-ios-sim`.
   - UniFFI Swift bindings (`relay_rover.swift`).
   - Direct C++ JSI host object (`RoverHostObject`) binding onto Hermes runtime.
   - Swift Expo Module (`RoverModule`).
5. **JavaScript / TypeScript (`@rhq/rover`)**:
   - `index.js`: Web and Node.js execution.
   - `index.native.js`: React Native Hermes execution with **zero imports of `fs`, `path`, or `url`**.
   - Metro bundler resolves `index.native.js` automatically without requiring custom aliases or polyfills.

---

## 📱 Supported Platforms

| Platform | Runtime | Execution Layer | Off-Thread Async |
|---|---|---|:---:|
| **React Native (Android)** | Hermes / JSC | C++ JSI + UniFFI Kotlin | ✅ |
| **React Native (iOS)** | Hermes / JSC | C++ JSI + UniFFI Swift | ✅ |
| **Expo (Managed / Bare)** | Hermes / JSC | Expo Modules Core | ✅ |
| **Web / Browsers** | V8 / SpiderMonkey / JSC | Linear WebAssembly | ✅ |
| **Node.js** | Node.js 18+ | WebAssembly | ✅ |
| **Server / Cloudflare / Deno** | V8 / Workers | WebAssembly | ✅ |

---

## 📦 Installation

### React Native & Expo

```bash
# Using Expo
npx expo install @rhq/rover

# Using npm
npm install @rhq/rover

# Using yarn
yarn add @rhq/rover

# Using pnpm
pnpm add @rhq/rover
```

For vanilla React Native on iOS, run CocoaPods:
```bash
npx pod-install
```

### Node.js & Web

```bash
npm install @rhq/rover
```

---

## 🚀 API Usage (100% Preserved & Identical Across Platforms)

Existing consumers do not need to modify their code. The exact same API runs on Web, Node, Android, and iOS:

```typescript
import { RelayRover, RoverIndex, htmlToText, cleanEmailBody, normalizeEmail, extractSummary } from '@rhq/rover';

// 1. Classify an incoming email
const category = RelayRover.classify(
  "Your payment of ₹10.0 is successful",
  "Hey Soham, You paid ₹10.0 to DigiGold with txn id FMPIB6973726141."
);
console.log(category); // "Payment"

// 2. Generate contextual inbox preview (< 100 characters)
const preview = RelayRover.generatePreview(
  "Your payment of ₹10.0 is successful",
  "Hey Soham, You paid ₹10.0 to DigiGold with txn id FMPIB6973726141."
);
console.log(preview); // "₹10.0 paid to DigiGold · Successful"

// 3. Process a full email object (auto-sanitizes HTML and attaches cleanBody)
const processed = RelayRover.process({
  id: "msg_123",
  from: "Google Play <googleplay-noreply@google.com>",
  subject: "Your Google Play Order Receipt",
  htmlBody: "<h1>Order number: GPA.3399-0949-2150-45472</h1><p>Special Airdrop 412 ₹29.00</p>"
});

console.log(processed.primary_category); // "shopping"
console.log(processed.preview);          // "₹29.00 · Special Airdrop 412 (Google Play)"
console.log(processed.extraction.data);  // { store: 'Google Play', order_id: '...', amount: '29.00', ... }
console.log(processed.timeline_key);     // "order:GPA.3399-0949-2150-45472"

// 4. Batch process multiple emails in a single native call
const batchResults = RelayRover.processBatch(incomingEmails);

// 5. Connect related emails into activity timelines
const { timelines, unlinked } = RoverIndex.buildTimelines(batchResults);

// 6. Asynchronous execution off the UI thread (React Native / Hermes)
const asyncResult = await RelayRover.processAsync(singleEmail);
const asyncBatch = await RelayRover.processBatchAsync(largeEmailArray);
```

---

## 🛠 How to Build, Maintain, and Update

Rover's core workflow is designed for high maintainability:

### 1. Adding or Updating Regex Rules & Categories

- Category filters live in `src/filters/` (`payment.rs`, `otp.rs`, `shopping.rs`, etc.).
- Entity extractors live in `src/extractors/` (`amount.rs`, `date.rs`, `identity.rs`, etc.).
- To expose a new FFI method, add its signature to `src/rover.udl` and implement it in `src/ffi.rs`.

### 2. Regenerating UniFFI Bindings

When modifying `src/rover.udl`:
```bash
./scripts/generate-uniffi.sh
```
This automatically compiles `uniffi-bindgen` and updates:
- Android: `packages/rover/android/src/main/java/uniffi/relay_rover/relay_rover.kt`
- iOS: `packages/rover/ios/relay_rover.swift` and `packages/rover/ios/relay_roverFFI.h`

### 3. Compiling Android Native Binaries (`.so`)

Requires Android NDK (`ANDROID_HOME` or installed via Android Studio):
```bash
./scripts/build-android.sh
```
Compiles release `.so` files into `packages/rover/android/src/main/jniLibs/`:
- `arm64-v8a/librelay_rover.so` (64-bit ARM devices)
- `x86_64/librelay_rover.so` (Android Studio emulator)
- `armeabi-v7a/librelay_rover.so` (32-bit ARM devices)

### 4. Compiling iOS Static Libraries (`.a` / XCFramework)

Requires macOS with Xcode installed:
```bash
./scripts/build-ios.sh
```
Compiles release static libraries for `aarch64-apple-ios` and `aarch64-apple-ios-sim`.

### 5. Running Tests

```bash
# 1. Rust engine unit & dataset tests (20 tests)
cargo test

# 2. WebAssembly benchmark & report generator
node test_rover.js

# 3. React Native, Hermes parity, and integrity test suite (17 tests)
node tests/test_react_native_parity.js
```

---

## ⚡ Performance Highlights

| Metric | Native Rust / JSI | WebAssembly (Node / Web) | Cloud LLMs |
|---|:---:|:---:|:---:|
| **Median Latency (p50)** | **30 µs** | **202 µs** (in batch) | 1,200,000 µs (1.2 s) |
| **Throughput** | **9,221+ emails/sec** | **4,939 emails/sec** | ~5 emails/sec |
| **Accuracy** | **100.0%** | **100.0%** | Non-deterministic |
| **Cost** | **$0.00** | **$0.00** | ~$0.02 / email |
| **Privacy** | **100% On-Device** | **100% On-Device** | Cloud transmission |

---

## ❓ Troubleshooting

### Metro bundler resolution issues
Ensure Metro cache is cleared after updating:
```bash
npx expo start -c
# or
npx react-native start --reset-cache
```

### Missing JSI engine on bare React Native
If running without Expo, ensure `RoverPackage` is included in your `MainApplication.kt`:
```kotlin
packages.add(RoverPackage())
```

---

## 📄 License

MIT © [Relay](https://blogs.relaymail.online)
