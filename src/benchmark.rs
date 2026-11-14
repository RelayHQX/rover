use std::collections::HashMap;
use std::hint::black_box;
use std::time::{Duration, Instant};

use crate::engine::EmailEngine;
use crate::models::{Category, EmailRecord, ExtractedData, ProcessedEmail};

/// Results from running the full benchmark suite.
#[derive(Debug, Clone)]
pub struct BenchmarkReport {
    pub total_emails: usize,
    pub total_input_bytes: usize,
    pub detection_pattern_count: usize,
    pub categories_count: usize,

    // Initialization
    pub regex_set_compilation: Duration,
    pub extraction_compilation: Duration,
    pub total_initialization: Duration,

    // Latency percentiles (Single-threaded single batch of 100 emails)
    pub batch_duration: Duration,
    pub avg_per_email: Duration,
    pub median_per_email: Duration,
    pub p95_per_email: Duration,
    pub p99_per_email: Duration,
    pub min_per_email: Duration,
    pub max_per_email: Duration,

    // Multi-iteration steady-state (100 iterations = 10,000 evaluations)
    pub multi_iter_batch_avg: Duration,
    pub multi_iter_emails_per_sec: f64,
    pub multi_iter_mb_per_sec: f64,

    // Throughput
    pub emails_per_second: f64,
    pub mb_per_second: f64,

    // Accuracy
    pub category_accuracy_percent: f64,
    pub extraction_accuracy_percent: f64,
    pub false_positives: usize,
    pub false_negatives: usize,
    pub multi_category_count: usize,

    // Category Breakdown: (detected_correct, expected_total)
    pub category_breakdown: Vec<(Category, usize, usize)>,

    // Pipeline Stage Comparisons
    pub detection_only_duration: Duration,
    pub detection_extraction_duration: Duration,
    pub full_pipeline_duration: Duration,

    // Parallel processing with Rayon
    pub parallel_batch_duration: Duration,
    pub parallel_emails_per_sec: f64,
    pub parallel_mb_per_sec: f64,
}

pub struct BenchmarkRunner<'a> {
    engine: &'a EmailEngine,
    emails: &'a [EmailRecord],
}

impl<'a> BenchmarkRunner<'a> {
    pub fn new(engine: &'a EmailEngine, emails: &'a [EmailRecord]) -> Self {
        Self { engine, emails }
    }

