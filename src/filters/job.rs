use crate::extractors::DateExtractor;
use crate::models::{Category, EmailRecord, ExtractedData, JobExtraction};
use crate::preview::generate_preview;
use regex::Regex;

pub struct JobFilter {
    date_ext: DateExtractor,
    company_re: Regex,
    company_intro_re: Regex,
    position_re: Regex,
    position_as_re: Regex,
    position_at_re: Regex,
    coding_challenge_re: Regex,
    subject_role_re: Regex,
    location_re: Regex,
    salary_re: Regex,
    deadline_re: Regex,
    interview_date_re: Regex,
    job_count_re: Regex,
    dash_location_re: Regex,
    job_url_re: Regex,
}

impl JobFilter {
    pub fn new() -> Result<Self, regex::Error> {
        let date_ext = DateExtractor::new()?;

        let company_re = Regex::new(
            r"(?i)\b(?:at|with|to)\s+([A-Z][A-Za-z0-9&.'\- ]{2,25}?)(?:\s+for|\s+Talent|\s+Careers|\s+People|\s+HR|\.|\,|\;|\n|$)",
        )?;

        let company_intro_re = Regex::new(
            r"(?i)\b([A-Z][A-Za-z0-9&.'\- ]{2,25}?)\s+is\s+(?:pleased\s+to\s+extend|reviewing\s+applications|reviewing\s+your)",
        )?;

        let position_re = Regex::new(
            r"(?i)\b(?:role\s+of|position\s+of|for\s+the\s+role\s+of|hiring\s+at\s+[A-Za-z0-9&.'\- ]+?\s+for|position\s+at\s+[A-Za-z0-9&.'\- ]+?\s+for)\s*[:]?\s*([A-Z][A-Za-z0-9\s/()\-]{4,40}?)(?:\s+located|\s+at|\.|\,|\;|\n|$)",
        )?;

        let position_as_re = Regex::new(
            r"(?i)\b(?:as|for the)\s+([A-Z][A-Za-z0-9\s/()\-]{4,35}?)(?:\s+in\s+[A-Z]|\s+vacancy|\.|\,|\;|\n|$)",
        )?;

        let position_at_re = Regex::new(
            r"(?i)^([A-Za-z0-9\s/()\-]{4,35}?)\s+at\s+[A-Za-z0-9]",
        )?;

        let coding_challenge_re = Regex::new(
            r"(?i)\bCoding Challenge\s*:\s*([A-Za-z0-9\s/()\-]{4,35}?)(?:\s*\([^)]*\))?$",
        )?;

        let subject_role_re =
            Regex::new(r"(?i):\s*([A-Za-z0-9\s/()\-]{4,35}?)(?:\s+at|\s+located|\s*$)")?;

        let location_re = Regex::new(
            r"(?i)\b(?:Location\s*[:]\s*|located\s+in\s+|role\s+in\s+|vacancy\s+in\s+|as\s+[^\n]+?\s+in\s+)([A-Za-z0-9\s,/\-()]{2,30}?)(?:\.|\;|\n|Salary|Our|Expected|Compensation|$)",
        )?;

        let salary_re = Regex::new(
            r"(?i)\b(?:salary(?:\s+range)?|compensation(?:\s+band)?|starting\s+salary|base\s+(?:salary|compensation)|band)\s*[:]?\s*((?:CAD\s*)?[\$€£₹]?\d{1,3}(?:,\d{3})*(?:\s*-\s*[\$€£₹]?\d{1,3}(?:,\d{3})*)?)",
        )?;

        let deadline_re = Regex::new(
            r"(?i)\b(?:deadline(?:\s+(?:of|is|to\s+\w+))?|closes\s+deadline|respond\s+by|confirm\s+by|availability\s+by(?:\s+deadline)?)\s*[:]?\s*(\d{4}-\d{2}-\d{2})\b",
        )?;

        let interview_date_re = Regex::new(
            r"(?i)\b(?:interview\s+date|scheduled\s+for|round\s+at[^\n]+Date|session\s+is|call\s+on)\s*[:]?\s*(\d{4}-\d{2}-\d{2})\b",
        )?;

        let job_count_re = Regex::new(
            r"(?i)\b(\d+)\s+new\s+[^\n]+vacanc|\bJobs\s+1-\d+\s+of\s+(\d+)\s+new\s+jobs|\b(\d+)\s+more\s+[^\n]+jobs",
        )?;

        let dash_location_re = Regex::new(
            r"(?m)^[A-Za-z0-9&.'\- ]{2,30}\s*-\s*([A-Za-z0-9\s,]{2,35})$",
        )?;

        let job_url_re = Regex::new(
            r#"https://[^\s"'<>]+(?:indeed\.com/(?:rc/clk|jobs)|linkedin\.com/jobs/view)[^\s"'<>]*"#,
        )?;

        Ok(Self {
            date_ext,
            company_re,
            company_intro_re,
            position_re,
            position_as_re,
            position_at_re,
            coding_challenge_re,
            subject_role_re,
            location_re,
            salary_re,
            deadline_re,
            interview_date_re,
            job_count_re,
            dash_location_re,
            job_url_re,
        })
    }
}

