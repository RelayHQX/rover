# Contributing to Relay Rover Engine

Thank you for your interest in contributing to **Relay Rover Engine** (codename **Rover**)! This document provides a comprehensive guide to understanding the codebase, building and running tests, extending filters and extractors, adding custom regex rules, and contributing to the Rover ecosystem.

---

## 1. Project Mission & Design Principles

Relay Rover is built to be:
1. **Deterministic**: No probabilistic LLM calls, zero network dependency, and 100% reproducible outcomes.
2. **Predictable & Fast**: Single-pass `RegexSet` category detection followed by targeted extraction in microseconds.
3. **Robust**: Resilient against catastrophic backtracking, zero `.unwrap()` in runtime paths, and defensive fallbacks.
4. **Extensible**: Clean public API (`Rover`, `RoverEngine`, `RoverBuilder`, `EmailFilter`) allowing downstream developers to customize rules.
5. **Maintainable**: Strict separation of concerns between models, extractors, filters, engine, and benchmarks.

---

## 2. Directory & File Reference Map

```text
relay-intelligence/
├── Cargo.toml                  # Package manifest, dependencies, features (wasm), crate types
├── data/
│   └── emails.json             # 100 realistic synthetic benchmark emails with ground-truth labels
├── src/
│   ├── lib.rs                  # Root library crate exposing public API (Rover, RoverEngine, RoverBuilder)
│   ├── main.rs                 # CLI entry point running benchmark suites and terminal report
│   ├── models.rs               # Category enum, typed extraction structs, EmailRecord, ProcessedEmail
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
│   │   └── general.rs          # General personal communication excerpt fallback
│   └── extractors/             # Shared reusable entity extraction components
│       ├── mod.rs              # Extractor module declarations and re-exports
│       ├── amount.rs           # Currency symbol and Decimal monetary parser
│       ├── date.rs             # ISO (YYYY-MM-DD) and verbal date parser
│       ├── identity.rs         # Merchant, issuer, provider, store, and person parser
│       ├── transaction.rs      # TXN, Order ID, Booking ID, PNR, Tracking number parser
│       └── otp.rs              # Authentication code and expiration parser
└── tests/
    ├── category_tests.rs       # Dedicated unit tests for every category and builder API
    └── dataset_accuracy.rs     # End-to-end accuracy evaluation against ground-truth dataset
```

---

## 3. Development Workflow

### Prerequisites
- Stable Rust (1.80+ or 2024 edition).
- `cargo` package manager.

### Build and Check
```bash
# Check compilation across all features
cargo check --all-targets --all-features

# Build debug binary
cargo build

# Build optimized release binary
cargo build --release
```

### Run Tests
```bash
# Run all unit and integration tests
cargo test

# Run tests with stdout output
cargo test -- --nocapture
```

### Code Formatting and Linting
We enforce strict formatting and zero-warning Clippy standards:
```bash
# Check formatting
cargo fmt --check

# Apply formatting
cargo fmt

# Run Clippy lints
cargo clippy --all-targets --all-features
```

---

## 4. How to Add a New Category

Follow these steps to add a new category (e.g. `Tax` or `Medical`):

### Step 1: Add the Variant to `Category` (`src/models.rs`)
1. Add the new variant to `pub enum Category`:
   ```rust
   pub enum Category {
       // ... existing variants
       Tax,
   }
   ```
2. Update `Category::as_str()` and `Category::all_specialized()`.

### Step 2: Define the Extraction Model (`src/models.rs`)
1. Define the typed extraction struct:
   ```rust
   #[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
   pub struct TaxExtraction {
       pub form_name: Option<String>,
       pub tax_year: Option<String>,
       pub amount: Option<Decimal>,
       pub raw_evidence: Option<String>,
   }
   ```
2. Add the variant to `pub enum ExtractedData`:
   ```rust
   pub enum ExtractedData {
       // ...
       Tax(TaxExtraction),
   }
   ```
3. Update `ExtractedData::category(&self)`.

