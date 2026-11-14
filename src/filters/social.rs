use crate::extractors::IdentityExtractor;
use crate::models::{Category, EmailRecord, ExtractedData, SocialExtraction};
use crate::preview::generate_preview;
use regex::Regex;

pub struct SocialFilter {
    id_ext: IdentityExtractor,
    platform_re: Regex,
    activity_re: Regex,
    actor_re: Regex,
    post_url_re: Regex,
    target_post_re: Regex,
}

impl SocialFilter {
    pub fn new() -> Result<Self, regex::Error> {
        let id_ext = IdentityExtractor::new()?;

        let platform_re = Regex::new(
            r"(?i)\b(?:Platform|on)\s*[:]?\s*(ProConnect|ChatterBox|CodeReviewHub|VideoSphere|TechDiscuss)\b",
        )?;

        let activity_re = Regex::new(
            r"(?i)\b(?:Activity|Activity type)\s*[:]?\s*(Connection request|Mention|PR Review|Video Comment|Post Like)\b",
        )?;

        let actor_re = Regex::new(
            r"(?i)^([A-Za-z0-9\s.]{2,35}?)(?:\s*-\s*[^\n]+?)?\s+(?:reacted\s+to|commented|shared\s+a\s+post|liked\s+your|wants\s+to\s+connect|sent\s+you|mentioned\s+you)",
        )?;

        let post_url_re = Regex::new(
            r#"https://[^\s"'<>]+\b(?:feed/update|status|p/|posts/|comments/|pull/|issues/)[^\s"'<>]*"#,
        )?;

        let target_post_re = Regex::new(
            r"(?i)(?:reacted\s+to\s+this\s+post|commented\s+(?:on\s+a\s+post|a\s+post)|shared\s+a\s+post)\s*:\s*([^\n\r]+)",
        )?;

        Ok(Self {
            id_ext,
            platform_re,
            activity_re,
            actor_re,
            post_url_re,
            target_post_re,
        })
    }
}

pub const SOCIAL_DETECTION_PATTERNS: &[&str] = &[
    r"(?i)\b(?:invitation\s+to\s+connect\s+on|mentioned\s+you\s+in\s+a\s+post|requested\s+changes\s+on\s+pr)\b",
    r"(?i)\b(?:commented\s+on\s+your\s+video|liked\s+your\s+reply\s+in\s+thread)\b",
    r"(?i)\b(?:reacted\s+to\s+(?:this|your)|shared\s+a\s+post|started\s+following|new\s+connection\s+request|wants\s+to\s+connect|endorsed\s+you)\b",
];

impl SocialFilter {
    pub fn category(&self) -> Category {
        Category::Social
    }

    pub fn detection_patterns(&self) -> &[&'static str] {
        SOCIAL_DETECTION_PATTERNS
    }

    pub fn extract(&self, email: &EmailRecord) -> ExtractedData {
        let text = format!("{} {}", email.subject, email.body);

        let platform = self
            .platform_re
            .captures(&text)
            .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
            .or_else(|| {
                let name = &email.sender_name;
                let sender = &email.sender;
                for p in &[
                    "LinkedIn",
                    "GitHub",
                    "Twitter",
                    "YouTube",
                    "Discord",
                    "Reddit",
                    "Slack",
                    "Substack",
                    "ProConnect",
                    "ChatterBox",
                    "CodeReviewHub",
                    "VideoSphere",
                    "TechDiscuss",
                ] {
                    let p_lower = p.to_lowercase();
                    if name.to_lowercase().contains(&p_lower)
                        || sender.to_lowercase().contains(&p_lower)
                        || text.to_lowercase().contains(&p_lower)
                    {
                        return Some((*p).to_string());
                    }
                }
                None
            });

        let activity_type = self
            .activity_re
            .captures(&text)
            .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
            .or_else(|| {
                let lower = text.to_lowercase();
                if lower.contains("invitation to connect") || lower.contains("connection request") {
                    Some("Connection request".to_string())
                } else if lower.contains("mentioned you") {
                    Some("Mention".to_string())
                } else if lower.contains("pull request") || lower.contains("pr #") {
                    Some("PR Review".to_string())
                } else if lower.contains("commented on") {
                    Some("Video Comment".to_string())
                } else if lower.contains("reacted to") {
                    Some("Reaction".to_string())
                } else if lower.contains("shared a post") {
                    Some("Post Share".to_string())
                } else if lower.contains("liked your") {
                    Some("Post Like".to_string())
                } else {
                    None
                }
            });

        let actor = self
            .actor_re
            .captures(&email.subject)
            .or_else(|| self.actor_re.captures(&email.body))
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
            .or_else(|| self.id_ext.extract_social_actor(&text));

        let relevant_entity = self
            .target_post_re
            .captures(&email.subject)
            .or_else(|| self.target_post_re.captures(&email.body))
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
            .or_else(|| actor.clone());

        let post_url = self
            .post_url_re
            .find(&email.body)
            .or_else(|| self.post_url_re.find(&email.subject))
            .map(|m| {
                let url = m.as_str();
                if let Some(pos) = url.find('?') {
                    url[..pos].to_string()
                } else {
                    url.to_string()
                }
            });

        let extraction = SocialExtraction {
            platform,
            activity_type,
            relevant_entity,
            actor,
            post_url,
            raw_evidence: Some(email.subject.clone()),
        };

        ExtractedData::Social(extraction)
    }

    pub fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String {
        generate_preview(Category::Social, data, &email.subject, &email.body)
    }
}
