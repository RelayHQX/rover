use crate::models::{Category, EmailRecord, ExtractedData, NewsletterExtraction};
use crate::preview::generate_preview;
use regex::Regex;

pub struct NewsletterFilter {
    publisher_re: Regex,
    curator_re: Regex,
    topic_re: Regex,
    headline_re: Regex,
}

impl NewsletterFilter {
    pub fn new() -> Result<Self, regex::Error> {
        let publisher_re = Regex::new(
            r"(?i)\b(?:Publisher)\s*[:]?\s*([A-Za-z0-9\s.'\-]{3,35}?)(?:\.|\,|\;|\n|Main topic|$)",
        )?;

        let curator_re = Regex::new(
            r"(?i)\b(?:curated by)\s+([A-Za-z0-9\s.'\-]{3,35}?)(?:\.|\,|\;|\n|Main topic|$)",
        )?;

        let topic_re = Regex::new(
            r"(?i)\b(?:Main topic|Topic)\s*[:]?\s*([A-Za-z0-9\s.'\-]{3,60}?)(?:\.|\,|\;|\n|Headline|$)",
        )?;

        let headline_re = Regex::new(
            r"(?i)\b(?:Headline|Lead story)\s*[:]?\s*([A-Za-z0-9\s.'\-:]{5,70}?)(?:\.|\,|\;|\n|\bRead\b|\bUnsubscribe\b|$)",
        )?;

        Ok(Self {
            publisher_re,
            curator_re,
            topic_re,
            headline_re,
        })
    }
}

pub const NEWSLETTER_DETECTION_PATTERNS: &[&str] = &[
    r"(?i)\b(?:weekly\s+digest|newsletter\s+issue|morning\s+market\s+brief|insights\s+dispatch|engineering\s+digest|cyber\s+bulletin)\b",
    r"(?i)\b(?:startup\s+rounds|biotech\s+weekly|devops\s+frontier|data\s+lakeview|ux\s+craft\s+letters)\b",
];

impl NewsletterFilter {
    pub fn category(&self) -> Category {
        Category::Newsletter
    }

    pub fn detection_patterns(&self) -> &[&'static str] {
        NEWSLETTER_DETECTION_PATTERNS
    }

    pub fn extract(&self, email: &EmailRecord) -> ExtractedData {
        let publisher = self
            .publisher_re
            .captures(&email.body)
            .or_else(|| self.curator_re.captures(&email.body))
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
            .or_else(|| Some(email.sender_name.clone()));

        let mut main_topic = self
            .topic_re
            .captures(&email.body)
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()));

        if let Some(t) = main_topic.as_mut()
            && let Some(pos) = t.find(" and ")
        {
            *t = t[..pos].trim().to_string();
        }

        let headline = self
            .headline_re
            .captures(&email.body)
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
            .or_else(|| {
                email
                    .subject
                    .find(':')
                    .map(|pos| email.subject[pos + 1..].trim().to_string())
            });

        let extraction = NewsletterExtraction {
            publisher,
            main_topic,
            headline,
            raw_evidence: Some(email.subject.clone()),
        };

        ExtractedData::Newsletter(extraction)
    }

    pub fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String {
        generate_preview(Category::Newsletter, data, &email.subject, &email.body)
    }
}
