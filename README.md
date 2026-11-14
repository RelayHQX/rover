# Relay Rover Engine

> **High-performance, deterministic email classification, information extraction, and preview generation engine for Relay Mail, built with the official Rust `regex` crate.**
>

[![Category Accuracy](https://img.shields.io/badge/category%20accuracy-100%25-success.svg)](#accuracy)
[![Extraction Accuracy](https://img.shields.io/badge/extraction%20accuracy-100%25-success.svg)](#accuracy)
[![Throughput](https://img.shields.io/badge/throughput-36%2C000%2B%20emails%2Fsec-blueviolet.svg)](#throughput)

---

## Table of Contents

- [Overview](#overview)
- [Key Features](#key-features)
- [How RegexSet Works](#how-regexset-works)
- [Architecture & Module Organization](#architecture--module-organization)
- [Using as a Rust Library](#using-as-a-rust-library)
  - [Basic Usage with Rover](#basic-usage-with-rover)
  - [Customizing & Adding New Rules via RoverBuilder](#customizing--adding-new-rules-via-roverbuilder)
  - [Processing Arbitrary Emails](#processing-arbitrary-emails)
- [Building for WebAssembly (WASM)](#building-for-webassembly-wasm)
- [Connecting with React Native](#connecting-with-react-native)
  - [Option A: JSI / Nitro Native Bridge (Recommended for Ultra-Low Latency)](#option-a-jsi--nitro-native-bridge-recommended-for-ultra-low-latency)
  - [Option B: React Native WASM Runtime](#option-b-react-native-wasm-runtime)
- [Running Benchmarks & Tests](#running-benchmarks--tests)
- [Supported Categories](#supported-categories)
- [Known Limitations](#known-limitations)
- [Contributing](#contributing)

---

## Overview

**Relay Rover** (or simply **Rover**) is a lightweight, zero-LLM email processing engine designed for mail clients and edge services. Instead of unpredictable, costly, and latency-heavy AI calls, it utilizes compiled finite-state automata (`RegexSet` and pre-compiled typed `Regex` instances) to achieve:

- **100.0% Category Detection Accuracy** on realistic synthetic email workloads.
- **100.0% Field Extraction Accuracy** across monetary amounts, dates, identities, OTPs, and transaction IDs.
- **Sub-30 microsecond latency** per email for the full pipeline (Detection + Extraction + Preview generation).
- **Over 36,000 emails/second** single-core throughput.
- **Zero dynamic regex compilation**: all patterns are compiled strictly once at initialization.

---

## Key Features

- **Multi-Pattern Category Detection with `RegexSet`**: Evaluates all detection rules simultaneously in a single DFA pass.
- **Lazy Extraction Dispatch**: Extraction regexes are executed **only** for categories triggered by the detector. Unmatched categories are never scanned.
- **Typed Information Extraction**: Structured, typed structs with `rust_decimal::Decimal`, normalized currency codes, dates, transaction IDs, and clean evidence.
- **Deterministic Contextual Previews**: Concise snippets (< 100 characters) designed for notification badges and inbox preview lines.
- **Extensible & Pluggable**: `RoverBuilder` (`EngineBuilder`) allows downstream Rust developers to add custom filters, replace rules, or add custom detection patterns without modifying the core crate.
- **WebAssembly (WASM) Ready**: First-class support for `wasm32-unknown-unknown` and `wasm-bindgen`.
- **Cross-Platform React Native Support**: Run native on iOS/Android via JSI/Rust-bridge or via WASM.

---

## How RegexSet Works

The standard approach of iterating through $N$ individual regular expressions has time complexity $O(N \times L)$ where $L$ is email length:

$$\text{Naive approach: } \text{Regex}_1 \to \text{Regex}_2 \to \dots \to \text{Regex}_N$$

In contrast, Relay Rover's `regex::RegexSet` compiles all detection patterns into a unified Aho-Corasick / PikeVM / Lazy DFA automaton:

$$\text{Rover Engine: } \text{Input Text} \longrightarrow \mathbf{\text{RegexSet DFA (1 Pass)}} \longrightarrow \text{Matched Category IDs}$$

1. **Single-Pass Matching**: The email text is streamed through the automaton once in $O(L)$ time, regardless of pattern count.
2. **Bitset Result**: `RegexSet::matches()` returns a compact set of matching pattern indices.
3. **Targeted Extraction**: The engine maps matching indices to a `Category` and runs *only* the specific extraction regexes associated with that category. Unmatched categories are never evaluated.

---

## Architecture & Module Organization

```text
relay-intelligence/
├── Cargo.toml                  # Crate dependencies, features, cdylib + rlib
├── data/
│   └── emails.json             # 100 realistic synthetic benchmark emails
├── src/
│   ├── lib.rs                  # Public library exports & API surface (Rover, RoverEngine)
│   ├── main.rs                 # CLI binary benchmark driver
│   ├── models.rs               # Category enum, typed extractions, EmailRecord, ProcessedEmail
│   ├── engine.rs               # RoverEngine (EmailEngine) & RoverBuilder
│   ├── registry.rs             # RegexRegistry & RegistryBuilder managing RegexSet DFA
│   ├── preview.rs              # Deterministic contextual preview generation (< 100 chars)
│   ├── benchmark.rs            # Latency percentiles, throughput, and accuracy suite
│   ├── wasm.rs                 # WebAssembly FFI bindings and JSON bridge
│   ├── filters/                # Category filter implementations
│   │   ├── mod.rs              # EmailFilter trait
│   │   ├── payment.rs          # Payments, debits, UPI, receipts
│   │   ├── otp.rs              # 2FA codes, auth PINs, expiry
│   │   ├── job.rs              # Interview invitations, job offers, vacancies
│   │   ├── shopping.rs         # Order tracking, dispatch, deliveries
│   │   ├── invoice.rs          # Utility bills, cloud invoices, due dates
│   │   ├── subscription.rs     # Memberships, renewals, billing frequency
│   │   ├── travel.rs           # Flights, hotels, PNRs, booking IDs
│   │   ├── newsletter.rs       # Weekly digests, publishers, topics
│   │   ├── social.rs           # Connection requests, mentions, PR reviews
│   │   └── general.rs          # Clean excerpt fallback
│   └── extractors/             # Reusable typed field parsers
│       ├── mod.rs
│       ├── amount.rs           # Currency & Decimal parser ($49.99, INR 2500, etc.)
│       ├── date.rs             # ISO & verbal dates (YYYY-MM-DD)
│       ├── identity.rs         # Merchants, issuers, stores, platforms
│       ├── transaction.rs      # TXN, Order IDs, PNRs, Tracking numbers
│       └── otp.rs              # Context-anchored authentication codes
├── tests/
│   ├── category_tests.rs       # Unit tests for all categories & RoverBuilder API
│   └── dataset_accuracy.rs     # 100-email ground-truth evaluation
├── packages/
│   └── react-native-relay-rover/ # JavaScript & React Native bindings package
├── scratch/
│   └── generate_emails.py      # Python generator for 100 benchmark synthetic emails
├── test_rover.js               # Node.js / JavaScript end-to-end test script
├── CONTRIBUTING.md             # Developer guide and contribution workflows
└── BENCHMARK.md                # Comprehensive benchmark results & analysis
```

---

## Using as a Rust Library

Add `relay-intelligence` to your `Cargo.toml`:

```toml
[dependencies]
relay-intelligence = { path = "path/to/relay-eng" }
```

### Basic Usage with Rover

```rust
use relay_intelligence::{Rover, RoverEngine, Category, ExtractedData};

fn main() -> anyhow::Result<()> {
    // 1. Initialize Rover engine (compiles all patterns once)
    let rover = Rover::new()?;

    // 2. Process an arbitrary email
    let subject = "Transaction Alert: INR 2,500.00 debited from A/C XX4912";
    let body = "Dear Customer, your account has been debited by INR 2500.00 on 2026-11-05 for payment to QuickMart Retail. Txn Ref: TXN-89210492. Status: Successful.";

    let processed = rover.process_raw(subject, body);

    println!("Category: {}", processed.primary_category); // "Payment"
    println!("Preview:  {}", processed.preview);          // "₹2500.00 paid to QuickMart Retail · Successful"

    // Access category-specific typed extractions
    if let ExtractedData::Payment(p) = processed.extraction {
        println!("Amount:    {:?}", p.amount);             // Some(2500.00)
        println!("Currency:  {:?}", p.currency);           // Some("INR")
        println!("Recipient: {:?}", p.recipient);          // Some("QuickMart Retail")
        println!("Txn ID:    {:?}", p.transaction_id);     // Some("TXN-89210492")
    }

    Ok(())
}
```

### Customizing & Adding New Rules via RoverBuilder

Downstream users can customize the engine using `RoverBuilder`:

```rust
use relay_intelligence::{RoverBuilder, Category};

let custom_rover = RoverBuilder::new()
    // Add custom pattern mapped to an existing category
    .with_detection_pattern(r"(?i)\bProject Titan Release\b", Category::Job)
    // Or override an entire filter with a custom EmailFilter implementation
    // .with_filter(MyCustomFilter::new())
    .build()?;
```

### Processing Arbitrary Emails

You can process emails from various representations:

```rust
// From raw subject and body
let res1 = rover.process_raw("Order #SQ-102 placed", "Total $45.00 at BookHaven");

// From a single multiline raw string (first line or "Subject: " line parsed as subject)
let res2 = rover.process_text("Subject: Flight to Paris confirmed\nPNR: SK-78921 Destination: Paris");

// From a full EmailRecord struct
let res3 = rover.process(&email_record);
```

### Choosing Single-Threaded vs Multi-Threaded Execution

Rover allows callers to choose between **single-threaded** and **multi-threaded** execution. By default, **single-threaded execution** is used to avoid context-switching overhead on lightweight workloads.

```rust
use relay_rover::{Rover, RoverBuilder, ExecutionMode};

// 1. Single-Threaded (Default)
let rover = Rover::new()?; 
assert_eq!(rover.num_threads(), 1);
assert!(!rover.is_multi_threaded());

// 2. Multi-Threaded with custom thread count (e.g., 4 worker threads)
let multi_rover = RoverBuilder::new()
    .with_multi_threaded(Some(4))
    .build()?;
assert_eq!(multi_rover.num_threads(), 4);

// 3. Multi-Threaded with automatic thread count (defaults to system CPU cores)
let auto_core_rover = RoverBuilder::new()
    .with_multi_threaded(None)
    .build()?;

// 4. Dynamic selection via with_threading(multithread: bool, threads: Option<usize>)
let dynamic_rover = RoverBuilder::new()
    .with_threading(true, Some(8))
    .build()?;

// Process batch: automatically uses the configured single or multi-threaded worker pool
let results = multi_rover.process_batch(&email_records);
```

#### CLI Execution Options:
```bash
# Run benchmark in single-threaded mode (default: 1 thread)
cargo run --release

# Run benchmark in multi-threaded mode with 4 worker threads
cargo run --release -- --threads 4

# Run benchmark in multi-threaded mode with all CPU cores
cargo run --release -- --multi-thread
```

---

## Building for WebAssembly (WASM)

Rover includes built-in WebAssembly bindings powered by `wasm-bindgen`.

### 1. Install wasm-pack
```bash
cargo install wasm-pack
```

### 2. Compile to WASM

#### For Web Browsers / ES Modules:
```bash
wasm-pack build --target web -- --features wasm
```

#### For Node.js / Bundlers:
```bash
wasm-pack build --target bundler -- --features wasm
```

The generated package in `./pkg` includes:
- `relay_intelligence_bg.wasm` (optimized WebAssembly binary)
- `relay_intelligence.js` (JavaScript wrapper)
- `relay_intelligence.d.ts` (TypeScript typings)

### 3. Using in JavaScript / TypeScript

```typescript
import init, { process_raw_email, classify_email, generate_email_preview } from './pkg/relay_intelligence.js';

async function run() {
  await init();

  // Fast category classification
  const category = classify_email("Your verification code", "Your code is 482913. Expires in 10 mins.");
  console.log("Category:", category); // "OTP"

  // Contextual preview
  const preview = generate_email_preview("Your verification code", "Your code is 482913 for Acme Portal.");
  console.log("Preview:", preview); // "Acme Portal verification code · 482913"

  // Full structured extraction
  const jsonResult = process_raw_email(
    "email_01",
    "alerts@bank.test",
    "Bank Alerts",
    "Your payment was successful",
    "Paid $49.99 to StreamCast. Txn ID: PF-994821. Status: Completed."
  );

  const parsed = JSON.parse(jsonResult);
  console.log(parsed.extraction);
}
```

---

## Connecting with React Native

Rover can be embedded into any React Native app (Bare React Native or Expo) using either **Native JSI / Nitro Modules** (sub-millisecond execution) or **WebAssembly (WASM)**.

### The `react-native-relay-rover` Package

The ready-to-use JavaScript & React Native bindings package is located directly in this repository at:
[`packages/react-native-relay-rover/`](packages/react-native-relay-rover/)

It contains:
- `package.json` — npm package definition (ready for `npm publish`)
- `eng/relay_rover.wasm` — Pre-compiled, zero-dependency in-memory WebAssembly engine (1.7 MB, zero imports)
- `index.js` — Universal WASM runtime bridge (zero binary child processes, sub-millisecond execution)
- `index.d.ts` — Full TypeScript typings for all categories, extractions, and preview models
- `README.md` — Integration and publishing guide

#### Test the JavaScript Package Right Now
You can run the included JavaScript test suite in Node.js immediately:

```bash
node test_rover.js
```
This runs 5 individual email tests and a **100-email batch test in a single WASM call** in **37 milliseconds** (over 2,600 emails/second in pure JavaScript!).

### Installation in a React Native Project

From your React Native project root:
```bash
npm install /path/to/relay-eng/packages/react-native-relay-rover
# or local workspace:
# npm install /path/to/relay-eng/packages/react-native-relay-rover
```

### Usage in React Native (TypeScript / JavaScript)

```typescript
import { RelayRover } from 'react-native-relay-rover';

// 1. Fast category detection (sub-millisecond)
const category = RelayRover.classify(
  "Transaction Alert: INR 2,500.00 debited from A/C XX4912",
  "Dear Customer, your account has been debited by INR 2500.00 on 2026-11-05 for payment to QuickMart Retail. Txn Ref: TXN-89210492. Status: Successful."
);
console.log("Category:", category); // "Payment"

// 2. Notification Preview Line (< 100 chars)
const preview = RelayRover.generatePreview(
  "Your verification code is 482913",
  "Your one-time verification code for Acme Portal is 482913. Expires in 10 minutes."
);
console.log("Preview:", preview); // "Acme Portal verification code · 482913"

// 3. Full Typed Information Extraction
const processed = RelayRover.processEmail(
  "Flight Booking Confirmation: PNR SK-78921 to Tokyo",
  "Booking ID: SK-78921. Destination: Tokyo (HND). Departure date: 2026-12-05. Booking status: Confirmed."
);

console.log(processed.preview); // "Booking to Tokyo (HND) · Confirmed for 2026-12-05"
console.log(processed.extraction.data);

// 4. Batch Processing Multiple Emails in a Single Call
const emails = [
  { subject: "Payment Alert", body: "Paid $49.99 to StreamCast. Txn ID: PF-994821" },
  { subject: "Security PIN", body: "Your verification code is 482913" },
  { subject: "Flight Confirmed", body: "Booking ID: SK-78921 to Tokyo" }
];

const batchResults = RelayRover.processBatch(emails);
console.log(`Processed ${batchResults.length} emails in a single in-memory WASM call!`);
// {
//   booking_id: "SK-78921",
//   destination: "Tokyo (HND)",
//   departure_date: "2026-12-05",
//   booking_status: "Confirmed"
// }
```

---

### Option A: JSI / Nitro Native Bridge (Cross-Compilation for iOS & Android)

This approach compiles the Rust engine into native static libraries (`.a` / `.so`) and exposes synchronous JSI methods to JavaScript.

#### 1. Cross-Compile for Android & iOS
```bash
# Android NDK targets
cargo build --target aarch64-linux-android --release
cargo build --target x86_64-linux-android --release

# iOS targets
cargo build --target aarch64-apple-ios --release
cargo build --target aarch64-apple-ios-sim --release
```

---

### Option B: React Native WASM Runtime

If you want a pure JavaScript/TypeScript setup without compiling native C libraries:

1. Install `react-native-wasm`:
   ```bash
   npm install react-native-wasm
   ```
2. Bundle `relay_intelligence_bg.wasm` into your mobile app assets (`android/app/src/main/assets` and iOS bundle resources).
3. Instantiate and run:
   ```typescript
   import { instantiate } from 'react-native-wasm';
   import wasmBinary from './relay_intelligence_bg.wasm';

   const { instance } = await instantiate(wasmBinary);
   // Invoke exported functions directly
   ```

---

## Dataset Generation & Benchmark Reproduction

### 1. Generate / Regenerate Synthetic Emails
To regenerate or inspect the 100 synthetic benchmark emails dataset:
```bash
python3 scratch/generate_emails.py
```
This script populates `data/emails.json` with 100 realistic synthetic email records distributed across 10 categories, accompanied by ground-truth labels for automated accuracy verification.

### 2. Run All Unit and Accuracy Tests
```bash
cargo test
cargo test --package relay-intelligence --test category_tests -- --nocapture
```

### 3. Run Strict Code Linting (Clippy)
```bash
cargo clippy --all-targets --all-features
```

### 4. Run Formatting Verification
```bash
cargo fmt --check
```

### 5. Run Official Release-Mode Benchmark
```bash
cargo run --release
```
```

### Run Strict Code Linting (Clippy)
```bash
cargo clippy --all-targets --all-features
```

### Run Formatting Verification
```bash
cargo fmt --check
```

### Run Release-Mode Benchmark
```bash
cargo run --release
```

---

## Supported Categories

| Category | Description | Extracted Entities |
|---|---|---|
| **Payment** | Debits, credits, UPI, wire transfers, receipts | Amount, Currency, Recipient, Payer, Txn ID, Date, Status |
| **OTP / Auth** | Two-factor verification, PINs, passcodes | Code, Service Name, Expiration duration |
| **Jobs** | Interview invites, job offers, application statuses | Company, Position, Location, Salary, Deadline, Interview Date |
| **Shopping** | E-commerce orders, shipments, delivery status | Store, Order ID, Product, Amount, Status, Tracking Number |
| **Invoices** | Cloud bills, utilities, statements | Invoice Number, Amount, Currency, Due Date, Issuer |
| **Subscriptions**| SaaS, recurring memberships, renewals | Provider, Subscription Amount, Billing Frequency, Renewal Date, Cancellation Terms |
| **Travel** | Flights, hotel reservations, train bookings | Booking ID / PNR, Destination, Departure Date, Return Date, Status |
| **Newsletters** | Digests, tech bulletins, editorial roundups | Publisher, Main Topic, Headline |
| **Social** | Invitations, mentions, pull requests, comments | Platform, Activity Type, Actor / Entity |
| **General** | Personal conversations, meetings, notes | Clean excerpt, Sender display |

---

## Contributing

Please review [CONTRIBUTING.md](CONTRIBUTING.md) for code structure, architectural conventions, and guidelines for adding new categories or extractors to Rover.

For detailed performance numbers and profiling methodology, see [BENCHMARK.md](BENCHMARK.md).