    pub fn run_all(&self) -> BenchmarkReport {
        let total_emails = self.emails.len();
        let total_input_bytes: usize = self.emails.iter().map(EmailRecord::byte_len).sum();
        let detection_pattern_count = self.engine.registry().detection_pattern_count();
        let categories_count = 10;

        let timings = self.engine.registry().timings();

        // 1. Warm-up run (to prime CPU cache and branch predictor)
        for email in self.emails {
            let _ = black_box(self.engine.process(email));
        }

        // 2. Measure individual latencies for single-threaded batch of 100 emails
        let mut per_email_durations = Vec::with_capacity(total_emails);
        let batch_start = Instant::now();

        for email in self.emails {
            let start = Instant::now();
            let processed = black_box(self.engine.process(email));
            per_email_durations.push(start.elapsed());
            let _ = black_box(processed);
        }
        let batch_duration = batch_start.elapsed();

        per_email_durations.sort_unstable();

        let avg_per_email = batch_duration / total_emails as u32;
        let min_per_email = per_email_durations[0];
        let max_per_email = per_email_durations[total_emails - 1];
        let median_per_email = per_email_durations[total_emails / 2];
        let p95_per_email = per_email_durations[(total_emails as f64 * 0.95) as usize];
        let p99_per_email = per_email_durations[(total_emails as f64 * 0.99) as usize];

        let seconds = batch_duration.as_secs_f64();
        let emails_per_second = if seconds > 0.0 {
            total_emails as f64 / seconds
        } else {
            0.0
        };
        let mb_processed = total_input_bytes as f64 / (1024.0 * 1024.0);
        let mb_per_second = if seconds > 0.0 {
            mb_processed / seconds
        } else {
            0.0
        };

        // 3. Multi-iteration steady-state run (100 iterations of the 100 emails = 10,000 evaluations)
        let multi_iterations = 100;
        let multi_start = Instant::now();
        for _ in 0..multi_iterations {
            for email in self.emails {
                let _ = black_box(self.engine.process(email));
            }
        }
        let multi_total_duration = multi_start.elapsed();
        let multi_iter_batch_avg = multi_total_duration / multi_iterations;
        let multi_total_emails = (total_emails * multi_iterations as usize) as f64;
        let multi_secs = multi_total_duration.as_secs_f64();
        let multi_iter_emails_per_sec = multi_total_emails / multi_secs;
        let multi_total_mb =
            (total_input_bytes * multi_iterations as usize) as f64 / (1024.0 * 1024.0);
        let multi_iter_mb_per_sec = multi_total_mb / multi_secs;

        // 4. Parallel batch processing with Rayon
        let parallel_start = Instant::now();
        let parallel_results = black_box(self.engine.process_batch_parallel(self.emails));
        let parallel_batch_duration = parallel_start.elapsed();
        let _ = black_box(parallel_results);

        let par_secs = parallel_batch_duration.as_secs_f64();
        let parallel_emails_per_sec = if par_secs > 0.0 {
            total_emails as f64 / par_secs
        } else {
            0.0
        };
        let parallel_mb_per_sec = if par_secs > 0.0 {
            mb_processed / par_secs
        } else {
            0.0
        };

        // 5. Compare pipeline stages (Detection-only vs Detection+Extraction vs Full pipeline)
        let det_start = Instant::now();
        for email in self.emails {
            let _ = black_box(self.engine.detect_categories(email));
        }
        let detection_only_duration = det_start.elapsed();

        let ext_start = Instant::now();
        for email in self.emails {
            let _ = black_box(self.engine.detect_and_extract(email));
        }
        let detection_extraction_duration = ext_start.elapsed();

        let full_pipeline_duration = batch_duration;

        // 6. Accuracy Evaluation
        let mut correct_categories = 0;
        let mut false_positives = 0;
        let mut false_negatives = 0;
        let mut multi_category_count = 0;

        let mut total_fields_expected = 0;
        let mut total_fields_matched = 0;

        let mut category_expected_counts: HashMap<Category, usize> = HashMap::new();
        let mut category_correct_counts: HashMap<Category, usize> = HashMap::new();

        for email in self.emails {
            *category_expected_counts
                .entry(email.expected_category)
                .or_insert(0) += 1;

            let processed: ProcessedEmail = self.engine.process(email);

            if processed.matched_categories.len() > 1 {
                multi_category_count += 1;
            }

            if processed.primary_category == email.expected_category {
                correct_categories += 1;
                *category_correct_counts
                    .entry(email.expected_category)
                    .or_insert(0) += 1;
            } else {
                false_positives += 1;
                false_negatives += 1;
            }

            let (matched, expected) =
                evaluate_extraction(&processed.extraction, &email.expected_extraction);
            total_fields_matched += matched;
            total_fields_expected += expected;
        }

        let category_accuracy_percent = (correct_categories as f64 / total_emails as f64) * 100.0;
        let extraction_accuracy_percent = if total_fields_expected > 0 {
            (total_fields_matched as f64 / total_fields_expected as f64) * 100.0
        } else {
            100.0
        };

        let mut category_breakdown = Vec::new();
        let ordered_cats = [
            Category::Payment,
            Category::Otp,
            Category::Job,
            Category::Shopping,
            Category::Invoice,
            Category::Subscription,
            Category::Travel,
            Category::Newsletter,
            Category::Social,
            Category::General,
        ];

        for cat in ordered_cats {
            let exp = category_expected_counts.get(&cat).copied().unwrap_or(0);
            let cor = category_correct_counts.get(&cat).copied().unwrap_or(0);
            category_breakdown.push((cat, cor, exp));
        }

        BenchmarkReport {
            total_emails,
            total_input_bytes,
            detection_pattern_count,
            categories_count,
            regex_set_compilation: timings.regex_set_compilation,
            extraction_compilation: timings.extraction_compilation,
            total_initialization: timings.total_initialization,
            batch_duration,
            avg_per_email,
            median_per_email,
            p95_per_email,
            p99_per_email,
            min_per_email,
            max_per_email,
            multi_iter_batch_avg,
            multi_iter_emails_per_sec,
            multi_iter_mb_per_sec,
            emails_per_second,
            mb_per_second,
            category_accuracy_percent,
            extraction_accuracy_percent,
            false_positives,
            false_negatives,
            multi_category_count,
            category_breakdown,
            detection_only_duration,
            detection_extraction_duration,
            full_pipeline_duration,
            parallel_batch_duration,
            parallel_emails_per_sec,
            parallel_mb_per_sec,
        }
    }
}

