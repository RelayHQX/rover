use crate::models::{Category, EmailRecord, ExtractedData, SecurityAlertExtraction};
use crate::preview::generate_preview;
use regex::Regex;

pub struct SecurityFilter {
    alert_type_re: Regex,
    repo_resource_re: Regex,
    account_resource_re: Regex,
    action_re: Regex,
}

impl SecurityFilter {
    pub fn new() -> Result<Self, regex::Error> {
        let alert_type_re = Regex::new(
            r"(?i)\b(Run\s+failed|Build\s+failed|Deployment\s+failed|Pipeline\s+failed|Security\s+alert|Unusual\s+(?:sign-in|activity)|New\s+device\s+signed\s+in|Account\s+(?:suspended|restricted|locked)|Unauthorized\s+access)\b",
        )?;

        let repo_resource_re = Regex::new(
            r"(?i)\b(?:Repository|Workflow run|Run failed)\s*[:]?\s*([A-Za-z0-9_.\-]+/[A-Za-z0-9_.\-]+)\b",
        )?;

        let account_resource_re = Regex::new(
            r"(?i)\b(?:for\s+your|to\s+your|in\s+your)\s+([A-Za-z0-9\s]{3,25}?\s+Account)\b",
        )?;

        let action_re = Regex::new(
            r"(?i)\b(?:Action required|Please\s+verify|Review\s+and\s+secure|Take\s+action)\s*[:]?\s*([A-Za-z0-9\s,.]{5,50}?)(?:\.|\;|\n|$)",
        )?;

        Ok(Self {
            alert_type_re,
            repo_resource_re,
            account_resource_re,
            action_re,
        })
    }
}

pub const SECURITY_DETECTION_PATTERNS: &[&str] = &[
    r"(?i)\b(?:run\s+failed|workflow\s+run\s+failed|build\s+failed|deployment\s+(?:failed|crashed)|pipeline\s+failed|action\s+failed)\b",
    r"(?i)\b(?:security\s+alert|unusual\s+(?:sign-in|activity)|new\s+device\s+signed\s+in|suspicious\s+login|compromised\s+account|action\s+required\s*:\s*security)\b",
    r"(?i)\b(?:account\s+(?:suspended|restricted|deactivated|locked)|unauthorized\s+access|security\s+warning)\b",
];

impl SecurityFilter {
    pub fn category(&self) -> Category {
        Category::SecurityAlert
    }

    pub fn detection_patterns(&self) -> &[&'static str] {
        SECURITY_DETECTION_PATTERNS
    }

    pub fn extract(&self, email: &EmailRecord) -> ExtractedData {
        let text = format!("{}\n{}", email.subject, email.body);

        let alert_type = self
            .alert_type_re
            .captures(&text)
            .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
            .or_else(|| {
                if email.subject.to_lowercase().contains("run failed") {
                    Some("Build Failure".to_string())
                } else if email.subject.to_lowercase().contains("security") {
                    Some("Security Alert".to_string())
                } else {
                    Some("Alert".to_string())
                }
            });

        let target_resource = self
            .repo_resource_re
            .captures(&text)
            .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
            .or_else(|| {
                self.account_resource_re
                    .captures(&text)
                    .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
            })
            .or_else(|| {
                if let Some(open) = email.subject.find('[')
                    && let Some(close) = email.subject.find(']')
                    && open < close
                {
                    let tag = &email.subject[open + 1..close];
                    if tag.contains('/') {
                        return Some(tag.to_string());
                    }
                }
                None
            });

        let severity = if text.to_lowercase().contains("critical")
            || text.to_lowercase().contains("unauthorized")
            || text.to_lowercase().contains("suspended")
        {
            Some("Critical".to_string())
        } else {
            Some("High".to_string())
        };

        let action_required = self
            .action_re
            .captures(&text)
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()));

        let timeline_key = target_resource.as_ref().map(|res| {
            if res.contains('/') {
                format!("repo:{res}")
            } else {
                format!("account:{res}")
            }
        });

        let extraction = SecurityAlertExtraction {
            alert_type,
            target_resource,
            severity,
            action_required,
            timeline_key,
            raw_evidence: Some(email.subject.clone()),
        };

        ExtractedData::SecurityAlert(extraction)
    }

    pub fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String {
        generate_preview(Category::SecurityAlert, data, &email.subject, &email.body)
    }
}
