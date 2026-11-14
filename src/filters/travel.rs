use crate::extractors::{DateExtractor, TransactionExtractor};
use crate::models::{Category, EmailRecord, ExtractedData, TravelExtraction};
use crate::preview::generate_preview;
use regex::Regex;

pub struct TravelFilter {
    date_ext: DateExtractor,
    txn_ext: TransactionExtractor,
    destination_re: Regex,
    subject_dest_re: Regex,
    dep_date_re: Regex,
    ret_date_re: Regex,
    status_re: Regex,
}

impl TravelFilter {
    pub fn new() -> Result<Self, regex::Error> {
        let date_ext = DateExtractor::new()?;
        let txn_ext = TransactionExtractor::new()?;

        let destination_re = Regex::new(
            r"(?i)\b(?:Destination|heading to)\s*[:]?\s*([A-Z][A-Za-z0-9\s()\-]{2,25}?)(?:\.|\,|\;|\n|Departure|Booking|Seat|Guide|$)",
        )?;

        let subject_dest_re = Regex::new(r"(?i)to\s+([A-Za-z0-9\s()\-]{3,25})$")?;

        let dep_date_re =
            Regex::new(r"(?i)\b(?:departure\s+date|departure)\s*[:]?\s*(\d{4}-\d{2}-\d{2})\b")?;

        let ret_date_re =
            Regex::new(r"(?i)\b(?:return\s+date|return)\s*[:]?\s*(\d{4}-\d{2}-\d{2})\b")?;

        let status_re = Regex::new(
            r"(?i)\b(?:Booking\s+status|Status)\s*[:]?\s*(Confirmed|Issued|Pending|Completed)\b",
        )?;

        Ok(Self {
            date_ext,
            txn_ext,
            destination_re,
            subject_dest_re,
            dep_date_re,
            ret_date_re,
            status_re,
        })
    }
}

pub const TRAVEL_DETECTION_PATTERNS: &[&str] = &[
    r"(?i)\b(?:flight\s+booking|hotel\s+reservation|train\s+e-ticket|car\s+rental\s+confirmation|ferry\s+ticket\s+confirmation|tour\s+reservation\s+status)\b",
    r"(?i)\b(?:cozystay\s+booking|flight\s+itinerary\s+&\s+boarding)\b",
];

impl TravelFilter {
    pub fn category(&self) -> Category {
        Category::Travel
    }

    pub fn detection_patterns(&self) -> &[&'static str] {
        TRAVEL_DETECTION_PATTERNS
    }

    pub fn extract(&self, email: &EmailRecord) -> ExtractedData {
        let text = format!("{}\n{}", email.subject, email.body);

        let booking_id = self.txn_ext.extract_booking_id(&text);

        let destination = self
            .destination_re
            .captures(&email.body)
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
            .or_else(|| {
                self.subject_dest_re
                    .captures(&email.subject)
                    .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
            });

        let departure_date = self
            .dep_date_re
            .captures(&email.body)
            .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
            .or_else(|| self.date_ext.extract(&email.body));

        let return_date = self
            .ret_date_re
            .captures(&email.body)
            .and_then(|c| c.get(1).map(|m| m.as_str().to_string()));

        let booking_status = self
            .status_re
            .captures(&email.body)
            .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
            .or_else(|| {
                if text.to_lowercase().contains("confirmed") {
                    Some("Confirmed".to_string())
                } else {
                    None
                }
            });

        let timeline_key = booking_id.as_ref().map(|id| format!("travel:{id}"));

        let extraction = TravelExtraction {
            booking_id,
            destination,
            departure_date,
            return_date,
            booking_status,
            timeline_key,
            raw_evidence: Some(email.subject.clone()),
        };

        ExtractedData::Travel(extraction)
    }

    pub fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String {
        generate_preview(Category::Travel, data, &email.subject, &email.body)
    }
}