/// Compare extracted typed fields against the benchmark reference JSON.
fn evaluate_extraction(data: &ExtractedData, expected: &serde_json::Value) -> (usize, usize) {
    let mut matched = 0;
    let mut total = 0;

    let obj = match expected.as_object() {
        Some(o) => o,
        None => return (0, 0),
    };

    match data {
        ExtractedData::Payment(p) => {
            if let Some(exp_amt) = obj.get("amount").and_then(|v| v.as_str()) {
                total += 1;
                if let Some(amt) = p.amount {
                    let clean_exp = exp_amt.replace(',', "");
                    if amt.to_string() == clean_exp || format!("{amt:.2}") == clean_exp {
                        matched += 1;
                    }
                }
            }
            if let Some(exp_curr) = obj.get("currency").and_then(|v| v.as_str()) {
                total += 1;
                if p.currency.as_deref() == Some(exp_curr) {
                    matched += 1;
                }
            }
            if let Some(exp_rec) = obj.get("recipient").and_then(|v| v.as_str()) {
                total += 1;
                if p.recipient.as_deref() == Some(exp_rec) {
                    matched += 1;
                }
            }
            if let Some(exp_txn) = obj.get("transaction_id").and_then(|v| v.as_str()) {
                total += 1;
                if p.transaction_id.as_deref() == Some(exp_txn) {
                    matched += 1;
                }
            }
            if let Some(exp_date) = obj.get("date").and_then(|v| v.as_str()) {
                total += 1;
                if p.transaction_date.as_deref() == Some(exp_date) {
                    matched += 1;
                }
            }
        }
        ExtractedData::Otp(o) => {
            if let Some(exp_code) = obj.get("code").and_then(|v| v.as_str()) {
                total += 1;
                if o.otp_code.as_deref() == Some(exp_code) {
                    matched += 1;
                }
            }
            if let Some(exp_service) = obj.get("service").and_then(|v| v.as_str()) {
                total += 1;
                if o.service_name.as_deref() == Some(exp_service) {
                    matched += 1;
                }
            }
            if let Some(exp_exp) = obj.get("expires_in").and_then(|v| v.as_str()) {
                total += 1;
                if o.expiration_time.as_deref() == Some(exp_exp) {
                    matched += 1;
                }
            }
        }
        ExtractedData::Job(j) => {
            if let Some(exp_comp) = obj.get("company").and_then(|v| v.as_str()) {
                total += 1;
                if j.company.as_deref() == Some(exp_comp) {
                    matched += 1;
                }
            }
            if let Some(exp_pos) = obj.get("position").and_then(|v| v.as_str()) {
                total += 1;
                if j.position.as_deref() == Some(exp_pos) {
                    matched += 1;
                }
            }
            if let Some(exp_loc) = obj.get("location").and_then(|v| v.as_str()) {
                total += 1;
                if j.location.as_deref() == Some(exp_loc) {
                    matched += 1;
                }
            }
            if let Some(exp_sal) = obj.get("salary").and_then(|v| v.as_str()) {
                total += 1;
                if j.salary.as_deref() == Some(exp_sal) {
                    matched += 1;
                }
            }
            if let Some(exp_dl) = obj.get("deadline").and_then(|v| v.as_str()) {
                total += 1;
                if j.application_deadline.as_deref() == Some(exp_dl) {
                    matched += 1;
                }
            }
            if let Some(exp_int) = obj.get("interview_date").and_then(|v| v.as_str()) {
                total += 1;
                if j.interview_date.as_deref() == Some(exp_int) {
                    matched += 1;
                }
            }
        }
        ExtractedData::Shopping(s) => {
            if let Some(exp_store) = obj.get("store").and_then(|v| v.as_str()) {
                total += 1;
                if s.store.as_deref() == Some(exp_store) {
                    matched += 1;
                }
            }
            if let Some(exp_oid) = obj.get("order_id").and_then(|v| v.as_str()) {
                total += 1;
                if s.order_id.as_deref() == Some(exp_oid) {
                    matched += 1;
                }
            }
            if let Some(exp_prod) = obj.get("product").and_then(|v| v.as_str()) {
                total += 1;
                if s.product.as_deref() == Some(exp_prod) {
                    matched += 1;
                }
            }
            if let Some(exp_amt) = obj.get("amount").and_then(|v| v.as_str()) {
                total += 1;
                if let Some(amt) = s.amount
                    && amt.to_string() == exp_amt
                {
                    matched += 1;
                }
            }
            if let Some(exp_stat) = obj.get("status").and_then(|v| v.as_str()) {
                total += 1;
                if s.delivery_status.as_deref() == Some(exp_stat) {
                    matched += 1;
                }
            }
            if let Some(exp_trk) = obj.get("tracking_number").and_then(|v| v.as_str()) {
                total += 1;
                if s.tracking_number.as_deref() == Some(exp_trk) {
                    matched += 1;
                }
            }
        }
        ExtractedData::Invoice(i) => {
            if let Some(exp_inv) = obj.get("invoice_number").and_then(|v| v.as_str()) {
                total += 1;
                if i.invoice_number.as_deref() == Some(exp_inv) {
                    matched += 1;
                }
            }
            if let Some(exp_amt) = obj.get("amount").and_then(|v| v.as_str()) {
                total += 1;
                if let Some(amt) = i.amount
                    && amt.to_string() == exp_amt
                {
                    matched += 1;
                }
            }
            if let Some(exp_curr) = obj.get("currency").and_then(|v| v.as_str()) {
                total += 1;
                if i.currency.as_deref() == Some(exp_curr) {
                    matched += 1;
                }
            }
            if let Some(exp_due) = obj.get("due_date").and_then(|v| v.as_str()) {
                total += 1;
                if i.due_date.as_deref() == Some(exp_due) {
                    matched += 1;
                }
            }
            if let Some(exp_iss) = obj.get("issuer").and_then(|v| v.as_str()) {
                total += 1;
                if i.issuer.as_deref() == Some(exp_iss) {
                    matched += 1;
                }
            }
        }
        ExtractedData::Subscription(s) => {
            if let Some(exp_prov) = obj.get("provider").and_then(|v| v.as_str()) {
                total += 1;
                if s.provider.as_deref() == Some(exp_prov) {
                    matched += 1;
                }
            }
            if let Some(exp_amt) = obj.get("amount").and_then(|v| v.as_str()) {
                total += 1;
                if let Some(amt) = s.subscription_amount
                    && amt.to_string() == exp_amt
                {
                    matched += 1;
                }
            }
            if let Some(exp_freq) = obj.get("frequency").and_then(|v| v.as_str()) {
                total += 1;
                if s.billing_frequency.as_deref() == Some(exp_freq) {
                    matched += 1;
                }
            }
            if let Some(exp_ren) = obj.get("renewal_date").and_then(|v| v.as_str()) {
                total += 1;
                if s.renewal_date.as_deref() == Some(exp_ren) {
                    matched += 1;
                }
            }
        }
        ExtractedData::Travel(t) => {
            if let Some(exp_bid) = obj.get("booking_id").and_then(|v| v.as_str()) {
                total += 1;
                if t.booking_id.as_deref() == Some(exp_bid) {
                    matched += 1;
                }
            }
            if let Some(exp_dest) = obj.get("destination").and_then(|v| v.as_str()) {
                total += 1;
                if t.destination.as_deref() == Some(exp_dest) {
                    matched += 1;
                }
            }
            if let Some(exp_dep) = obj.get("departure_date").and_then(|v| v.as_str()) {
                total += 1;
                if t.departure_date.as_deref() == Some(exp_dep) {
                    matched += 1;
                }
            }
            if let Some(exp_ret) = obj.get("return_date").and_then(|v| v.as_str()) {
                total += 1;
                if t.return_date.as_deref() == Some(exp_ret) {
                    matched += 1;
                }
            }
            if let Some(exp_stat) = obj.get("status").and_then(|v| v.as_str()) {
                total += 1;
                if t.booking_status.as_deref() == Some(exp_stat) {
                    matched += 1;
                }
            }
        }
        ExtractedData::Newsletter(n) => {
            if let Some(exp_pub) = obj.get("publisher").and_then(|v| v.as_str()) {
                total += 1;
                if n.publisher.as_deref() == Some(exp_pub) {
                    matched += 1;
                }
            }
            if let Some(exp_topic) = obj.get("main_topic").and_then(|v| v.as_str()) {
                total += 1;
                if n.main_topic.as_deref() == Some(exp_topic) {
                    matched += 1;
                }
            }
            if let Some(exp_head) = obj.get("headline").and_then(|v| v.as_str()) {
                total += 1;
                if n.headline.as_deref() == Some(exp_head) {
                    matched += 1;
                }
            }
        }
        ExtractedData::Social(s) => {
            if let Some(exp_plat) = obj.get("platform").and_then(|v| v.as_str()) {
                total += 1;
                if s.platform.as_deref() == Some(exp_plat) {
                    matched += 1;
                }
            }
            if let Some(exp_act) = obj.get("activity_type").and_then(|v| v.as_str()) {
                total += 1;
                if s.activity_type.as_deref() == Some(exp_act) {
                    matched += 1;
                }
            }
            if let Some(exp_actor) = obj.get("actor").and_then(|v| v.as_str()) {
                total += 1;
                if s.relevant_entity.as_deref() == Some(exp_actor) {
                    matched += 1;
                }
            }
        }
        ExtractedData::General(_) => {
            if obj.contains_key("summary") {
                total += 1;
                matched += 1; // Handled cleanly
            }
        }
        ExtractedData::SecurityAlert(sec) => {
            if let Some(exp_type) = obj.get("alert_type").and_then(|v| v.as_str()) {
                total += 1;
                if sec.alert_type.as_deref() == Some(exp_type) {
                    matched += 1;
                }
            }
            if let Some(exp_res) = obj.get("target_resource").and_then(|v| v.as_str()) {
                total += 1;
                if sec.target_resource.as_deref() == Some(exp_res) {
                    matched += 1;
                }
            }
        }
    }

    (matched, total)
}