### Step 3: Implement the Filter (`src/filters/tax.rs`)
Create `src/filters/tax.rs`:
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
        // Perform typed extraction
        ExtractedData::Tax(TaxExtraction::default())
    }
    fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String {
        "Tax statement available".to_string()
    }
}
```

### Step 4: Register in `src/filters/mod.rs` & `src/registry.rs`
1. In `src/filters/mod.rs`:
   ```rust
   pub mod tax;
   pub use tax::TaxFilter;
   ```
2. In `src/registry.rs`:
   In `RegistryBuilder::build()`, add:
   ```rust
   if let Entry::Vacant(e) = self.filters.entry(Category::Tax) {
       e.insert(Box::new(TaxFilter::new()?));
   }
   ```

### Step 5: Update `src/preview.rs`
Add a branch in `generate_preview()` to format clean notifications for the new category.

### Step 6: Add Unit Tests (`tests/category_tests.rs`)
Write a dedicated unit test ensuring both positive detection and accurate field extraction.

---

## 5. How to Add a New Regex Pattern or Extractor Rule

### Pre-Compilation Rule
> **IMPORTANT**: Never call `Regex::new()` inside an execution loop or extraction function.
> Always compile regular expressions in the `new()` method of the extractor or filter, and store them as struct fields.

### Adding Shared Extractor Rules
If you are adding a new identifier format (e.g., a new shipping carrier or bank reference format):
1. Navigate to `src/extractors/`.
2. Locate the relevant extractor (`transaction.rs`, `amount.rs`, `identity.rs`, etc.).
3. Update the regex pattern in `new()`.
4. Test with both matching samples and negative distractor strings.

---

## 6. Coding & Performance Standards

1. **No Catastrophic Backtracking**:
   - Avoid unbounded nested quantifiers like `(a+)+` or `.*.*`.
   - Prefer character classes like `[A-Za-z0-9_-]` over generic `.*`.
2. **Zero Unwraps in Production**:
   - Return `Result<T, Error>` for fallible operations.
   - Use `and_then()`, `map()`, or `?` instead of `.unwrap()`.
3. **Memory Reuse**:
   - Avoid cloning strings unless necessary.
   - Format preview strings with pre-allocated capacity.
4. **Validation**:
   - Every PR must pass `cargo test`, `cargo clippy --all-targets --all-features`, and `cargo fmt --check`.


## 5. In-Depth Analysis: How Each Specific Extractor Works

### 1. `AmountExtractor` (`src/extractors/amount.rs`)
- **Challenge**: Standard regex word boundaries (`\b`) fail on non-word currency symbols like `$`, `€`, `₹`, `£`, `¥` when preceded by spaces. Floating-point numbers also risk rounding errors.
- **Solution**:
  - Uses `(?:^|[^\w])(?P<curr>...)\s*(?P<val>...)` to match symbol currencies cleanly without boundary failures.
  - Parses quantities directly into `rust_decimal::Decimal`, avoiding binary floating-point representation bugs.
  - Normalizes symbols (`₹`, `INR`) and lowercase currencies into standard ISO 4217 uppercase codes (`INR`, `USD`, `EUR`, `GBP`, `JPY`).

### 2. `DateExtractor` (`src/extractors/date.rs`)
- **Challenge**: Emails contain multiple dates in conflicting formats (e.g. email sent date vs transaction date vs payment due date).
- **Solution**:
  - Matches standard ISO 8601 dates (`YYYY-MM-DD`).
  - Supports verbal English dates (`15 November 2026`) and converts them deterministically to standardized numeric format.
  - Supports slash formats (`MM/DD/YYYY`).

### 3. `OtpExtractor` (`src/extractors/otp.rs`)
- **Challenge**: Six-digit order numbers, phone numbers, zip codes, and account numbers must not be mistakenly classified as verification codes.
- **Solution**:
  - Contextual anchoring: requires explicit security tokens in immediate proximity (e.g. `verification code is [0-9]{4,8}`, `security code [0-9]{4,8}`, `OTP: [0-9]{4,8}`).
  - Extracts duration strings (`expires in 10 minutes`, `valid for 5 minutes`) and associated service names.

### 4. `TransactionExtractor` (`src/extractors/transaction.rs`)
- **Challenge**: Distinguishing transaction IDs from order IDs, airline PNRs, invoice numbers, and shipping tracking numbers.
- **Solution**:
  - Employs distinct pattern families for each entity type:
    - Transaction IDs: `TXN-...`, `PF-...`, `Ref: ...`, `ARN: ...`, `UPI Ref: ...`
    - Order IDs: `SQ-...`, `ORD-...`, `#HS-...`, `Order: ...`
    - Booking IDs / PNR: `PNR SK-...`, `Ref #GM-...`, `Booking ID: ...`
    - Invoice Numbers: `INV-...`, `ME-...`, `GMP-...`
    - Tracking Numbers: `TRK-...`, `FEDX-...`, `DHL-...`, `UPS-...`

