use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractedOtp {
    pub code: String,
    pub service: Option<String>,
    pub expires_in: Option<String>,
}

/// OTP / Verification code extractor with security against false positives.
pub struct OtpExtractor {
    code_pattern_1: Regex,
    code_pattern_2: Regex,
    code_pattern_3: Regex,
    code_pattern_4: Regex,
    expiry_pattern: Regex,
    service_pattern: Regex,
}

impl OtpExtractor {
    pub fn new() -> Result<Self, regex::Error> {
        // Pattern 1: "verification code is 482913", "security code 918204", "OTP is 334812", "Wire Authorization: 334812"
        let code_pattern_1 = Regex::new(
            r"(?i)\b(?:verification code|security code|login code|otp|passcode|pin|authorization(?:\s+code)?)\s*(?:is|:|\b)\s*([0-9]{4,8})\b",
        )?;

        // Pattern 2: "code: 772019", "OTP: 649201", "PIN: 554109"
        let code_pattern_2 = Regex::new(r"(?i)\b(?:code|otp|pin)\s*[:#]\s*([0-9]{4,8})\b")?;

        // Pattern 3: "820155 is your account recovery code"
        let code_pattern_3 = Regex::new(r"(?i)\b([0-9]{4,8})\s+is\s+your\b")?;

        // Pattern 4: "Enter security code 491028", "Enter verification code 772019", "Enter OTP 491028"
        let code_pattern_4 = Regex::new(
            r"(?i)\benter\s+(?:the\s+)?(?:security code|verification code|otp|authorization code|pin)?\s*([0-9]{4,8})\b",
        )?;

        // Expiry pattern: "expires in 10 minutes", "valid for 5 minutes", "active for 20 minutes"
        let expiry_pattern = Regex::new(
            r"(?i)\b(?:valid for|expires in|active for|expire in)\s+([0-9]+\s+(?:minutes?|mins?|hours?|seconds?))\b",
        )?;

        // Service pattern: "for Acme Portal is", "sign in to your VaultCloud", "access to GamingNetwork"
        let service_pattern = Regex::new(
            r"(?i)\b(?:for|to your|on|to access|access to|sign in to your|withdrawal from|session on)\s+([A-Za-z0-9]{3,25}(?:\s+[A-Za-z0-9]{3,20})?)\b",
        )?;

        Ok(Self {
            code_pattern_1,
            code_pattern_2,
            code_pattern_3,
            code_pattern_4,
            expiry_pattern,
            service_pattern,
        })
    }

    /// Extract OTP code, service, and expiry details from subject and body.
    pub fn extract(&self, subject: &str, body: &str) -> Option<ExtractedOtp> {
        let text = format!("{subject}\n{body}");

        let code = self
            .code_pattern_1
            .captures(&text)
            .or_else(|| self.code_pattern_2.captures(&text))
            .or_else(|| self.code_pattern_3.captures(&text))
            .or_else(|| self.code_pattern_4.captures(&text))
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()));

        code.map(|c| {
            let expires_in = self
                .expiry_pattern
                .captures(&text)
                .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()));

            let mut service = self
                .service_pattern
                .captures(&text)
                .and_then(|caps| caps.get(1).map(|m| m.as_str().trim().to_string()));

            // Filter false positive captures and strip trailing words like "account"
            if let Some(s) = service.as_mut() {
                if s.ends_with(" account") {
                    *s = s[..s.len() - 8].trim().to_string();
                }
                if s.contains("corporate resources")
                    || s.contains("Wire Authorization")
                    || s == "Linux"
                    || s == "security"
                    || s == "creating"
                    || s.starts_with("creating ")
                {
                    service = None;
                }
            }

            ExtractedOtp {
                code: c,
                service,
                expires_in,
            }
        })
    }
}
