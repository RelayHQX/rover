use crate::extractors::{AmountExtractor, IdentityExtractor, TransactionExtractor};
use crate::models::{Category, EmailRecord, ExtractedData, ShoppingExtraction};
use crate::preview::generate_preview;
use regex::Regex;

pub struct ShoppingFilter {
    amount_ext: AmountExtractor,
    id_ext: IdentityExtractor,
    txn_ext: TransactionExtractor,
    product_re: Regex,
    product_containing_re: Regex,
    product_for_re: Regex,
    subject_for_re: Regex,
    subject_dispatched_re: Regex,
    item_price_re: Regex,
    status_re: Regex,
    tax_re: Regex,
    order_date_re: Regex,
    payment_method_re: Regex,
    courier_re: Regex,
}

impl ShoppingFilter {
    pub fn new() -> Result<Self, regex::Error> {
        let amount_ext = AmountExtractor::new()?;
        let id_ext = IdentityExtractor::new()?;
        let txn_ext = TransactionExtractor::new()?;

        let product_re = Regex::new(
            r"(?i)\b(?:Product|Item|exchange)\s*[:]?\s*([A-Za-z0-9\s/()\-]{4,35}?)(?:\.|\,|\;|\n|Total|Amount|Value|\$|€|£|₹|$)",
        )?;

        let product_containing_re = Regex::new(
            r"(?i)\bcontaining\s+([A-Za-z0-9\s/()\-]{4,35}?)(?:\s+was|\s+has|\.|\,|\;|\n)",
        )?;

        let product_for_re = Regex::new(
            r"(?i)\b(?:order\s+[A-Za-z0-9\-#]+\s+for|order\s+confirmed\s*:\s*[A-Za-z0-9\-#]+\s+for)\s+([A-Za-z0-9\s/()\-]{4,35}?)(?:\s+has|\s+is|\.|\,|\;|\n|$)",
        )?;

        let subject_for_re = Regex::new(r"(?i)for\s+([A-Za-z0-9\s/()\-]{4,35}?)$")?;

        let subject_dispatched_re =
            Regex::new(r"(?i)Dispatched\s*:\s*([A-Za-z0-9\s/()\-]{4,35}?)\s+#")?;

        let item_price_re = Regex::new(
            r"(?im)(?:^|\n)\s*([A-Za-z0-9 /()\-]{2,60}?)\s*[\$€£₹]\s*\d",
        )?;

        let status_re = Regex::new(
            r"(?i)\b(?:Delivery status|Status)\s*[:]?\s*(Processing|In Transit|Out for Delivery|Delivered|Shipped|Preparing for Dispatch|Scheduled for Delivery|Ready for Pickup)\b",
        )?;

        let tax_re = Regex::new(
            r"(?i)\b(?:Includes\s+GST\s+of|Tax|GST|VAT|Sales\s+Tax)\s*[:]?\s*[\$€£₹]?\s*(\d+(?:\.\d{2})?)",
        )?;

        let order_date_re = Regex::new(
            r"(?i)\b(?:Order\s+date|Date\s+of\s+order|Purchased\s+on|Ordered\s+on)\s*[:]?\s*([0-9]{1,2}\s+[A-Za-z]{3,9}\s+[0-9]{4}|\d{4}-\d{2}-\d{2})",
        )?;

        let payment_method_re = Regex::new(
            r"(?i)\b(?:Payment\s+method|Paid\s+with|Paid\s+using|Payment\s+type)\s*[:]?\s*\r?\n*\s*([A-Za-z0-9\s\-]{3,35}?)(?:\r?\n\s*\r?\n|\.|\,|\;|<|Play\s+Points|$)",
        )?;

        let courier_re = Regex::new(
            r"(?i)\b(?:Courier|Shipped\s+via|Delivered\s+by|Carrier)\s*[:]?\s*([A-Za-z0-9\s]{3,25}?)(?:\.|\,|\;|\n|Tracking|$)",
        )?;

        Ok(Self {
            amount_ext,
            id_ext,
            txn_ext,
            product_re,
            product_containing_re,
            product_for_re,
            subject_for_re,
            subject_dispatched_re,
            item_price_re,
            status_re,
            tax_re,
            order_date_re,
            payment_method_re,
            courier_re,
        })
    }
}

pub const SHOPPING_DETECTION_PATTERNS: &[&str] = &[
    r"(?i)\b(?:order\s+(?:confirmed|confirmation|placed|shipped|dispatched|receipt)|package\s+(?:shipped|delivered|on\s+the\s+way)|out\s+for\s+delivery|delivered\s*:\s*order)\b",
    r"(?i)\b(?:shipment\s+update|grocery\s+delivery\s+is\s+scheduled|return\s+received\s+&\s+exchange|ready\s+for\s+(?:pickup|locker))\b",
    r"(?i)\b(?:dispatched\s*:\s*[A-Za-z0-9]|has\s+shipped\s+your\s+order|purchase\s+(?:receipt|from)|order\s+(?:#|number)|item\s+ordered)\b",
];

