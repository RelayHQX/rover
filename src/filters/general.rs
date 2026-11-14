use crate::models::{Category, EmailRecord, ExtractedData, GeneralExtraction};
use crate::preview::{clean_excerpt, generate_preview};

pub struct GeneralFilter;

impl GeneralFilter {
    pub const fn new() -> Self {
        Self
    }
}

impl Default for GeneralFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl GeneralFilter {
    pub const fn category(&self) -> Category {
        Category::General
    }

    pub const fn detection_patterns(&self) -> &[&'static str] {
        &[]
    }

    pub fn extract(&self, email: &EmailRecord) -> ExtractedData {
        let clean = clean_excerpt(&email.body);
        let summary = if clean.is_empty() {
            Some(clean_excerpt(&email.subject))
        } else {
            // First sentence or up to 60 characters
            let sentence = clean.split(&['.', '?', '!'][..]).next().unwrap_or(&clean);
            Some(sentence.trim().to_string())
        };

        let extraction = GeneralExtraction {
            summary,
            sender_display: Some(email.sender_name.clone()),
            raw_evidence: Some(email.subject.clone()),
        };

        ExtractedData::General(extraction)
    }

    pub fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String {
        generate_preview(Category::General, data, &email.subject, &email.body)
    }
}