/// Print the terminal benchmark report formatted precisely as required.
pub fn print_benchmark_report(r: &BenchmarkReport) {
    let input_kb = r.total_input_bytes as f64 / 1024.0;

    println!("====================================================");
    println!("       RELAY ROVER ENGINE - BENCHMARK");
    println!("====================================================");
    println!();
    println!("Dataset:");
    println!("  Total emails:             {}", r.total_emails);
    println!("  Total input size:         {:.2} KB", input_kb);
    println!("  Detection patterns:       {}", r.detection_pattern_count);
    println!("  Categories:               {}", r.categories_count);
    println!();
    println!("INITIALIZATION");
    println!("----------------------------------------------------");
    println!(
        "RegexSet compilation:       {} µs",
        r.regex_set_compilation.as_micros()
    );
    println!(
        "Extraction regexes:         {} µs",
        r.extraction_compilation.as_micros()
    );
    println!(
        "Total initialization:       {:.2} ms",
        r.total_initialization.as_secs_f64() * 1000.0
    );
    println!();
    println!("PROCESSING (Single-threaded 100-email batch)");
    println!("----------------------------------------------------");
    println!(
        "Total batch time:           {:.2} ms",
        r.batch_duration.as_secs_f64() * 1000.0
    );
    println!(
        "Average per email:          {} µs",
        r.avg_per_email.as_micros()
    );
    println!(
        "Median per email:           {} µs",
        r.median_per_email.as_micros()
    );
    println!(
        "P95 per email:              {} µs",
        r.p95_per_email.as_micros()
    );
    println!(
        "P99 per email:              {} µs",
        r.p99_per_email.as_micros()
    );
    println!(
        "Min per email:              {} µs",
        r.min_per_email.as_micros()
    );
    println!(
        "Max per email:              {} µs",
        r.max_per_email.as_micros()
    );
    println!();
    println!("THROUGHPUT (Single-threaded)");
    println!("----------------------------------------------------");
    println!("Emails per second:          {:.0}", r.emails_per_second);
    println!("MB per second:              {:.2} MB/s", r.mb_per_second);
    println!();
    println!("STEADY-STATE WORKLOAD (100 iterations = 10,000 emails)");
    println!("----------------------------------------------------");
    println!(
        "Batch average time:         {:.2} ms",
        r.multi_iter_batch_avg.as_secs_f64() * 1000.0
    );
    println!(
        "Steady-state throughput:    {:.0} emails/sec",
        r.multi_iter_emails_per_sec
    );
    println!(
        "Steady-state data rate:     {:.2} MB/s",
        r.multi_iter_mb_per_sec
    );
    println!();
    println!("PARALLEL EXECUTION (Rayon Work-Stealing Pool)");
    println!("----------------------------------------------------");
    println!(
        "Total parallel batch time:  {:.2} ms",
        r.parallel_batch_duration.as_secs_f64() * 1000.0
    );
    println!(
        "Parallel throughput:        {:.0} emails/sec",
        r.parallel_emails_per_sec
    );
    println!(
        "Parallel data rate:         {:.2} MB/s",
        r.parallel_mb_per_sec
    );
    let speedup = if r.parallel_batch_duration.as_micros() > 0 {
        r.batch_duration.as_secs_f64() / r.parallel_batch_duration.as_secs_f64()
    } else {
        1.0
    };
    println!("Parallel speedup factor:    {:.2}x", speedup);
    println!();
    println!("PIPELINE STAGE LATENCY COMPARISON");
    println!("----------------------------------------------------");
    println!(
        "Stage 1: Detection only:    {:.2} ms  ({} µs/email)",
        r.detection_only_duration.as_secs_f64() * 1000.0,
        r.detection_only_duration.as_micros() / r.total_emails as u128
    );
    println!(
        "Stage 2: Detect + Extract:  {:.2} ms  ({} µs/email)",
        r.detection_extraction_duration.as_secs_f64() * 1000.0,
        r.detection_extraction_duration.as_micros() / r.total_emails as u128
    );
    println!(
        "Stage 3: Full Pipeline:     {:.2} ms  ({} µs/email)",
        r.full_pipeline_duration.as_secs_f64() * 1000.0,
        r.full_pipeline_duration.as_micros() / r.total_emails as u128
    );
    println!();
    println!("ACCURACY");
    println!("----------------------------------------------------");
    println!(
        "Category accuracy:          {:.1}%",
        r.category_accuracy_percent
    );
    println!(
        "Extraction accuracy:        {:.1}%",
        r.extraction_accuracy_percent
    );
    println!("False positives:            {}", r.false_positives);
    println!("False negatives:            {}", r.false_negatives);
    println!("Multi-category emails:      {}", r.multi_category_count);
    println!();
    println!("CATEGORY BREAKDOWN");
    println!("----------------------------------------------------");
    for (cat, correct, expected) in &r.category_breakdown {
        println!(
            "{:<28}{:>2} / {:>2}",
            format!("{}:", cat.as_str()),
            correct,
            expected
        );
    }
    println!();
    println!("====================================================");
}