impl ShoppingFilter {
    pub fn category(&self) -> Category {
        Category::Shopping
    }

    pub fn detection_patterns(&self) -> &[&'static str] {
        SHOPPING_DETECTION_PATTERNS
    }

    pub fn extract(&self, email: &EmailRecord) -> ExtractedData {
        let text = format!("{}\n{}", email.subject, email.body);

        let store = if email.sender_name.to_lowercase().contains("google play")
            || email.sender.to_lowercase().contains("googleplay")
            || text.to_lowercase().contains("on google play")
        {
            Some("Google Play".to_string())
        } else {
            self.id_ext.extract_store(&text).or_else(|| {
                let name = &email.sender_name;
                for keyword in &[
                    " Store",
                    " Logistics",
                    " Alerts",
                    " Decor",
                    " Returns",
                    " Supplies",
                    " Official",
                ] {
                    if let Some(pos) = name.find(keyword) {
                        return Some(name[..pos].trim().to_string());
                    }
                }
                Some(name.clone())
            })
        };

        let order_id = self.txn_ext.extract_order_id(&text);

        let mut product = self
            .product_re
            .captures(&email.body)
            .or_else(|| self.product_containing_re.captures(&email.body))
            .or_else(|| self.product_for_re.captures(&text))
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
            .filter(|p| !p.eq_ignore_ascii_case("price") && !p.eq_ignore_ascii_case("item"))
            .or_else(|| {
                for caps in self.item_price_re.captures_iter(&email.body) {
                    if let Some(m) = caps.get(1) {
                        let cand = m.as_str().trim();
                        let lower = cand.to_lowercase();
                        if !cand.starts_with('(')
                            && !lower.starts_with("total")
                            && !lower.starts_with("includes")
                            && !cand.is_empty()
                        {
                            return Some(cand.to_string());
                        }
                    }
                }
                None
            })
            .or_else(|| {
                self.subject_for_re
                    .captures(&email.subject)
                    .or_else(|| self.subject_dispatched_re.captures(&email.subject))
                    .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
            });

        if let Some(prod) = product.as_mut()
            && let Some(idx) = prod.find('(')
        {
            let trimmed = prod[..idx].trim();
            if !trimmed.is_empty() {
                *prod = trimmed.to_string();
            }
        }

        let amount_info = self.amount_ext.extract(&email.body);
        let amount = amount_info.as_ref().map(|a| a.value);
        let currency = amount_info.map(|a| a.currency);

        let tax = self
            .tax_re
            .captures(&text)
            .and_then(|c| c.get(1))
            .and_then(|m| rust_decimal::Decimal::from_str_exact(m.as_str()).ok());

        let order_date = self
            .order_date_re
            .captures(&text)
            .and_then(|c| c.get(1).map(|m| m.as_str().to_string()));

        let payment_method = self
            .payment_method_re
            .captures(&text)
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
            .or_else(|| {
                if text.contains("Google Play balance") {
                    Some("Google Play balance".to_string())
                } else {
                    None
                }
            });

        let courier = self
            .courier_re
            .captures(&text)
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()));

        let delivery_status = self
            .status_re
            .captures(&text)
            .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
            .or_else(|| {
                let lower = text.to_lowercase();
                if lower.contains("out for delivery") {
                    Some("Out for Delivery".to_string())
                } else if lower.contains("delivered") {
                    Some("Delivered".to_string())
                } else if lower.contains("shipped") || lower.contains("dispatched") {
                    Some("Shipped".to_string())
                } else if lower.contains("in transit") {
                    Some("In Transit".to_string())
                } else if lower.contains("ready for pickup") {
                    Some("Ready for Pickup".to_string())
                } else {
                    Some("Processing".to_string())
                }
            });

        let tracking_number = self.txn_ext.extract_tracking_number(&email.body);

        let timeline_key = order_id.as_ref().map(|id| format!("order:{id}"));

        let extraction = ShoppingExtraction {
            store,
            order_id,
            product,
            amount,
            currency,
            tax,
            order_date,
            delivery_status,
            tracking_number,
            courier,
            payment_method,
            timeline_key,
            raw_evidence: Some(email.subject.clone()),
        };

        ExtractedData::Shopping(extraction)
    }

    pub fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String {
        generate_preview(Category::Shopping, data, &email.subject, &email.body)
    }
}
