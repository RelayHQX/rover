# Contributing to Relay Rover Engine

Thank you for your interest in contributing to **Relay Rover Engine** (codename **Rover**)! This document provides a complete guide to understanding the codebase, building from source, running tests, reproducing benchmarks, extending filters and extractors, adding custom regex rules, and contributing to the Rover ecosystem.

---

## 1. Project Mission & Design Principles

Relay Rover is designed around strict deterministic principles:
1. **Deterministic & Zero-LLM**: No probabilistic LLM calls, zero network dependency, and 100% reproducible outcomes across devices.
2. **Predictable & Fast**: Single-pass `RegexSet` category detection followed by targeted extraction in microseconds (< 100 µs/email).
3. **Robust & Safe**: Resilient against catastrophic backtracking, zero `.unwrap()` in runtime paths, and defensive fallbacks.
4. **Extensible**: Clean public API (`Rover`, `RoverEngine`, `RoverBuilder`, `EmailFilter`) allowing downstream developers to customize rules.
5. **Separation of Concerns**: Clean boundaries between models, extractors, filters, engine, and benchmarks.

---

## 2. Directory & Architecture Map

```text
relay-eng/
├── Cargo.toml                  # Package manifest, dependencies, features (wasm), crate types
├── data/
│   └── emails.json             # 100 realistic synthetic benchmark emails with ground-truth labels
├── packages/
│   └── rover/                  # Universal JavaScript / React Native package (@rhq/rover)
│       ├── eng/
│       │   └── relay_rover.wasm# Production release WebAssembly engine
│       ├── index.js            # JavaScript WebAssembly runtime bridge
│       ├── index.d.ts          # Complete TypeScript typings
│       ├── package.json        # npm package manifest
│       └── README.md           # Client-side integration guide
├── src/
│   ├── lib.rs                  # Root library crate exposing public API (Rover, RoverEngine, RoverBuilder)
│   ├── main.rs                 # CLI entry point running benchmark suites and terminal report
│   ├── models.rs               # Category enum, classification structs, typed extractions, EmailRecord
│   ├── engine.rs               # High-level RoverEngine (EmailEngine) & RoverBuilder APIs
│   ├── registry.rs             # RegexRegistry & RegistryBuilder managing RegexSet DFA
│   ├── preview.rs              # Deterministic contextual preview generation (< 100 chars)
│   ├── benchmark.rs            # Latency percentiles, throughput, and accuracy evaluation
│   ├── wasm.rs                 # WebAssembly FFI bindings and JSON bridge for JS / React Native
│   ├── filters/                # Category-specific filter modules
│   │   ├── mod.rs              # EmailFilter trait definition and re-exports
│   │   ├── payment.rs          # Payment detection, amounts, recipient, txn ID, status
│   │   ├── otp.rs              # OTP / 2FA detection, PIN codes, expiration times
│   │   ├── job.rs              # Job recruitment, company, role, salary, deadlines
│   │   ├── shopping.rs         # Order tracking, dispatch, store, product, status
│   │   ├── invoice.rs          # Bills, invoice numbers, due dates, issuer
│   │   ├── subscription.rs     # SaaS subscriptions, renewals, frequency, cancellation
│   │   ├── travel.rs           # Airline/hotel bookings, PNRs, destinations, dates
│   │   ├── newsletter.rs       # Weekly digests, publishers, topics, headlines
│   │   ├── social.rs           # Social notifications, mentions, PR reviews, actors
│   │   ├── security.rs         # Security alerts, build failures, account notices
│   │   └── general.rs          # General personal communication excerpt fallback
│   └── extractors/             # Shared reusable entity extraction components
│       ├── mod.rs              # Extractor module declarations and re-exports
│       ├── amount.rs           # Currency symbol and Decimal monetary parser
│       ├── date.rs             # ISO (YYYY-MM-DD) and verbal date parser
│       ├── identity.rs         # Merchant, issuer, provider, store, and person parser
│       ├── transaction.rs      # TXN, Order ID, Booking ID, PNR, Tracking number parser
│       └── otp.rs              # Authentication code and expiration parser
├── tests/
│   ├── category_tests.rs       # Dedicated unit tests for every category and builder API
│   └── dataset_accuracy.rs     # End-to-end accuracy evaluation against ground-truth dataset
├── scratch/
│   └── generate_emails.py      # Python generator for 100 benchmark synthetic emails
├── sample_5_types.json         # Real-world email samples across 5 major categories
├── test_rover.js               # Node.js / JavaScript end-to-end benchmark & test script
├── CONTRIBUTING.md             # This contributor guide
├── BENCHMARK.md                # Comprehensive benchmark results & analysis
├── TEST_RESULTS.md             # Extraction test results & side-by-side comparison tables
└── README.md                   # Project overview & quick start
```

