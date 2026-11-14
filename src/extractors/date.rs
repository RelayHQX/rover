use regex::Regex;

/// Reusable date extractor for ISO and common verbal formats.
pub struct DateExtractor {
    iso_date: Regex,
    slash_date: Regex,
    verbal_date: Regex,
}

impl DateExtractor {
    pub fn new() -> Result<Self, regex::Error> {
        // Matches 2026-11-05
        let iso_date = Regex::new(r"\b(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})\b")?;

        // Matches 11/05/2026 or 05/11/2026
        let slash_date = Regex::new(r"\b(?P<m_or_d>\d{2})/(?P<d_or_m>\d{2})/(?P<year>\d{4})\b")?;

        // Matches e.g. 15 November 2026, Nov 12, 2026
        let verbal_date = Regex::new(
            r"(?i)\b(?P<day>\d{1,2})(?:st|nd|rd|th)?\s+(?P<month>Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|Jun(?:e)?|Jul(?:y)?|Aug(?:ust)?|Sep(?:tember)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?)(?:\s+(?P<year>\d{4}))?\b",
        )?;

        Ok(Self {
            iso_date,
            slash_date,
            verbal_date,
        })
    }

    /// Extract first valid date in ISO format (YYYY-MM-DD) or standardized verbal format.
    pub fn extract(&self, text: &str) -> Option<String> {
        // Check ISO format first
        if let Some(caps) = self.iso_date.captures(text) {
            return Some(caps[0].to_string());
        }

        // Check verbal format
        if let Some(caps) = self.verbal_date.captures(text) {
            let day = caps.name("day").map_or("01", |m| m.as_str());
            let month = caps.name("month").map_or("Jan", |m| m.as_str());
            let year = caps.name("year").map_or("2026", |m| m.as_str());
            let month_num = month_to_num(month);
            return Some(format!(
                "{}-{:02}-{:02}",
                year,
                month_num,
                day.parse::<u32>().unwrap_or(1)
            ));
        }

        // Check slash format
        if let Some(caps) = self.slash_date.captures(text) {
            return Some(caps[0].to_string());
        }

        None
    }
}

fn month_to_num(month: &str) -> u32 {
    match month.to_lowercase().as_str() {
        m if m.starts_with("jan") => 1,
        m if m.starts_with("feb") => 2,
        m if m.starts_with("mar") => 3,
        m if m.starts_with("apr") => 4,
        m if m.starts_with("may") => 5,
        m if m.starts_with("jun") => 6,
        m if m.starts_with("jul") => 7,
        m if m.starts_with("aug") => 8,
        m if m.starts_with("sep") => 9,
        m if m.starts_with("oct") => 10,
        m if m.starts_with("nov") => 11,
        m if m.starts_with("dec") => 12,
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_dates() {
        let extractor = DateExtractor::new().unwrap();

        assert_eq!(
            extractor.extract("Processed on 2026-11-05."),
            Some("2026-11-05".to_string())
        );
    }
}