pub const JOB_DETECTION_PATTERNS: &[&str] = &[
    r"(?i)\b(?:interview\s+invitation|interview\s+scheduled|offer\s+letter|job\s+offer|application\s+(?:received|acknowledgement|status)|coding\s+challenge|phone\s+screen\s+invitation|onboarding\s+schedule)\b",
    r"(?i)\b(?:next\s+steps\s*:[^\n]+interview|invitation\s+to\s+final\s+round)\b",
    r"(?i)\b(?:job\s+alert|new\s+jobs|jobs\s+matching|\d+\s+more\s+.*?jobs|job\s+opening|job\s+vacanc(?:y|ies)|open\s+positions|hiring\s+for)\b",
];

impl JobFilter {
    pub fn category(&self) -> Category {
        Category::Job
    }

    pub fn detection_patterns(&self) -> &[&'static str] {
        JOB_DETECTION_PATTERNS
    }

    pub fn extract(&self, email: &EmailRecord) -> ExtractedData {
        let text = format!("{}\n{}", email.subject, email.body);

        let company = self
            .company_intro_re
            .captures(&email.body)
            .or_else(|| self.company_re.captures(&text))
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
            .or_else(|| {
                let name = &email.sender_name;
                for keyword in &[
                    " Talent",
                    " Careers",
                    " People",
                    " HR",
                    " Recruitment",
                    " Recruiting",
                    " Health",
                ] {
                    if let Some(pos) = name.find(keyword) {
                        return Some(name[..pos].trim().to_string());
                    }
                }
                Some(name.clone())
            });

        let mut position = self
            .coding_challenge_re
            .captures(&email.subject)
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
            .or_else(|| {
                self.position_as_re
                    .captures(&email.body)
                    .or_else(|| self.position_re.captures(&text))
                    .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
            })
            .or_else(|| {
                self.position_at_re
                    .captures(&email.subject)
                    .or_else(|| self.subject_role_re.captures(&email.subject))
                    .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
            });

        // Clean trailing suffixes and extra qualifiers from position
        if let Some(pos) = position.as_mut() {
            for suffix in &[" role", " vacancy", " Interview", " interview"] {
                if pos.ends_with(suffix) {
                    *pos = pos[..pos.len() - suffix.len()].trim().to_string();
                }
            }
            if let Some(idx) = pos.find('(') {
                *pos = pos[..idx].trim().to_string();
            }
        }

        let mut location = self
            .location_re
            .captures(&email.body)
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
            .or_else(|| {
                for caps in self.dash_location_re.captures_iter(&email.body) {
                    if let Some(m) = caps.get(1) {
                        let cand = m.as_str().trim();
                        let lower = cand.to_lowercase();
                        if !lower.contains("jobs")
                            && !lower.contains("vacancy")
                            && !cand.chars().any(|ch| ch.is_ascii_digit())
                        {
                            return Some(cand.to_string());
                        }
                    }
                }
                None
            });

        if let Some(loc) = location.as_mut()
            && let Some(idx) = loc.find('(')
        {
            *loc = loc[..idx].trim().to_string();
        }

        let salary = self
            .salary_re
            .captures(&email.body)
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()));

        let deadline = self
            .deadline_re
            .captures(&email.body)
            .and_then(|c| c.get(1).map(|m| m.as_str().to_string()));

        let interview_date = self
            .interview_date_re
            .captures(&email.body)
            .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
            .or_else(|| {
                if email.subject.to_lowercase().contains("interview") {
                    self.date_ext.extract(&email.body)
                } else {
                    None
                }
            });

        let is_job_alert = email.subject.to_lowercase().contains("job alert")
            || email.body.to_lowercase().contains("job alert")
            || email.subject.to_lowercase().contains("more jobs")
            || email.body.to_lowercase().contains("vacanc")
            || email.body.to_lowercase().contains("new jobs")
            || email.sender.to_lowercase().contains("jobalert");

        let job_count = self
            .job_count_re
            .captures(&email.body)
            .or_else(|| self.job_count_re.captures(&email.subject))
            .and_then(|caps| {
                for i in 1..caps.len() {
                    if let Some(m) = caps.get(i)
                        && let Ok(n) = m.as_str().parse::<u32>()
                    {
                        return Some(n);
                    }
                }
                None
            });

        let job_url = self
            .job_url_re
            .find(&email.body)
            .or_else(|| self.job_url_re.find(&email.subject))
            .map(|m| m.as_str().to_string());

        let extraction = JobExtraction {
            company,
            position,
            location,
            salary,
            job_count,
            is_job_alert,
            job_url,
            application_deadline: deadline,
            interview_date,
            raw_evidence: Some(email.subject.clone()),
        };

        ExtractedData::Job(extraction)
    }

    pub fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String {
        generate_preview(Category::Job, data, &email.subject, &email.body)
    }
}
