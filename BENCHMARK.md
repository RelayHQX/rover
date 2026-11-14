# Relay Rover Engine — Performance Benchmark & Production Validation

> **Engine:** Relay Rover (Zero-LLM Deterministic Rust Regex WebAssembly Engine)  
> **Package:** `@rhq/rover`
> **Evaluation Reference:** [*Meet Rover: The Intelligence Behind Relay Mail*](https://blogs.relaymail.online/blog/rover)  
> **Tested On:** Linux x86_64, Node.js v22.13.1, Rust 1.98.0-nightly

---

## 1. Executive Summary

Relay Rover is the client-side email intelligence engine powering **Relay Mail**. Unlike LLM-based solutions that suffer from unpredictable hallucinations, 2,000–5,000 ms cloud latency, high token costs, and privacy vulnerabilities, Rover operates deterministically using compiled Rust regular expressions and state automata.

It runs:
- **Natively in Rust** at **9,221 emails/sec** (median latency: **30 µs/email**).
- **In-memory via WebAssembly (WASM)** in React Native and Node.js at **4,939 emails/sec** (average latency: **202.5 µs/email**).
- **100% Zero-LLM:** Zero external network calls, zero API keys, and complete data privacy.

---

## 2. Native Rust Benchmark Results (100 Synthetic Emails)

Compiled with `--release` profile (`opt-level = 3`, LTO enabled).

### Pipeline Latency & Throughput

| Metric | Measured Value | Unit | Production SLA |
|---|---|---|---|
| **RegexSet Compilation** | `9.71` | ms | < 25 ms |
| **Extraction Regex Compilation** | `97.71` | ms | < 150 ms |
| **Total Engine Initialization** | `107.43` | ms | < 200 ms |
| **Batch Processing Time (100 emails)** | `10.84` | ms | < 50 ms |
| **Median Latency (p50)** | **`30`** | **µs** | **< 100 µs** |
| **Average Latency** | **`108`** | **µs** | **< 250 µs** |
| **95th Percentile (p95)** | **`442`** | **µs** | **< 600 µs** |
| **99th Percentile (p99)** | **`504`** | **µs** | **< 800 µs** |
| **Min Latency** | `4` | µs | - |
| **Max Latency** | `504` | µs | - |
| **Throughput (Single-threaded)** | **`9,221`** | **emails/sec** | **> 5,000 emails/s** |
| **Throughput (Data rate)** | `2.36` | MB/s | - |
| **Steady-State Throughput (10k emails)** | **`8,967`** | **emails/sec** | **> 5,000 emails/s** |

### Latency by Pipeline Stage

```text
┌─────────────────────────────────┬──────────┬─────────────┐
│ Pipeline Stage                  │ Time/100 │ Per Email   │
├─────────────────────────────────┼──────────┼─────────────┤
│ Stage 1: Category Detection     │ 7.69 ms  │  76 µs      │
│ Stage 2: Detection + Extraction │ 10.45 ms │ 104 µs      │
│ Stage 3: Full Pipeline (Preview)│ 10.84 ms │ 108 µs      │
└─────────────────────────────────┴──────────┴─────────────┘
```

### Classification & Extraction Accuracy (100 Emails Dataset)

| Category | Sample Size | Category Accuracy | Extraction Accuracy | False Positives |
|---|:---:|:---:|:---:|:---:|
| **Payment / Transactions** | 15 | 100.0% (15/15) | 100.0% (15/15) | 0 |
| **Authentication / OTP** | 10 | 100.0% (10/10) | 100.0% (10/10) | 0 |
| **Job Opportunities & Alerts**| 12 | 100.0% (12/12) | 100.0% (12/12) | 0 |
| **Shopping & Deliveries** | 12 | 100.0% (12/12) | 100.0% (12/12) | 0 |
| **Invoices & Billing** | 10 | 100.0% (10/10) | 100.0% (10/10) | 0 |
| **Subscriptions** | 8 | 100.0% (8/8) | 100.0% (8/8) | 0 |
| **Travel & Reservations** | 8 | 100.0% (8/8) | 100.0% (8/8) | 0 |
| **Newsletters & Digests** | 10 | 100.0% (10/10) | 100.0% (10/10) | 0 |
| **Social Notifications** | 5 | 100.0% (5/5) | 100.0% (5/5) | 0 |
| **General Correspondence** | 10 | 100.0% (10/10) | 100.0% (10/10) | 0 |
| **Overall Aggregate** | **100** | **100.0%** | **100.0%** | **0** |

---

## 3. WebAssembly (WASM) Benchmark Results (JavaScript / React Native)

Running through `@rhq/rover` via linear memory C-ABI bindings:

| Metric | Measured Value | Unit | Target / SLA |
|---|---|---|---|
| **Cold Start (Module Instantiation)** | `2.83` | ms | < 20 ms |
| **Batch Processing Throughput** | **`4,939`** | **emails/sec** | **> 1,000 emails/s** |
| **Batch Per-Email Latency** | **`202.5`** | **µs** | **< 500 µs** |
| **Sequential Per-Email Latency (p50)** | `2.51` | ms | < 5 ms |
| **Memory Footprint** | `< 4` | MB | < 16 MB |

---

## 4. Rover vs LLM Architecture Comparison

| Architectural Property | Relay Rover (Deterministic Rust WASM) | Cloud LLM (GPT-4o / Claude 3.5 Sonnet) |
|---|---|---|
| **Latency** | **0.10 ms – 0.20 ms** | 1,200 ms – 3,500 ms |
| **Throughput** | **5,000 – 9,000 emails/sec** | ~2–10 emails/sec (rate-limited) |
| **Cost** | **$0.00** (Zero operational costs) | ~$0.01 – $0.03 per email |
| **Reliability** | **100% Deterministic** (Unit tested) | Non-deterministic, subject to drift |
| **Privacy / Security** | **100% Local / On-Device** | Content sent over network to 3rd party |
| **Offline Support** | **Full offline capability** | Fails completely without internet |

---

## 5. Rover Intelligence Index: Connected Timelines

Rover extracts deterministic `timeline_key` attributes during extraction to group fragmented emails into continuous activity timelines:

- `order:<order_id>` (e.g. `order:GPA.3399-0949-2150-45472`): Connects Order Placed -> Payment Receipt -> Shipment Dispatched -> Out for Delivery.
- `payment:<transaction_id>` (e.g. `payment:FMPIB6973726141`): Connects P2P payments, receipts, and merchant statements.
- `travel:<booking_id>`: Connects Booking Confirmation -> Check-in Reminder -> Boarding Pass -> Flight Delays.
- `invoice:<invoice_number>`: Connects Invoice Issued -> Payment Due Reminder -> Settlement.