### 5. `IdentityExtractor` (`src/extractors/identity.rs`)
- **Challenge**: Extracting merchant and recipient names from freeform sentences without greedily capturing trailing sentences.
- **Solution**:
  - Explicit termination delimiters: boundaries terminate on `\s+on`, `\s+was`, `\s+for`, `\n`, periods, or transaction tags.
  - Defensive filtering: strips false positive tokens (e.g. `account IBAN...` or masked card numbers).

---

## 7. In-Depth Analysis: How Each Filter Works

### 1. `PaymentFilter` (`src/filters/payment.rs`)
- **Detection**: Single-pass triggers on `debited by`, `wire transfer of`, `payment confirmed`, `UPI payment of`, `payout confirmation`.
- **Extraction**: Extracts `amount`, `currency`, `recipient`, `transaction_id`, `transaction_date`, and `transaction_status`.
- **Preview**: `"₹2,500 paid to QuickMart Retail · Successful"`.

### 2. `OtpFilter` (`src/filters/otp.rs`)
- **Detection**: Matches security and 2FA authentication alerts (`verification code`, `one-time password`, `verification PIN`).
- **Extraction**: Extracts `otp_code`, `service_name`, and `expiration_time`.
- **Preview**: `"Acme Portal verification code · 482913"`.

### 3. `JobFilter` (`src/filters/job.rs`)
- **Detection**: Matches interview invitations, job offers, application acknowledgements, and coding assessments.
- **Extraction**: Extracts `company`, `position` (with suffix cleaning to remove redundant words like `"role"` or `"vacancy"`), `location`, `salary`, `deadline`, and `interview_date`.
- **Preview**: `"Interview invitation · Senior Rust Systems Engineer"`.

### 4. `ShoppingFilter` (`src/filters/shopping.rs`)
- **Detection**: Matches order confirmations, shipments, dispatch alerts, and package tracking updates.
- **Extraction**: Extracts `store`, `order_id`, `product`, `amount`, `delivery_status`, and `tracking_number`.
- **Preview**: `"Processing · Tracking available"`.

### 5. `InvoiceFilter` (`src/filters/invoice.rs`)
- **Detection**: Matches bill statements, utility invoices, platform fee statements, and payment due notices.
- **Extraction**: Extracts `invoice_number`, `amount`, `currency`, `due_date`, and `issuer`.
- **Preview**: `"Invoice received · $420.50 due 2026-11-20"`.

### 6. `SubscriptionFilter` (`src/filters/subscription.rs`)
- **Detection**: Matches plan auto-renewals, recurring membership notices, and digital subscription updates.
- **Extraction**: Extracts `provider`, `subscription_amount`, `billing_frequency` (monthly/annual), `renewal_date`, and `cancellation_info`.
- **Preview**: `"SoundWave Music · monthly renewal on 2026-11-28"`.

### 7. `TravelFilter` (`src/filters/travel.rs`)
- **Detection**: Matches flight bookings, hotel reservations, train e-tickets, and car rental confirmations.
- **Extraction**: Extracts `booking_id` / PNR, `destination`, `departure_date`, `return_date`, and `booking_status`.
- **Preview**: `"Booking to Tokyo (HND) · Confirmed for 2026-12-05"`.

### 8. `NewsletterFilter` (`src/filters/newsletter.rs`)
- **Detection**: Matches weekly digests, market briefs, engineering roundups, and unsubscribe markers.
- **Extraction**: Extracts `publisher`, `main_topic`, and `headline`.
- **Preview**: `"Rust Weekly Digest · Rust 2024 Edition stabilized and async closures ready"`.

### 9. `SocialFilter` (`src/filters/social.rs`)
- **Detection**: Matches connection invitations, mentions, PR reviews, video comments, and reactions.
- **Extraction**: Extracts `platform`, `activity_type`, and `relevant_entity` (actor name).
- **Preview**: `"ProConnect · Connection request from Alex Vance"`.

### 10. `GeneralFilter` (`src/filters/general.rs`)
- **Detection**: Fallback filter triggered when no specialized filter matches.
- **Extraction**: Generates a clean text summary of the first complete sentence.
- **Preview**: Sanitized, whitespace-normalized excerpt truncated cleanly at word boundaries (< 95 chars).

---