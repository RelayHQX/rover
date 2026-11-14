use crate::filters::EmailFilter;
use crate::models::{
    Category, ConfidenceFactors, EmailClassification, EmailRecord, ExtractedData, GeneralCategory,
    Importance, ProcessedEmail, SecurityClassification,
};
use crate::registry::{RegexRegistry, RegistryBuilder};
use rayon::prelude::*;
use std::sync::Arc;

/// Execution mode for batch email processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExecutionMode {
    /// Single-threaded sequential processing (Default)
    #[default]
    SingleThreaded,
    /// Multi-threaded parallel processing using Rayon work-stealing threadpool
    MultiThreaded { num_threads: usize },
}

/// Builder for creating and customizing an `EmailEngine` (Rover).
pub struct EngineBuilder {
    registry_builder: RegistryBuilder,
    execution_mode: ExecutionMode,
}

impl Default for EngineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl EngineBuilder {
    /// Create a new builder with default single-threaded execution mode.
    #[must_use]
    pub fn new() -> Self {
        Self {
            registry_builder: RegistryBuilder::new(),
            execution_mode: ExecutionMode::SingleThreaded,
        }
    }

    /// Configure the engine for single-threaded execution (default).
    #[must_use]
    pub fn with_single_threaded(mut self) -> Self {
        self.execution_mode = ExecutionMode::SingleThreaded;
        self
    }

    /// Configure the engine for multi-threaded execution with a specified or default thread count.
    /// If `threads` is `None` or 0, it defaults to the available system CPU cores.
    #[must_use]
    pub fn with_multi_threaded(mut self, threads: Option<usize>) -> Self {
        let num_threads = threads.filter(|&t| t > 0).unwrap_or_else(|| {
            std::thread::available_parallelism()
                .map(|p| p.get())
                .unwrap_or(4)
        });
        self.execution_mode = ExecutionMode::MultiThreaded { num_threads };
        self
    }

    /// Configure threading mode dynamically:
    /// - `multithread = false`: Uses single-threaded mode (ignoring `threads`).
    /// - `multithread = true`: Uses multi-threaded mode with `threads` workers (or CPU cores if `None`).
    #[must_use]
    pub fn with_threading(self, multithread: bool, threads: Option<usize>) -> Self {
        if multithread {
            self.with_multi_threaded(threads)
        } else {
            self.with_single_threaded()
        }
    }

    /// Override or add a custom category filter implementation.
    #[must_use]
    pub fn with_filter<F: EmailFilter + 'static>(mut self, filter: F) -> Self {
        self.registry_builder = self.registry_builder.register_filter(filter);
        self
    }

    /// Add an extra detection regex pattern mapped to a category.
    #[must_use]
    pub fn with_detection_pattern(mut self, pattern: &'static str, category: Category) -> Self {
        self.registry_builder = self
            .registry_builder
            .add_detection_pattern(pattern, category);
        self
    }

    /// Build the compiled `EmailEngine` (Rover).
    pub fn build(self) -> Result<EmailEngine, anyhow::Error> {
        let registry = self.registry_builder.build()?;
        let (thread_pool, execution_mode) = match self.execution_mode {
            ExecutionMode::SingleThreaded => (None, ExecutionMode::SingleThreaded),
            ExecutionMode::MultiThreaded { num_threads } => {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(num_threads)
                    .build()
                    .map_err(|e| {
                        anyhow::anyhow!(
                            "Failed to build threadpool with {} threads: {}",
                            num_threads,
                            e
                        )
                    })?;
                (
                    Some(Arc::new(pool)),
                    ExecutionMode::MultiThreaded { num_threads },
                )
            }
        };

        Ok(EmailEngine {
            registry,
            execution_mode,
            thread_pool,
        })
    }
}

/// Deterministic, high-performance email intelligence engine (Relay Rover).
pub struct EmailEngine {
    registry: RegexRegistry,
    execution_mode: ExecutionMode,
    thread_pool: Option<Arc<rayon::ThreadPool>>,
}

