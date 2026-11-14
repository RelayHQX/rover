use regex::Regex;
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractedAmount {
    pub value: Decimal,
    pub currency: String,
    pub raw: String,
}

/// Specialized amount extractor with compiled regex patterns.
pub struct AmountExtractor {
    currency_first: Regex,
    currency_after: Regex,
}

impl AmountExtractor {
    pub fn new() -> Result<Self, regex::Error> {
        // Matches e.g. INR 2,500.00, $49.99, €1200.00, £ 150.50, ₹ 850, ¥15000, CAD 160,000
        // Notice we do NOT use \b before symbol currencies since $, €, ₹, £, ¥ are \W characters!
        let currency_first = Regex::new(
            r"(?i)(?:^|[^\w])(?P<curr>INR|USD|EUR|GBP|JPY|CAD|AUD|₹|\$|€|£|¥)\s*(?P<val>\d{1,3}(?:,\d{3})*(?:\.\d{1,2})?|\d+(?:\.\d{1,2})?)\b",
        )?;

        // Matches e.g. 2500.00 INR, 49.99 USD, 1200.00 EUR, 150.50 GBP
        let currency_after = Regex::new(
            r"(?i)\b(?P<val>\d{1,3}(?:,\d{3})*(?:\.\d{1,2})?|\d+(?:\.\d{1,2})?)\s*(?P<curr>INR|USD|EUR|GBP|JPY|CAD|AUD)\b",
        )?;

        Ok(Self {
            currency_first,
            currency_after,
        })
    }

    /// Extract amount and currency from text.
    pub fn extract(&self, text: &str) -> Option<ExtractedAmount> {
        // Try currency first (e.g. ₹2,500 or $49.99)
        if let Some(caps) = self.currency_first.captures(text)
            && let (Some(curr), Some(val)) = (caps.name("curr"), caps.name("val"))
        {
            let clean_val = val.as_str().replace(',', "");
            if let Ok(decimal_val) = Decimal::from_str(&clean_val) {
                return Some(ExtractedAmount {
                    value: decimal_val,
                    currency: normalize_currency(curr.as_str()),
                    raw: caps.get(0).map_or("", |m| m.as_str()).to_string(),
                });
            }
        }

        // Try currency after (e.g. 2500.00 INR)
        if let Some(caps) = self.currency_after.captures(text)
            && let (Some(curr), Some(val)) = (caps.name("curr"), caps.name("val"))
        {
            let clean_val = val.as_str().replace(',', "");
            if let Ok(decimal_val) = Decimal::from_str(&clean_val) {
                return Some(ExtractedAmount {
                    value: decimal_val,
                    currency: normalize_currency(curr.as_str()),
                    raw: caps.get(0).map_or("", |m| m.as_str()).to_string(),
                });
            }
        }

        None
    }
}

/// Normalize symbols and lowercase codes to standard ISO currency codes.
pub fn normalize_currency(raw: &str) -> String {
    match raw.trim().to_uppercase().as_str() {
        "$" | "USD" => "USD".to_string(),
        "€" | "EUR" => "EUR".to_string(),
        "£" | "GBP" => "GBP".to_string(),
        "₹" | "INR" => "INR".to_string(),
        "¥" | "JPY" => "JPY".to_string(),
        "CAD" => "CAD".to_string(),
        "AUD" => "AUD".to_string(),
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_amounts() {
        let extractor = AmountExtractor::new().expect("Valid regex");

        let res = extractor.extract("Your account was debited INR 2,500.00 today");
        assert!(res.is_some());
        let res = res.unwrap();
        assert_eq!(res.value, Decimal::from_str("2500.00").unwrap());
        assert_eq!(res.currency, "INR");

        let res2 = extractor.extract("Total: $49.99 paid");
        assert_eq!(res2.unwrap().value, Decimal::from_str("49.99").unwrap());

        let res3 = extractor.extract("Paid €1200.00 to merchant");
        assert_eq!(res3.unwrap().value, Decimal::from_str("1200.00").unwrap());
    }
}
