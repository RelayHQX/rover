use crate::extractors::{AmountExtractor, DateExtractor, IdentityExtractor, TransactionExtractor};
use crate::models::{Category, EmailRecord, ExtractedData, InvoiceExtraction};
use crate::preview::generate_preview;
use regex::Regex;

pub struct InvoiceFilter {
    amount_ext: AmountExtractor,
    date_ext: DateExtractor,
    id_ext: IdentityExtractor,
    txn_ext: TransactionExtractor,
    due_date_re: Regex,
}

impl InvoiceFilter {
    pub fn new() -> Result<Self, regex::Error> {
        let amount_ext = AmountExtractor::new()?;
        let date_ext = DateExtractor::new()?;
        let id_ext = IdentityExtractor::new()?;
        let txn_ext = TransactionExtractor::new()?;

        let due_date_re = Regex::new(
            r"(?i)\b(?:due\s+date|payment\s+due(?:\s+date)?|due|pay\s+by(?:\s+due\s+date)?)\s*[:]?\s*(\d{4}-\d{2}-\d{2})\b",
        )?;

        Ok(Self {
            amount_ext,
            date_ext,
            id_ext,
            txn_ext,
            due_date_re,
        })
    }
}

pub const INVOICE_DETECTION_PATTERNS: &[&str] = &[
    r"(?i)\b(?:invoice\s+(?:inv|#|statement|attached|number|[A-Za-z0-9\-]+)|bill\s+statement|utility\s+bill|broadband\s+bill|patient\s+statement)\b",
    r"(?i)\b(?:total\s+amount\s+due|payment\s+due\s+date|freight\s+invoice|facilities\s+maintenance\s+invoice|total\s+invoice\s+amount)\b",
];

impl InvoiceFilter {
    pub fn category(&self) -> Category {
        Category::Invoice
    }

    pub fn detection_patterns(&self) -> &[&'static str] {
        INVOICE_DETECTION_PATTERNS
    }

    pub fn extract(&self, email: &EmailRecord) -> ExtractedData {
        let text = format!("{} {}", email.subject, email.body);

        let invoice_number = self.txn_ext.extract_invoice_number(&text);

        let amount_info = self.amount_ext.extract(&text);
        let amount = amount_info.as_ref().map(|a| a.value);
        let currency = amount_info.as_ref().map(|a| a.currency.clone());

        let due_date = self
            .due_date_re
            .captures(&text)
            .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
            .or_else(|| self.date_ext.extract(&email.body));

        let issuer = self.id_ext.extract_issuer(&text).or_else(|| {
            let name = &email.sender_name;
            for keyword in &[
                " Billing",
                " Corp",
                " Facilities",
                " Partners",
                " Clinic",
                " Broadband",
            ] {
                if let Some(pos) = name.find(keyword) {
                    return Some(name[..pos + keyword.len()].trim().to_string());
                }
            }
            Some(name.clone())
        });

        let timeline_key = invoice_number.as_ref().map(|num| format!("invoice:{num}"));

        let extraction = InvoiceExtraction {
            invoice_number,
            amount,
            currency,
            due_date,
            issuer,
            timeline_key,
            raw_evidence: Some(email.subject.clone()),
        };

        ExtractedData::Invoice(extraction)
    }

    pub fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String {
        generate_preview(Category::Invoice, data, &email.subject, &email.body)
    }
}