/// Official Relay Rover Engine aliases
pub type Rover = EmailEngine;
pub type RoverEngine = EmailEngine;
pub type RoverBuilder = EngineBuilder;

impl EmailEngine {
    /// Return an `EngineBuilder` to configure custom filters or regex rules.
    #[must_use]
    pub fn builder() -> EngineBuilder {
        EngineBuilder::new()
    }

    /// Initialize default engine with all standard compiled patterns.
    pub fn new() -> Result<Self, anyhow::Error> {
        Self::builder().build()
    }

    /// Access the underlying compiled regex registry.
    #[must_use]
    pub fn registry(&self) -> &RegexRegistry {
        &self.registry
    }

    /// Helper to produce a combined search slice for category detection.
    #[inline]
    fn make_search_text(email: &EmailRecord) -> String {
        let mut text = String::with_capacity(email.subject.len() + email.body.len() + 1);
        text.push_str(&email.subject);
        text.push('\n');
        text.push_str(&email.body);
        text
    }

    /// Step 1: Detect matching categories using the single-pass RegexSet.
    pub fn detect_categories(&self, email: &EmailRecord) -> Vec<Category> {
        let search_text = Self::make_search_text(email);
        self.registry.detect_categories(&search_text)
    }

    /// Step 2: Detect category and execute category-specific extraction.
    pub fn detect_and_extract(&self, email: &EmailRecord) -> (Category, ExtractedData) {
        let matched = self.detect_categories(email);
        let primary = matched.first().copied().unwrap_or(Category::General);

        let data = if let Some(filter) = self.registry.get_filter(primary) {
            filter.extract(email)
        } else {
            crate::filters::GeneralFilter::new().extract(email)
        };

        (primary, data)
    }

    /// Step 3: Full end-to-end processing: detection, extraction, and preview generation.
    pub fn process(&self, email: &EmailRecord) -> ProcessedEmail {
        let search_text = Self::make_search_text(email);
        let matched_categories = self.registry.detect_categories(&search_text);
        let primary_category = matched_categories
            .first()
            .copied()
            .unwrap_or(Category::General);

        let filter = self.registry.get_filter(primary_category);

        let extraction = match filter {
            Some(f) => f.extract(email),
            None => crate::filters::GeneralFilter::new().extract(email),
        };

        let preview = match filter {
            Some(f) => f.generate_preview(email, &extraction),
            None => crate::preview::generate_preview(
                primary_category,
                &extraction,
                &email.subject,
                &email.body,
            ),
        };

        let (classification, timeline_key) = Self::classify_email(
            primary_category,
            &matched_categories,
            &extraction,
            email,
        );

        ProcessedEmail {
            id: email.id.clone(),
            primary_category,
            matched_categories,
            classification,
            extraction,
            preview,
            timeline_key,
        }
    }