---

## 3. How the Engine Works Internally

### Single-Pass Category Detection (`RegexSet`)
The standard approach of sequentially evaluating $N$ independent regular expressions scales with $O(N \times L)$ time complexity (where $L$ is email body length):

$$\text{Naive approach: } \text{Regex}_1 \to \text{Regex}_2 \to \dots \to \text{Regex}_N$$

In contrast, Rover compiles all category detection patterns into a unified `regex::RegexSet` DFA automaton:

$$\text{Rover: } \text{Input Text} \longrightarrow \mathbf{\text{RegexSet DFA (1 Pass)}} \longrightarrow \text{Matching Category IDs}$$

1. **Single-Pass Stream**: The email text streams through the automaton once in $O(L)$ time, regardless of pattern count.
2. **Bitset Result**: `RegexSet::matches()` produces a compact bitset of matching pattern indices.
3. **Lazy Extraction Dispatch**: Rover maps matched indices to the primary `Category` and runs **only** the extraction regexes associated with that category. Unmatched categories are never scanned.

---

## 4. Development Workflow

### Prerequisites
- Stable Rust (1.80+ or 2024 edition).
- `cargo` package manager.
- Node.js (v18+) for running JavaScript package tests.
- Python 3 for dataset generation scripts.

### Build and Check
```bash
# Check compilation across all features and targets
cargo check --all-targets --all-features

# Build debug binary
cargo build

# Build optimized release binary
cargo build --release
```

### Running Tests
```bash
# Run all unit and integration tests
cargo test --all-targets

# Run tests with live stdout output
cargo test -- --nocapture

# Run specific test suite
cargo test --test category_tests
cargo test --test dataset_accuracy
```

### Code Formatting & Linting
We enforce zero Clippy warnings and standard `rustfmt` formatting:
```bash
# Check formatting
cargo fmt --check

# Apply formatting
cargo fmt

# Run strict Clippy lints
cargo clippy --all-targets --all-features
```

---

## 5. Dataset Generation & Benchmarking

### Generating the 100-Email Evaluation Dataset
To regenerate or inspect the ground-truth benchmark dataset:
```bash
python3 scratch/generate_emails.py
```
This populates `data/emails.json` with 100 realistic email records across 10 categories, each labeled with expected categories and field extractions.

### Running the Native Benchmark
```bash
# Single-threaded mode (default: 1 thread)
cargo run --release

# Multi-threaded mode with custom thread pool
cargo run --release -- --threads 4

# Multi-threaded mode using all available CPU cores
cargo run --release -- --multi-thread
```

### Running the JavaScript / WASM Benchmark
```bash
node test_rover.js
```
This executes the benchmark over `sample_5_types.json` and `data/emails.json`, outputting p50, p95, and p99 latencies and updating `TEST_RESULTS.md`.

---

## 6. Compiling WebAssembly & Mobile Targets

