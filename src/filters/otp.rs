use crate::extractors::OtpExtractor;
use crate::models::{Category, EmailRecord, ExtractedData, OtpExtraction};
use crate::preview::generate_preview;

pub struct OtpFilter {
    otp_ext: OtpExtractor,
    action_url_re: regex::Regex,
    expiry_within_re: regex::Regex,
}

impl OtpFilter {
    pub fn new() -> Result<Self, regex::Error> {
        let otp_ext = OtpExtractor::new()?;
        let action_url_re = regex::Regex::new(
            r"(?i)(https?://[^\s\x22<>]+(?:verify|confirm|token|account|auth|support)[^\s\x22<>]*)",
        )?;
        let expiry_within_re = regex::Regex::new(
            r"(?i)\b(?:within|expires\s+in|valid\s+for)\s+(\d+\s+(?:hours?|minutes?|mins?|days?))\b",
        )?;
        Ok(Self {
            otp_ext,
            action_url_re,
            expiry_within_re,
        })
    }
}

pub const OTP_DETECTION_PATTERNS: &[&str] = &[
    r"(?i)\b(?:verification\s+code|one-time\s+password|security\s+code|login\s+code|withdrawal\s+otp|recovery\s+code|temporary\s+passcode|authentication\s+code|verification\s+pin)\b",
    r"(?i)\b(?:your\s+otp\s+is|otp\s+for\s+wire|your\s+verification\s+pin|enter\s+pin\s+\d{4,8})\b",
    r"(?i)\b(?:account\s+creation|verify\s+(?:your\s+)?e-?mail\s+address|complete\s+(?:your\s+)?registration|email\s+address\s+verification|confirm\s+your\s+account)\b",
];

impl OtpFilter {
    pub fn category(&self) -> Category {
        Category::Otp
    }

    pub fn detection_patterns(&self) -> &[&'static str] {
        OTP_DETECTION_PATTERNS
    }

    pub fn extract(&self, email: &EmailRecord) -> ExtractedData {
        let extracted = self.otp_ext.extract(&email.subject, &email.body);

        let (code, service, expires_in) = match extracted {
            Some(o) => (Some(o.code), o.service, o.expires_in),
            None => (None, None, None),
        };

        // Fallback for service name if not found in body text: use sender_name or brand from subject
        let service_name = service.or_else(|| {
            let name = &email.sender_name;
            if !name.trim().is_empty() && !name.contains('@') {
                let cleaned = name.split_whitespace().next().unwrap_or(name).to_string();
                return Some(cleaned);
            }
            // Check subject for "[Brand] Account" or "Brand Account" or "[0313] Nintendo Account"
            if let Some(pos) = email.subject.find(" Account") {
                let prefix = &email.subject[..pos];
                let brand = prefix
                    .split(['[', ']', ' ', ':'])
                    .rfind(|s| !s.is_empty() && !s.chars().all(|ch| ch.is_ascii_digit()));
                if let Some(b) = brand {
                    return Some(b.to_string());
                }
            }
            if !name.trim().is_empty() {
                let cleaned = name.split_whitespace().next().unwrap_or(name).to_string();
                return Some(cleaned);
            }
            None
        });

        let text = format!("{}\n{}", email.subject, email.body);

        let final_expires_in = expires_in.or_else(|| {
            self.expiry_within_re
                .captures(&text)
                .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
        });

        let lower = text.to_lowercase();
        let auth_purpose = if lower.contains("account creation") || lower.contains("registration") {
            Some("Account Registration".to_string())
        } else if lower.contains("verification") || lower.contains("verify") {
            Some("Email Verification".to_string())
        } else if lower.contains("login") || lower.contains("sign in") {
            Some("Sign In Authorization".to_string())
        } else if lower.contains("password reset") || lower.contains("recovery") {
            Some("Password Reset".to_string())
        } else {
            Some("Two-Factor Authentication".to_string())
        };

        let action_url = self
            .action_url_re
            .captures(&email.body)
            .and_then(|c| c.get(1).map(|m| m.as_str().to_string()));

        let extraction = OtpExtraction {
            otp_code: code,
            service_name,
            expiration_time: final_expires_in,
            auth_purpose,
            action_url,
            raw_evidence: Some(email.subject.clone()),
        };

        ExtractedData::Otp(extraction)
    }

    pub fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String {
        generate_preview(Category::Otp, data, &email.subject, &email.body)
    }
}
