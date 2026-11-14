use crate::extractors::{AmountExtractor, DateExtractor, IdentityExtractor};
use crate::models::{Category, EmailRecord, ExtractedData, SubscriptionExtraction};
use crate::preview::generate_preview;
use regex::Regex;

pub struct SubscriptionFilter {
    amount_ext: AmountExtractor,
    date_ext: DateExtractor,
    id_ext: IdentityExtractor,
    frequency_re: Regex,
    cancel_re: Regex,
}

impl SubscriptionFilter {
    pub fn new() -> Result<Self, regex::Error> {
        let amount_ext = AmountExtractor::new()?;
        let date_ext = DateExtractor::new()?;
        let id_ext = IdentityExtractor::new()?;

        let frequency_re = Regex::new(r"(?i)\b(monthly|annual|yearly|weekly|quarterly)\b")?;

        let cancel_re = Regex::new(
            r"(?i)\b((?:cancel|submit\s+notice|disable|to\s+cancel|manage\s+or\s+cancel)\s+[^\n.,;!]+?(?:anytime|dashboard|settings|prior|renewal\s+date|renewal)|cancel\s+anytime\s+[^\n.,;!]+)\b",
        )?;

        Ok(Self {
            amount_ext,
            date_ext,
            id_ext,
            frequency_re,
            cancel_re,
        })
    }
}

pub const SUBSCRIPTION_DETECTION_PATTERNS: &[&str] = &[
    r"(?i)\b(?:subscription\s+(?:renews|renewal|monthly|annual|plan)|membership\s+auto-renewal|domain\s+auto-renewal|auto-renew\s+notice)\b",
    r"(?i)\b(?:monthly\s+renewal\s+confirmation|plan\s+renewal\s+notice|subscription\s+will\s+auto-renew)\b",
    r"(?i)\b(?:(?:monthly|annual|yearly)\s+subscription|annual\s+plan\s+renewal|subscription\s+to\s+[A-Za-z0-9\s]+renews)\b",
];

impl SubscriptionFilter {
    pub fn category(&self) -> Category {
        Category::Subscription
    }

    pub fn detection_patterns(&self) -> &[&'static str] {
        SUBSCRIPTION_DETECTION_PATTERNS
    }

    pub fn extract(&self, email: &EmailRecord) -> ExtractedData {
        let text = format!("{}\n{}", email.subject, email.body);

        let provider = self
            .id_ext
            .extract_provider(&text)
            .or_else(|| Some(email.sender_name.clone()));

        let amount_info = self.amount_ext.extract(&text);
        let subscription_amount = amount_info.map(|a| a.value);

        let billing_frequency = self
            .frequency_re
            .captures(&text)
            .and_then(|c| c.get(1).map(|m| m.as_str().to_lowercase()));

        let renewal_date = self.date_ext.extract(&email.body);

        let mut cancellation_info = self
            .cancel_re
            .captures(&email.body)
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()));

        if let Some(c) = cancellation_info.as_mut() {
            if c.starts_with("manage or ") {
                *c = c[10..].to_string();
            }
            if c.starts_with("To cancel without penalty, ") {
                *c = c[27..].to_string();
            }
        }

        let extraction = SubscriptionExtraction {
            provider,
            subscription_amount,
            billing_frequency,
            renewal_date,
            cancellation_info,
            raw_evidence: Some(email.subject.clone()),
        };

        ExtractedData::Subscription(extraction)
    }

    pub fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String {
        generate_preview(Category::Subscription, data, &email.subject, &email.body)
    }
}