### Building WebAssembly (`wasm32-unknown-unknown`)
```bash
# 1. Compile optimized release target
cargo build --target wasm32-unknown-unknown --release

# 2. Copy compiled binary into packages/rover/eng/
cp target/wasm32-unknown-unknown/release/relay_rover.wasm packages/rover/eng/relay_rover.wasm
```

### Cross-Compiling for Mobile JSI / Nitro (Optional)
If embedding Rover natively into mobile binaries rather than WASM:
```bash
# Android NDK targets
cargo build --target aarch64-linux-android --release
cargo build --target x86_64-linux-android --release

# iOS targets
cargo build --target aarch64-apple-ios --release
cargo build --target aarch64-apple-ios-sim --release
```

---

## 7. How to Add a New Category

To add a new category (for example, `Tax`):

### Step 1: Add Variant to `Category` (`src/models.rs`)
```rust
pub enum Category {
    // ...
    Tax,
}
```
Update `Category::as_str()` and `Category::all_specialized()`.

### Step 2: Define the Extraction Model (`src/models.rs`)
```rust
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaxExtraction {
    pub form_name: Option<String>,
    pub tax_year: Option<String>,
    pub amount: Option<Decimal>,
    pub timeline_key: Option<String>,
    pub raw_evidence: Option<String>,
}
```
Add `Tax(TaxExtraction)` to `pub enum ExtractedData` and update `ExtractedData::category(&self)`.

### Step 3: Implement the Filter (`src/filters/tax.rs`)
```rust
use crate::models::{Category, EmailRecord, ExtractedData, TaxExtraction};
use crate::filters::EmailFilter;

pub struct TaxFilter {
    // Pre-compile all regexes here during new()!
}

impl TaxFilter {
    pub fn new() -> Result<Self, regex::Error> {
        Ok(Self {})
    }
}

pub const TAX_DETECTION_PATTERNS: &[&str] = &[
    r"(?i)\b(?:tax\s+return|form\s+w-2|form\s+1099|tax\s+statement)\b",
];

impl EmailFilter for TaxFilter {
    fn category(&self) -> Category { Category::Tax }
    fn detection_patterns(&self) -> &[&'static str] { TAX_DETECTION_PATTERNS }
    fn extract(&self, email: &EmailRecord) -> ExtractedData {
        ExtractedData::Tax(TaxExtraction::default())
    }
    fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String {
        "Tax statement available".to_string()
    }
}
```

### Step 4: Register in `src/filters/mod.rs` & `src/registry.rs`
1. In `src/filters/mod.rs`, add:
   ```rust
   pub mod tax;
   pub use tax::TaxFilter;
   ```
2. In `src/registry.rs`, register the filter inside `RegistryBuilder::build()`.

### Step 5: Update `src/preview.rs` & `src/engine.rs`
1. Add preview formatting logic in `src/preview.rs`.
2. Add dual-layer classification mappings in `EmailEngine::classify_email()`.

### Step 6: Add Unit Tests (`tests/category_tests.rs`)
Add dedicated test cases verifying both positive category detection and accurate field extraction.

---

## 8. Coding & Performance Standards

1. **Pre-Compilation Rule**: Never call `Regex::new()` inside extraction loops or request handlers. Compile all regexes inside `new()` constructors and store them in struct fields.
2. **No Catastrophic Backtracking**:
   - Avoid nested unbounded quantifiers like `(a+)+` or `.*.*`.
   - Use strict character classes (e.g. `[A-Za-z0-9_-]`) rather than open-ended `.*`.
3. **Zero Unwraps in Production Paths**: Return `Result<T, Error>` for fallible operations. Use `.and_then()`, `.map()`, or `?` rather than `.unwrap()`.
4. **Memory Reuse**: Avoid cloning large strings where references suffice. Format preview strings with pre-allocated capacity.
5. **Continuous Verification**: Pull requests must pass `cargo test`, `cargo clippy --all-targets --all-features`, and `cargo fmt --check`.