    /// Compute rich classification, confidence scoring, and index timeline keys.
    fn classify_email(
        primary_category: Category,
        matched_categories: &[Category],
        extraction: &ExtractedData,
        email: &EmailRecord,
    ) -> (EmailClassification, Option<String>) {
        let (general_cat, importance, is_alert, sec_class, confidence, specific_reason) =
            match primary_category {
                Category::Otp => {
                    let sec = if email.subject.to_lowercase().contains("password")
                        || email.body.to_lowercase().contains("password reset")
                    {
                        SecurityClassification::PasswordReset
                    } else {
                        SecurityClassification::Authentication
                    };
                    (
                        GeneralCategory::Primary,
                        Importance::Urgent,
                        true,
                        sec,
                        0.99,
                        "High-precision verification code extraction",
                    )
                }
                Category::SecurityAlert => {
                    let sec = if email.subject.to_lowercase().contains("build failed")
                        || email.body.to_lowercase().contains("build failed")
                    {
                        SecurityClassification::BuildFailure
                    } else if email.subject.to_lowercase().contains("password") {
                        SecurityClassification::PasswordReset
                    } else {
                        SecurityClassification::SecurityAlert
                    };
                    (
                        GeneralCategory::Updates,
                        Importance::Urgent,
                        true,
                        sec,
                        0.98,
                        "Security event / system status pattern matched",
                    )
                }
                Category::Payment => (
                    GeneralCategory::Updates,
                    Importance::High,
                    false,
                    SecurityClassification::None,
                    0.96,
                    "Deterministic monetary transaction and payment entity detected",
                ),
                Category::Invoice => (
                    GeneralCategory::Updates,
                    Importance::High,
                    false,
                    SecurityClassification::None,
                    0.95,
                    "Invoice number and payable amount detected",
                ),
                Category::Shopping => (
                    GeneralCategory::Updates,
                    Importance::Normal,
                    false,
                    SecurityClassification::None,
                    0.95,
                    "Commerce order and shipment tracking entities detected",
                ),
                Category::Job => {
                    let is_alert_email = match extraction {
                        ExtractedData::Job(j) => j.is_job_alert,
                        _ => false,
                    };
                    let (gen_cat, imp) = if is_alert_email {
                        (GeneralCategory::Updates, Importance::Normal)
                    } else if email.subject.to_lowercase().contains("offer")
                        || email.subject.to_lowercase().contains("interview")
                    {
                        (GeneralCategory::Primary, Importance::Urgent)
                    } else {
                        (GeneralCategory::Primary, Importance::Normal)
                    };
                    (
                        gen_cat,
                        imp,
                        false,
                        SecurityClassification::None,
                        0.93,
                        "Job opportunity and hiring workflow pattern matched",
                    )
                }
                Category::Social => (
                    GeneralCategory::Social,
                    Importance::Normal,
                    false,
                    SecurityClassification::None,
                    0.92,
                    "Social network activity and notification pattern matched",
                ),
                Category::Newsletter => (
                    GeneralCategory::Promotions,
                    Importance::Low,
                    false,
                    SecurityClassification::None,
                    0.90,
                    "Digest / newsletter publication format detected",
                ),
                Category::Subscription => (
                    GeneralCategory::Updates,
                    Importance::Normal,
                    false,
                    SecurityClassification::None,
                    0.93,
                    "Recurring subscription schedule pattern matched",
                ),
                Category::Travel => (
                    GeneralCategory::Updates,
                    Importance::High,
                    false,
                    SecurityClassification::None,
                    0.94,
                    "Travel itinerary and reservation pattern matched",
                ),
                Category::General => {
                    let is_promo = email.body.to_lowercase().contains("unsubscribe")
                        && (email.body.to_lowercase().contains("% off")
                            || email.subject.to_lowercase().contains("deal")
                            || email.subject.to_lowercase().contains("sale"));
                    let gen_cat = if is_promo {
                        GeneralCategory::Promotions
                    } else {
                        GeneralCategory::Primary
                    };
                    (
                        gen_cat,
                        Importance::Normal,
                        false,
                        SecurityClassification::None,
                        0.75,
                        "General unstructured correspondence",
                    )
                }
            };

        let has_structured_entities = match extraction {
            ExtractedData::Payment(p) => p.amount.is_some() || p.transaction_id.is_some(),
            ExtractedData::Otp(o) => o.otp_code.is_some(),
            ExtractedData::Job(j) => j.position.is_some() || j.company.is_some(),
            ExtractedData::Shopping(s) => {
                s.order_id.is_some() || s.product.is_some() || s.amount.is_some()
            }
            ExtractedData::Invoice(i) => i.invoice_number.is_some() || i.amount.is_some(),
            ExtractedData::Subscription(s) => s.provider.is_some(),
            ExtractedData::Travel(t) => t.booking_id.is_some() || t.destination.is_some(),
            ExtractedData::Newsletter(n) => n.publisher.is_some() || n.headline.is_some(),
            ExtractedData::Social(s) => s.platform.is_some() || s.actor.is_some(),
            ExtractedData::SecurityAlert(s) => s.alert_type.is_some(),
            ExtractedData::General(_) => false,
        };

        let factors = ConfidenceFactors {
            authenticated_sender: !email.sender.is_empty(),
            has_structured_entities,
            pattern_specificity: specific_reason.to_string(),
            subject_body_aligned: true,
        };

        let secondary_purposes: Vec<Category> = matched_categories
            .iter()
            .copied()
            .filter(|&c| c != primary_category)
            .collect();

        let classification = EmailClassification {
            general_category: general_cat,
            purpose: primary_category,
            importance,
            is_alert,
            security: sec_class,
            confidence,
            factors,
            secondary_purposes,
        };

        let timeline_key = extraction.timeline_key();

        (classification, timeline_key)
    }

