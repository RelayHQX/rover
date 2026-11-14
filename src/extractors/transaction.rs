use regex::Regex;

/// Extractor for transaction IDs, order numbers, booking references, and tracking numbers.
pub struct TransactionExtractor {
    txn_id: Regex,
    order_id: Regex,
    tracking_num: Regex,
    booking_id: Regex,
    invoice_num: Regex,
}

impl TransactionExtractor {
    pub fn new() -> Result<Self, regex::Error> {
        // Matches e.g. Txn Ref: TXN-89210492, Transaction ID: PF-994821, Reference: ET-7721839,
        // Mandate Ref: NC-0091823, Authorization code: TX-449102, Ride Ref: CC-109283, Transaction: SP-4910283
        let txn_id = Regex::new(
            r"(?i)\b(?:Txn Ref|Transaction ID|Ref Number|Transaction Reference|Trans ID|Transfer ID|Gateway Ref|Refund ARN|Batch Ref|UPI Ref|Reference|Mandate Ref|Authorization code|Ride Ref|Transaction)\s*[:#]?\s*([A-Za-z0-9_-]{6,35})\b",
        )?;

        // Matches e.g. Order #SQ-881920, Order ID: ORD-449102, Order number: BH-301928, Order: KC-20491, #NA-55102, Order number: GPA.3399-0949-2150-45472
        let order_id = Regex::new(
            r"(?i)\b(?:Order\s+ID|Order\s+number|Order\s*#|Order\s*:\s*#?|Order\s+Confirmation\s*#)\s*[:#]?\s*#?([A-Za-z0-9_.-]{5,35})\b",
        )?;

        // Matches e.g. Tracking number: TRK-9901423, Tracking: SP-10293847, Courier tracking: UF-PARCEL-881
        let tracking_num = Regex::new(
            r"(?i)\b(?:Tracking\s*(?:number)?|Courier tracking|Courier)\s*[:#]?\s*([A-Za-z0-9_-]{6,35})\b",
        )?;

        // Matches e.g. Booking ID: SK-78921, PNR SK-78921, Ref #GM-99182, Booking Confirmed: Villa Toscana #CS-1029
        let booking_id = Regex::new(
            r"(?i)\b(?:Booking\s+ID|PNR|Reservation\s+(?:Ref|Status)|Ref\s*#|Booking\s+Confirmed\s*:[^\n#]+#|ticket\s+confirmation\s*:\s*#)\s*[:#]?\s*#?([A-Za-z0-9_-]{5,25})\b",
        )?;

        // Matches e.g. Invoice INV-2026-8812, Invoice Number: ME-9910482, Invoice #SLA-2026-301, Invoice GMP-1092, Bill INV-77410
        let invoice_num = Regex::new(
            r"(?i)\b(?:Invoice\s*(?:Number|#)?|bill\s+#?)\s*[:#]?\s*#?([A-Za-z0-9]{2,10}(?:-[A-Za-z0-9]+)+|\d{6,20})\b",
        )?;

        Ok(Self {
            txn_id,
            order_id,
            tracking_num,
            booking_id,
            invoice_num,
        })
    }

    pub fn extract_transaction_id(&self, text: &str) -> Option<String> {
        self.txn_id
            .captures(text)
            .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
    }

    pub fn extract_order_id(&self, text: &str) -> Option<String> {
        self.order_id
            .captures(text)
            .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
    }

    pub fn extract_tracking_number(&self, text: &str) -> Option<String> {
        self.tracking_num
            .captures(text)
            .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
    }

    pub fn extract_booking_id(&self, text: &str) -> Option<String> {
        self.booking_id
            .captures(text)
            .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
    }

    pub fn extract_invoice_number(&self, text: &str) -> Option<String> {
        self.invoice_num
            .captures(text)
            .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
    }
}
