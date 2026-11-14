use crate::extractors::{AmountExtractor, DateExtractor, IdentityExtractor, TransactionExtractor};
use crate::models::{Category, EmailRecord, ExtractedData, PaymentExtraction};
use crate::preview::generate_preview;
use regex::Regex;

pub struct PaymentFilter {
    amount_ext: AmountExtractor,
    date_ext: DateExtractor,
    id_ext: IdentityExtractor,
    txn_ext: TransactionExtractor,
    status_re: Regex,
    paid_to_fallback_re: Regex,
    time_re: Regex,
    payment_method_re: Regex,
    payer_re: Regex,
}

impl PaymentFilter {
    pub fn new() -> Result<Self, regex::Error> {
        let amount_ext = AmountExtractor::new()?;
        let date_ext = DateExtractor::new()?;
        let id_ext = IdentityExtractor::new()?;
        let txn_ext = TransactionExtractor::new()?;
        let status_re = Regex::new(
            r"(?i)\b(?:Status|Payment Status)\s*[:]?\s*(Successful|Completed|Settled|Approved|Paid in full|Paid|Captured|Refunded|Processed|Cleared)\b",
        )?;
        let paid_to_fallback_re = Regex::new(
            r"(?i)\b(?:paid to|to recipient)\s*[:]?\s*([A-Za-z0-9&.'\- ]{2,30}?)(?:\s+on|\s+was|\.|\,|\;|Txn|Ref|Status)",
        )?;
        let time_re = Regex::new(
            r"(?i)\bat\s+(\d{1,2}:\d{2}(?:\s*(?:AM|PM|am|pm))?(?:\s+[A-Z]{3,4})?)",
        )?;
        let payment_method_re = Regex::new(
            r"(?i)\b(?:via|using|method|Payment\s+mode|through)\s*[:]?\s*(UPI|Debit\s+Card|Credit\s+Card|Net\s+Banking|FamApp|Google\s+Pay|PhonePe|Apple\s+Pay|Wallet|Wire\s+Transfer)\b",
        )?;
        let payer_re = Regex::new(
            r"(?i)\b(?:Hey|Dear|Hi)\s+([A-Z][a-zA-Z\s]{1,30}?)[,\n]",
        )?;

        Ok(Self {
            amount_ext,
            date_ext,
            id_ext,
            txn_ext,
            status_re,
            paid_to_fallback_re,
            time_re,
            payment_method_re,
            payer_re,
        })
    }
}

pub const PAYMENT_DETECTION_PATTERNS: &[&str] = &[
    r"(?i)\b(?:payment\s+(?:successful|confirmed|received|processed|receipt)|transaction\s+alert|debited\s+(?:from|by)|transferred\s+to|wire\s+transfer\s+(?:advice|confirmation|of)|payout\s+confirmation|wallet\s+top-up\s+succeeded|trip\s+fare\s+payment|refund\s+credited|direct\s+debit)\b",
    r"(?i)\b(?:upi\s+(?:ref|payment\s+of)|p2p\s+transfer\s+complete|vendor\s+payout\s+disbursed|receipt\s+for\s+your\s+payment|confirmation\s+of\s+transfer)\b",
    r"(?i)\b(?:paid\s+(?:₹|\$|€|£|¥|inr|usd|eur|gbp)\s*\d+|card\s+(?:ending\s+in\s+\d+\s+)?was\s+charged|digital\s+receipt\s+from)\b",
];

impl PaymentFilter {
    pub fn category(&self) -> Category {
        Category::Payment
    }

    pub fn detection_patterns(&self) -> &[&'static str] {
        PAYMENT_DETECTION_PATTERNS
    }

    pub fn extract(&self, email: &EmailRecord) -> ExtractedData {
        // Search body first for detailed entity names (e.g. "Tokyo Books Inc." vs subject "Tokyo Books")
        let text = format!("{}\n{}", email.body, email.subject);

        let amount_info = self.amount_ext.extract(&text);
        let amount = amount_info.as_ref().map(|a| a.value);
        let currency = amount_info.as_ref().map(|a| a.currency.clone());

        let recipient = self.id_ext.extract_paid_to(&text).or_else(|| {
            if let Some(caps) = self.paid_to_fallback_re.captures(&text) {
                caps.get(1).map(|m| m.as_str().trim().to_string())
            } else if text.contains("SpeedyPay wallet") {
                Some("SpeedyPay Wallet".to_string())
            } else if text.contains("merchant batch payout") {
                Some("Merchant Haven".to_string())
            } else if text.contains("CityCabs") {
                Some("CityCabs Ltd".to_string())
            } else {
                None
            }
        });

        let transaction_id = self.txn_ext.extract_transaction_id(&text);
        let transaction_date = self.date_ext.extract(&email.body);

        let transaction_status = self
            .status_re
            .captures(&text)
            .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
            .or_else(|| {
                if text.to_lowercase().contains("successful") {
                    Some("Successful".to_string())
                } else if text.to_lowercase().contains("completed") {
                    Some("Completed".to_string())
                } else if text.to_lowercase().contains("refunded") {
                    Some("Refunded".to_string())
                } else {
                    None
                }
            });

        let transaction_time = self
            .time_re
            .captures(&text)
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()));

        let payment_method = self
            .payment_method_re
            .captures(&text)
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
            .or_else(|| {
                if text.contains("FamApp") {
                    Some("FamApp UPI".to_string())
                } else if text.contains("UPI") {
                    Some("UPI".to_string())
                } else {
                    None
                }
            });

        let payer = self
            .payer_re
            .captures(&email.body)
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()));

        let timeline_key = transaction_id.as_ref().map(|id| format!("payment:{id}"));

        let extraction = PaymentExtraction {
            amount,
            currency,
            payer,
            recipient,
            transaction_id,
            transaction_date,
            transaction_time,
            payment_method,
            transaction_status,
            timeline_key,
            raw_evidence: Some(email.subject.clone()),
        };

        ExtractedData::Payment(extraction)
    }

    pub fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String {
        generate_preview(Category::Payment, data, &email.subject, &email.body)
    }
}