    /// Process any arbitrary email given just its subject and body.
    pub fn process_raw(&self, subject: &str, body: &str) -> ProcessedEmail {
        let record = EmailRecord {
            id: "msg_custom".to_string(),
            sender: "unknown@relay.mail".to_string(),
            sender_name: "Sender".to_string(),
            subject: subject.to_string(),
            received_at: "2026-11-05T00:00:00Z".to_string(),
            body: body.to_string(),
            expected_category: Category::General,
            expected_extraction: serde_json::Value::Null,
        };
        self.process(&record)
    }

    /// Process any arbitrary single raw email string (parses subject if present as first line).
    pub fn process_text(&self, email_text: &str) -> ProcessedEmail {
        let mut lines = email_text.lines();
        let first_line = lines.next().unwrap_or("");
        let (subject, body) = if let Some(stripped) = first_line.strip_prefix("Subject:") {
            (stripped.trim(), lines.collect::<Vec<_>>().join("\n"))
        } else {
            (first_line.trim(), lines.collect::<Vec<_>>().join("\n"))
        };
        self.process_raw(subject, &body)
    }

    /// Get the execution mode of the engine (Single-threaded or Multi-threaded).
    #[must_use]
    pub fn execution_mode(&self) -> ExecutionMode {
        self.execution_mode
    }

    /// Returns true if the engine is running in multi-threaded mode.
    #[must_use]
    pub fn is_multi_threaded(&self) -> bool {
        matches!(self.execution_mode, ExecutionMode::MultiThreaded { .. })
    }

    /// Returns the number of worker threads configured (1 for single-threaded).
    #[must_use]
    pub fn num_threads(&self) -> usize {
        match self.execution_mode {
            ExecutionMode::SingleThreaded => 1,
            ExecutionMode::MultiThreaded { num_threads } => num_threads,
        }
    }

    /// Process a batch of emails using the configured execution mode (single-threaded by default).
    pub fn process_batch(&self, emails: &[EmailRecord]) -> Vec<ProcessedEmail> {
        if let Some(pool) = &self.thread_pool {
            pool.install(|| emails.par_iter().map(|email| self.process(email)).collect())
        } else {
            emails.iter().map(|email| self.process(email)).collect()
        }
    }

    /// Explicitly process a batch of emails sequentially (single-threaded).
    pub fn process_batch_single_threaded(&self, emails: &[EmailRecord]) -> Vec<ProcessedEmail> {
        emails.iter().map(|email| self.process(email)).collect()
    }

    /// Explicitly process a batch of emails in parallel using the engine's pool or Rayon global pool.
    pub fn process_batch_parallel(&self, emails: &[EmailRecord]) -> Vec<ProcessedEmail> {
        if let Some(pool) = &self.thread_pool {
            pool.install(|| emails.par_iter().map(|email| self.process(email)).collect())
        } else {
            emails.par_iter().map(|email| self.process(email)).collect()
        }
    }
}
