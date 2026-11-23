# Relay Rover — Test & Production Intelligence Report

> **Engine:** Relay Rover (Zero-LLM Deterministic Rust Regex WebAssembly Engine)  
> **Package:** `@rhq/rover` (`react-native-relay-rover`)  
> **Generated At:** 2026-11-23T09:50:07.371Z  
> **Architecture Alignment:** Fully aligned with [*Meet Rover: The Intelligence Behind Relay Mail*](https://blogs.relaymail.online/blog/rover)  

---

## 🚀 High-Resolution Performance & Benchmark

Rover runs directly inside the client JavaScript/React Native runtime via linear WebAssembly memory without child-process overhead or cloud LLM latency.

| Metric | Measured Value | Unit | Target / SLA |
|---|---|---|---|
| **Cold Start (Instantiation)** | `6.27 ms` | ms | < 20 ms |
| **Median Latency (p50)** | `164.5 µs` (`0.165 ms`) | µs / ms | < 500 µs |
| **95th Percentile (p95)** | `1065.1 µs` (`1.065 ms`) | µs / ms | < 1,500 µs |
| **99th Percentile (p99)** | `1439.1 µs` (`1.439 ms`) | µs / ms | < 3,000 µs |
| **Average Latency** | `480.5 µs` | µs | < 800 µs |
| **Min Latency** | `60.6 µs` | µs | - |
| **Max Latency** | `1738.9 µs` | µs | - |
| **Throughput** | **`2,069 emails/sec`** | emails/s | > 1,000 emails/s |
| **Total Benchmark Volume** | `150 inferences` | iterations | - |

---

## 🧠 Dual-Layer Intelligence Matrix

Rover implements dual-layer classification to bridge traditional inbox folders with purpose-driven intelligence:

1. **General Category (Folder Layer):** `Primary`, `Social`, `Promotions`, `Updates`, `Forums`
2. **Purpose Category (Action Layer):** `Authentication` (OTP), `Transactions` (Payment/Invoice), `Deliveries` (Shopping), `Jobs`, `SecurityAlerts`, `Travel`, `Subscriptions`, `Social`, `Newsletters`
3. **Urgency & Importance Engine:** `Urgent` (with alert banner `is_alert: true`), `High`, `Normal`, `Low`
4. **Security Classification:** `Authentication`, `PasswordReset`, `BuildFailure`, `SecurityAlert`, `AccountChange`
5. **Traceable Confidence:** Multi-factor scoring based on sender validation, entity structure, and pattern specificity.

## 🔗 The Rover Intelligence Index: Connected Timelines

As outlined in the blog, Rover extracts deterministic `timeline_key` identifiers to seamlessly group related communications into continuous timelines:

### Timeline: `auth:nintendo` (otp)

- **[Nintendo verification code: 0313 (Expires in 24 hours)]** — Sender: `no-reply@accounts.nintendo.com` | Category: `otp`

### Timeline: `payment:FMPIB6973726141` (payment)

- **[₹10.0 paid to DigiGold · Successful]** — Sender: `no-reply@famapp.in` | Category: `payment`

### Timeline: `order:GPA.3399-0949-2150-45472` (shopping)

- **[₹29.00 · Special Airdrop 412 (Google Play)]** — Sender: `googleplay-noreply@google.com` | Category: `shopping`

---

## 1. Real-World Emails: 5 Diverse Categories (`sample_5_types.json`)

Extracted from real emails in `relay-emails.jsonl` to test distinct category detection, typed extraction, and contextual preview generation.

### Summary Comparison Table

| # | Folder | Purpose | Importance | Contextual Preview | Email Subject & Sender | Extracted Rover Data |
|:---:|:---:|:---:|:---:|---|---|---|
| **1** | `SOCIAL` | `SOCIAL` | `NORMAL` | **LinkedIn · Reaction from Udoh Jeremiah** | Udoh Jeremiah - Finance Intern reacted to this pos<br>*From: LinkedIn (updates-noreply@linkedin.com)* | **platform:** `LinkedIn`<br>**activity_type:** `Reaction`<br>**actor:** `Udoh Jeremiah`<br>**relevant_entity:** `I am thrilled to announce…`<br>**post_url:** `https://www.linkedin.com/comm/feed/update/urn:li:activity:7523990675600809984` |
| **2** | `UPDATES` | `JOB` | `NORMAL` | **Job Alert · 8 React Native Developer jobs at Webomindapps** | React Native Developer at Webomindapps. 7 more rea<br>*From: Indeed (donotreply@jobalert.indeed.com)* | **company:** `Webomindapps`<br>**position:** `React Native Developer`<br>**location:** `Khombhadi, Gujarat`<br>**job_count:** `8`<br>**is_job_alert:** `true`<br>**job_url:** `https://in.indeed.com/jobs?q=react+native+expo&hl=en&from=ja&radius=25&alid=69e4af03aebf194d578c213b&tmtk=1k75l8h28h1qm800&utm_campaign=job_alerts&utm_medium=email&utm_source=jobseeker_emails&of=1&fr=t`<br>**service:** `Webomindapps`<br>**summary:** `React Native Developer at Webomindapps. 7 more react native expo jobs` |
| **3** | `PRIMARY` | `OTP` | `URGENT` | **Nintendo verification code: 0313 (Expires in 24 hours)** | [0313] Nintendo Account: E-mail address verificati<br>*From: no-reply@accounts.nintendo.com* | **otp_code:** `0313`<br>**code:** `0313`<br>**service_name:** `Nintendo`<br>**service:** `Nintendo`<br>**expiration_time:** `24 hours`<br>**expires_in:** `24 hours`<br>**auth_purpose:** `Account Registration`<br>**action_url:** `https://www.region-selector-support.nintendo.com/`<br>**timeline_key:** `auth:nintendo` |
| **4** | `UPDATES` | `PAYMENT` | `HIGH` | **₹10.0 paid to DigiGold · Successful** | Your payment of ₹10.0 is successful<br>*From: FamApp (no-reply@famapp.in)* | **amount:** `10.0`<br>**currency:** `INR`<br>**payer:** `Soham Pramanik`<br>**recipient:** `DigiGold`<br>**transaction_id:** `FMPIB6973726141`<br>**transaction_date:** `2026-11-11`<br>**date:** `2026-11-11`<br>**transaction_time:** `03:39 PM IST`<br>**transaction_status:** `Successful`<br>**status:** `Successful`<br>**timeline_key:** `payment:FMPIB6973726141` |
| **5** | `UPDATES` | `SHOPPING` | `NORMAL` | **₹29.00 · Special Airdrop 412 (Google Play)** | Your Google Play Order Receipt from 27 Oct 2026<br>*From: Google Play (googleplay-noreply@google.com)* | **store:** `Google Play`<br>**order_id:** `GPA.3399-0949-2150-45472`<br>**product:** `Special Airdrop 412`<br>**amount:** `29.00`<br>**currency:** `INR`<br>**tax:** `4.42`<br>**order_date:** `27 Oct 2026`<br>**delivery_status:** `Processing`<br>**status:** `Processing`<br>**payment_method:** `Google Play balance`<br>**timeline_key:** `order:GPA.3399-0949-2150-45472` |

### Detailed Side-by-Side: Email Body vs Rover JSON Response

#### Email #1: [SOCIAL] Udoh Jeremiah - Finance Intern reacted to this post: I am thrilled to announce…

| Attribute | Details |
|---|---|
| **Sender** | `LinkedIn <updates-noreply@linkedin.com>` |
| **Subject** | Udoh Jeremiah - Finance Intern reacted to this post: I am thrilled to announce… |
| **Folder / General** | `social` |
| **Purpose** | `SOCIAL` |
| **Importance / Urgency** | `normal` (Alert: `false`) |
| **Preview** | "LinkedIn · Reaction from Udoh Jeremiah" |

**Email Body Text:**
> Udoh Jeremiah reacted to this
> 
> Shruti Grover LIKE PRAISE APPRECIATION 196, 52 Comments
> 
> Udoh Jeremiahcommented a post: You can lose CTRL but please don’t lose CMD
> 
>  Read more: https://www.linkedin.com/comm/feed/update/urn:li:activity:7523990675600809984?highlightedUpdateUrn=urn%3Ali%3Aactivity%3A752...

**Rover JSON Response:**
```json
{
  "category": "social",
  "classification": {
    "general_category": "social",
    "purpose": "Social",
    "importance": "normal",
    "is_alert": false,
    "security": "none",
    "confidence": 0.92,
    "factors": {
      "authenticated_sender": true,
      "has_structured_entities": true,
      "pattern_specificity": "Social network activity and notification pattern matched",
      "subject_body_aligned": true
    },
    "secondary_purposes": []
  },
  "preview": "LinkedIn · Reaction from Udoh Jeremiah",
  "data": {
    "platform": "LinkedIn",
    "activity_type": "Reaction",
    "actor": "Udoh Jeremiah",
    "relevant_entity": "I am thrilled to announce…",
    "post_url": "https://www.linkedin.com/comm/feed/update/urn:li:activity:7523990675600809984",
    "raw_evidence": "Udoh Jeremiah - Finance Intern reacted to this post: I am thrilled to announce…"
  },
  "matched_categories": [
    "social",
    "social"
  ]
}
```

---

#### Email #2: [JOB] React Native Developer at Webomindapps. 7 more react native expo jobs

| Attribute | Details |
|---|---|
| **Sender** | `Indeed <donotreply@jobalert.indeed.com>` |
| **Subject** | React Native Developer at Webomindapps. 7 more react native expo jobs |
| **Folder / General** | `updates` |
| **Purpose** | `JOB` |
| **Importance / Urgency** | `normal` (Alert: `false`) |
| **Preview** | "Job Alert · 8 React Native Developer jobs at Webomindapps" |

**Email Body Text:**
> Indeed Job Alert
> 8 new react native expo vacancies
> 
> Jobs 1-8 of 8 new jobs
> See matching results on Indeed: https://in.indeed.com/jobs?q=react+native+expo&hl=en&from=ja&radius=25&alid=69e4af03aebf194d578c213b&tmtk=1k75l8h28h1qm800&utm_campaign=job_alerts&utm_medium=email&utm_source=jobseeker_emails&o...

**Rover JSON Response:**
```json
{
  "category": "job",
  "classification": {
    "general_category": "updates",
    "purpose": "Job",
    "importance": "normal",
    "is_alert": false,
    "security": "none",
    "confidence": 0.93,
    "factors": {
      "authenticated_sender": true,
      "has_structured_entities": true,
      "pattern_specificity": "Job opportunity and hiring workflow pattern matched",
      "subject_body_aligned": true
    },
    "secondary_purposes": []
  },
  "preview": "Job Alert · 8 React Native Developer jobs at Webomindapps",
  "data": {
    "company": "Webomindapps",
    "position": "React Native Developer",
    "location": "Khombhadi, Gujarat",
    "job_count": 8,
    "is_job_alert": true,
    "job_url": "https://in.indeed.com/jobs?q=react+native+expo&hl=en&from=ja&radius=25&alid=69e4af03aebf194d578c213b&tmtk=1k75l8h28h1qm800&utm_campaign=job_alerts&utm_medium=email&utm_source=jobseeker_emails&of=1&fr=t",
    "service": "Webomindapps",
    "summary": "React Native Developer at Webomindapps. 7 more react native expo jobs",
    "raw_evidence": "React Native Developer at Webomindapps. 7 more react native expo jobs"
  },
  "matched_categories": [
    "job",
    "job"
  ]
}
```

---

#### Email #3: [OTP] [0313] Nintendo Account: E-mail address verification

| Attribute | Details |
|---|---|
| **Sender** | `no-reply@accounts.nintendo.com` |
| **Subject** | [0313] Nintendo Account: E-mail address verification |
| **Folder / General** | `primary` |
| **Purpose** | `OTP` |
| **Importance / Urgency** | `urgent` (Alert: `true`) |
| **Preview** | "Nintendo verification code: 0313 (Expires in 24 hours)" |
| **Timeline Key** | `auth:nintendo` |

**Email Body Text:**
> This is an automatically generated e-mail from Nintendo.
> 
> -------------------------
> Nickname: sohamglx
> 
> Thank you for creating a Nintendo Account.
> Please enter the following verification code to complete the e-mail address registration process.
> 
> Verification code:
> 0313
> 
> Please be aware that if you d...

**Rover JSON Response:**
```json
{
  "category": "otp",
  "classification": {
    "general_category": "primary",
    "purpose": "Otp",
    "importance": "urgent",
    "is_alert": true,
    "security": "authentication",
    "confidence": 0.99,
    "factors": {
      "authenticated_sender": true,
      "has_structured_entities": true,
      "pattern_specificity": "High-precision verification code extraction",
      "subject_body_aligned": true
    },
    "secondary_purposes": []
  },
  "preview": "Nintendo verification code: 0313 (Expires in 24 hours)",
  "timeline_key": "auth:nintendo",
  "data": {
    "otp_code": "0313",
    "code": "0313",
    "service_name": "Nintendo",
    "service": "Nintendo",
    "expiration_time": "24 hours",
    "expires_in": "24 hours",
    "auth_purpose": "Account Registration",
    "action_url": "https://www.region-selector-support.nintendo.com/",
    "timeline_key": "auth:nintendo",
    "raw_evidence": "[0313] Nintendo Account: E-mail address verification"
  },
  "matched_categories": [
    "otp",
    "otp"
  ]
}
```

---

#### Email #4: [PAYMENT] Your payment of ₹10.0 is successful

| Attribute | Details |
|---|---|
| **Sender** | `FamApp <no-reply@famapp.in>` |
| **Subject** | Your payment of ₹10.0 is successful |
| **Folder / General** | `updates` |
| **Purpose** | `PAYMENT` |
| **Importance / Urgency** | `high` (Alert: `false`) |
| **Preview** | "₹10.0 paid to DigiGold · Successful" |
| **Timeline Key** | `payment:FMPIB6973726141` |

**Email Body Text:**
> Hey Soham Pramanik, You have successfully paid ₹10.0 to DigiGold at 03:39 PM IST, 11 November 2026 with transaction id FMPIB6973726141. Your updated balance is ₹12.6. If this was not done by you, report this transaction at support@famapp.in

**Rover JSON Response:**
```json
{
  "category": "payment",
  "classification": {
    "general_category": "updates",
    "purpose": "Payment",
    "importance": "high",
    "is_alert": false,
    "security": "none",
    "confidence": 0.96,
    "factors": {
      "authenticated_sender": true,
      "has_structured_entities": true,
      "pattern_specificity": "Deterministic monetary transaction and payment entity detected",
      "subject_body_aligned": true
    },
    "secondary_purposes": []
  },
  "preview": "₹10.0 paid to DigiGold · Successful",
  "timeline_key": "payment:FMPIB6973726141",
  "data": {
    "amount": "10.0",
    "currency": "INR",
    "payer": "Soham Pramanik",
    "recipient": "DigiGold",
    "transaction_id": "FMPIB6973726141",
    "transaction_date": "2026-11-11",
    "date": "2026-11-11",
    "transaction_time": "03:39 PM IST",
    "transaction_status": "Successful",
    "status": "Successful",
    "timeline_key": "payment:FMPIB6973726141",
    "raw_evidence": "Your payment of ₹10.0 is successful"
  },
  "matched_categories": [
    "payment",
    "payment"
  ]
}
```

---

#### Email #5: [SHOPPING] Your Google Play Order Receipt from 27 Oct 2026

| Attribute | Details |
|---|---|
| **Sender** | `Google Play <googleplay-noreply@google.com>` |
| **Subject** | Your Google Play Order Receipt from 27 Oct 2026 |
| **Folder / General** | `updates` |
| **Purpose** | `SHOPPING` |
| **Importance / Urgency** | `normal` (Alert: `false`) |
| **Preview** | "₹29.00 · Special Airdrop 412 (Google Play)" |
| **Timeline Key** | `order:GPA.3399-0949-2150-45472` |

**Email Body Text:**
> Google Play
> 
> 
> Thank you
> 
> 
> You've made a purchase from MOCO STUDIOS PRIVATE LIMITED on Google Play.
> 
> 
> 
> Order number: GPA.3399-0949-2150-45472
> Order date: 27 Oct 2026 12:29:12 GMT+5:30
> 
> 
> Item Price
> 
> 
> Special Airdrop 412 (Free Fire MAX x NARUTO) ₹29.00
> 
> 
> 
> Total: ₹29.00
> 
> (Includes GST of ₹4.42)
> 
> 
> Paymen...

**Rover JSON Response:**
```json
{
  "category": "shopping",
  "classification": {
    "general_category": "updates",
    "purpose": "Shopping",
    "importance": "normal",
    "is_alert": false,
    "security": "none",
    "confidence": 0.95,
    "factors": {
      "authenticated_sender": true,
      "has_structured_entities": true,
      "pattern_specificity": "Commerce order and shipment tracking entities detected",
      "subject_body_aligned": true
    },
    "secondary_purposes": []
  },
  "preview": "₹29.00 · Special Airdrop 412 (Google Play)",
  "timeline_key": "order:GPA.3399-0949-2150-45472",
  "data": {
    "store": "Google Play",
    "order_id": "GPA.3399-0949-2150-45472",
    "product": "Special Airdrop 412",
    "amount": "29.00",
    "currency": "INR",
    "tax": "4.42",
    "order_date": "27 Oct 2026",
    "delivery_status": "Processing",
    "status": "Processing",
    "payment_method": "Google Play balance",
    "timeline_key": "order:GPA.3399-0949-2150-45472",
    "raw_evidence": "Your Google Play Order Receipt from 27 Oct 2026"
  },
  "matched_categories": [
    "shopping",
    "shopping"
  ]
}
```

---

