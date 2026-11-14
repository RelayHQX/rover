# Relay Rover Engine

> **High-Performance, Email Intelligence Engine for Relay Mail, React Native, and Edge Services.**  
> *100% Deterministic · Sub-Millisecond Latency · On-Device*  

[![Blog](https://img.shields.io/badge/Blog-Meet%20Rover%3A%20The%20Intelligence%20Behind%20Relay%20Mail-8A2BE2.svg)](https://blogs.relaymail.online/blog/rover)
[![Category Accuracy](https://img.shields.io/badge/category%20accuracy-100%25-success.svg)](#performance-highlights)
[![Extraction Accuracy](https://img.shields.io/badge/extraction%20accuracy-100%25-success.svg)](#performance-highlights)
[![Native Throughput](https://img.shields.io/badge/native%20throughput-9%2C221%20emails%2Fsec-blueviolet.svg)](#performance-highlights)
[![WASM Throughput](https://img.shields.io/badge/WASM%20throughput-4%2C939%20emails%2Fsec-success.svg)](#performance-highlights)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

---

## What is Relay Rover?

Most email applications treat emails as generic text, forcing users to open messages to find verification codes, tracking numbers, flight gates, or receipts.

**Rover** is the deterministic email intelligence engine behind [Relay Mail](https://blogs.relaymail.online/blog/rover). Built in Rust and compiled to WebAssembly, Rover understands emails locally on-device without cloud LLMs, or network latency.

```text
Incoming Email ──► RegexSet DFA (1 Pass) ──► Purpose Category + Dual-Layer Classification
                                          ──► Deep Typed Entity Extraction (OTP, Order, Txn)
                                          ──► Semantic Preview Line (< 100 chars)
                                          ──► Rover Intelligence Index (Timeline Key)
```

---

## Key Capabilities

- **Dual-Layer Classification**: Traditional inbox folder routing (`Primary`, `Updates`, `Social`, `Promotions`, `Forums`) combined with actionable purpose categories (`Authentication`, `Transactions`, `Deliveries`, `Jobs`, `SecurityAlerts`, `Travel`, `Subscriptions`, `Newsletters`).
- **Urgency & Alert Engine**: Automatically tags priority (`Urgent`, `High`, `Normal`, `Low`) and raises `is_alert: true` flags for time-critical actions (active OTPs, CI/CD build failures).
- **Deep Deterministic Extraction**: Extracts structured fields (amounts, currencies, order IDs, vacancy counts, tracking numbers, auth codes, expiration windows) with zero hallucination.
- **Contextual Previews**: Generates informative summary lines tailored to the category (e.g. `₹29.00 · Special Airdrop 412 (Google Play)` or `Job Alert · 8 React Native Developer jobs at Webomindapps`).
- **The Rover Intelligence Index**: Extracts deterministic `timeline_key` values to group fragmented messages into unified activity timelines (Order Placed $\rightarrow$ Receipt $\rightarrow$ Dispatched $\rightarrow$ Delivered).

---

## Supported Categories

| Purpose Category | Folder Layer | Extracted Entities | Contextual Preview Example |
|---|---|---|---|
| **OTP / Auth** | `Primary` | Code, service name, expiration, auth purpose, URL | `Nintendo verification code: 0313 (Expires in 24 hours)` |
| **Payment** | `Updates` | Amount, currency, payer, recipient, txn ID, date, status | `₹10.0 paid to DigiGold · Successful` |
| **Shopping** | `Updates` | Store, order ID, product, amount, tax, date, tracking | `₹29.00 · Special Airdrop 412 (Google Play)` |
| **Jobs** | `Updates` | Company, position, location, job count, URL, deadline | `Job Alert · 8 React Native Developer jobs at Webomindapps` |
| **Security Alerts** | `Updates` | Alert type, target resource, severity, action required | `⚠️ Build Failure · Deploy Server (relay-server)` |
| **Invoices** | `Updates` | Invoice number, amount, currency, due date, issuer | `Invoice received · $420.50 due 2026-11-20` |
| **Subscriptions**| `Updates` | Provider, amount, frequency, renewal date | `SoundWave Music · monthly renewal on 2026-11-28` |
| **Travel** | `Updates` | Booking ID / PNR, destination, dates, status | `Booking to Tokyo (HND) · Confirmed for 2026-12-05` |
| **Newsletters** | `Promotions`| Publisher, main topic, headline | `Rust Weekly Digest · Rust 2024 Edition stabilized` |
| **Social** | `Social` | Platform, activity type, actor, post URL | `LinkedIn · Reaction from Udoh Jeremiah` |
| **General** | `Primary` | Clean single-sentence excerpt fallback | Word-boundary excerpt (< 95 chars) |

---

## Quick Start

### 1. Rust Library

Add to your `Cargo.toml`:
```toml
[dependencies]
relay-rover = { path = "path/to/relay-eng" }
```

```rust
use relay_rover::{Rover, ExtractedData};

fn main() -> anyhow::Result<()> {
    let rover = Rover::new()?;
    let processed = rover.process_raw(
        "Your payment of ₹10.0 is successful",
        "Hey Soham, You paid ₹10.0 to DigiGold with txn id FMPIB6973726141."
    );

    println!("Category: {}", processed.primary_category); // "Payment"
    println!("Preview:  {}", processed.preview);          // "₹10.0 paid to DigiGold · Successful"

    if let ExtractedData::Payment(p) = processed.extraction {
        println!("Amount: {:?} {:?}", p.amount, p.currency);
        println!("Txn ID: {:?}", p.transaction_id);
    }
    Ok(())
}
```

### 2. JavaScript & React Native (`@rhq/rover`)

```javascript
import { RelayRover } from '@rhq/rover';

// Process a single email object (HTML sanitized automatically)
const result = RelayRover.process({
  subject: "Your Google Play Order Receipt",
  htmlBody: "<h1>Order number: GPA.3399-0949-2150-45472</h1><p>Special Airdrop 412 ₹29.00</p>"
});

console.log(result.primary_category); // "shopping"
console.log(result.preview);          // "₹29.00 · Special Airdrop 412 (Google Play)"

// Batch process multiple emails in 1 WASM call (~4,900 emails/sec)
const batch = RelayRover.processBatch(incomingEmails);
```

---

## Performance Highlights

| Metric | Native Rust | WebAssembly (JS / React Native) | Cloud LLMs |
|---|:---:|:---:|:---:|
| **Median Latency (p50)** | **30 µs** | **202 µs** (in batch) | 1,200,000 µs (1.2 s) |
| **Throughput** | **9,221 emails/sec** | **4,939 emails/sec** | ~5 emails/sec |
| **Accuracy** | **100.0%** | **100.0%** | Non-deterministic |
| **Cost** | **$0.00** | **$0.00** | ~$0.02 / email |
| **Privacy** | **100% Local** | **100% Local** | Transmitted to 3rd party |

---

## Documentation & Deep Dives

- **[packages/rover/README.md](packages/rover/README.md)**: Full guide on integrating Rover into a real-world email client (action cards, one-tap OTP copy, delivery tracking, timeline groupings).
- **[BENCHMARK.md](BENCHMARK.md)**: High-resolution latency percentiles, throughput measurements, and pipeline stage breakdowns.
- **[CONTRIBUTING.md](CONTRIBUTING.md)**: Contributor guide, codebase architecture map, testing workflows, and instructions for adding new categories or regex rules.
- **[TEST_RESULTS.md](TEST_RESULTS.md)**: Real-world email test reports with side-by-side extraction outputs and JSON dumps.

---

## License

MIT © [Relay](https://blogs.relaymail.online)
