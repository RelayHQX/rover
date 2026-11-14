use crate::models::{Category, ExtractedData};

/// Generate a concise, contextual preview (< 100 chars) for a processed email.
pub fn generate_preview(
    category: Category,
    data: &ExtractedData,
    subject: &str,
    body: &str,
) -> String {
    let preview = match (category, data) {
        (Category::Payment, ExtractedData::Payment(p)) => {
            let amount_str = match (&p.currency, &p.amount) {
                (Some(c), Some(a)) => format_currency(c, a),
                (None, Some(a)) => format!("{a}"),
                _ => "Payment".to_string(),
            };
            let recipient_str = p.recipient.as_deref().unwrap_or("Merchant");
            let status_str = p.transaction_status.as_deref().unwrap_or("Processed");
            format!("{amount_str} paid to {recipient_str} · {status_str}")
        }
        (Category::Otp, ExtractedData::Otp(o)) => {
            let service = o.service_name.as_deref().unwrap_or("Service");
            let code = o.otp_code.as_deref().unwrap_or("------");
            if let Some(exp) = &o.expiration_time {
                format!("{service} verification code: {code} (Expires in {exp})")
            } else {
                format!("{service} verification code: {code}")
            }
        }
        (Category::Job, ExtractedData::Job(j)) => {
            let pos = j.position.as_deref().unwrap_or("Role");
            let company = j.company.as_deref().unwrap_or("Company");
            let sub_lower = subject.to_lowercase();
            if j.is_job_alert || sub_lower.contains("job alert") || sub_lower.contains("jobs for you") {
                let count_desc = match j.job_count {
                    Some(1) => format!("{pos} at {company}"),
                    Some(c) => format!("{c} {pos} jobs at {company}"),
                    None => format!("{pos} at {company}"),
                };
                format!("Job Alert · {count_desc}")
            } else if sub_lower.contains("offer") {
                format!("Job offer · {pos} at {company}")
            } else if sub_lower.contains("interview") {
                format!("Interview invitation · {pos} at {company}")
            } else {
                format!("Application update · {pos}")
            }
        }
        (Category::Shopping, ExtractedData::Shopping(s)) => {
            let store = s.store.as_deref().unwrap_or("Order");
            let amount_str = match (&s.currency, &s.amount) {
                (Some(c), Some(a)) => format_currency(c, a),
                (None, Some(a)) => format!("${a}"),
                _ => String::new(),
            };
            let status = s.delivery_status.as_deref().unwrap_or("Order confirmed");

            if let Some(prod) = &s.product {
                if !amount_str.is_empty() {
                    format!("{amount_str} · {prod} ({store})")
                } else if s.tracking_number.is_some() {
                    format!("{status} · {prod} (Tracking available)")
                } else {
                    format!("{status} · {prod}")
                }
            } else if !amount_str.is_empty() {
                format!("{amount_str} · {store} purchase")
            } else if s.tracking_number.is_some() {
                format!("{status} · Tracking available")
            } else {
                format!("{store} · {status}")
            }
        }
        (Category::SecurityAlert, ExtractedData::SecurityAlert(sec)) => {
            let alert_type = sec.alert_type.as_deref().unwrap_or("Security Alert");
            if let Some(res) = &sec.target_resource {
                format!("⚠️ {alert_type} · {res}")
            } else {
                format!("⚠️ {alert_type}")
            }
        }
        (Category::Invoice, ExtractedData::Invoice(i)) => {
            let amount_str = match (&i.currency, &i.amount) {
                (Some(c), Some(a)) => format_currency(c, a),
                _ => "Invoice".to_string(),
            };
            if let Some(due) = &i.due_date {
                format!("Invoice received · {amount_str} due {due}")
            } else {
                format!("Invoice received · {amount_str}")
            }
        }
        (Category::Subscription, ExtractedData::Subscription(s)) => {
            let provider = s.provider.as_deref().unwrap_or("Subscription");
            let freq = s.billing_frequency.as_deref().unwrap_or("recurring");
            if let Some(date) = &s.renewal_date {
                format!("{provider} · {freq} renewal on {date}")
            } else {
                format!("{provider} · {freq} renewal")
            }
        }
        (Category::Travel, ExtractedData::Travel(t)) => {
            let dest = t.destination.as_deref().unwrap_or("Trip");
            let status = t.booking_status.as_deref().unwrap_or("Confirmed");
            if let Some(dep) = &t.departure_date {
                format!("Booking to {dest} · {status} for {dep}")
            } else {
                format!("Booking to {dest} · {status}")
            }
        }
        (Category::Newsletter, ExtractedData::Newsletter(n)) => {
            let pub_name = n.publisher.as_deref().unwrap_or("Newsletter");
            let topic_or_headline = n
                .headline
                .as_deref()
                .or(n.main_topic.as_deref())
                .unwrap_or("Latest updates");
            format!("{pub_name} · {topic_or_headline}")
        }
        (Category::Social, ExtractedData::Social(s)) => {
            let platform = s.platform.as_deref().unwrap_or("Social");
            let activity = s.activity_type.as_deref().unwrap_or("Activity");
            if let Some(actor) = &s.actor {
                format!("{platform} · {activity} from {actor}")
            } else if let Some(actor) = &s.relevant_entity {
                format!("{platform} · {activity} from {actor}")
            } else {
                format!("{platform} · {activity}")
            }
        }
        (Category::General, _) => {
            let clean = clean_excerpt(body);
            if clean.is_empty() {
                clean_excerpt(subject)
            } else {
                clean
            }
        }
        _ => clean_excerpt(body),
    };

    truncate_clean(&preview, 95)
}

fn format_currency(currency: &str, amount: &rust_decimal::Decimal) -> String {
    match currency.to_uppercase().as_str() {
        "INR" => format!("₹{amount}"),
        "USD" => format!("${amount}"),
        "EUR" => format!("€{amount}"),
        "GBP" => format!("£{amount}"),
        "JPY" => format!("¥{amount}"),
        other => format!("{amount} {other}"),
    }
}

/// Clean up whitespace, strip newlines, and produce a readable snippet.
pub fn clean_excerpt(text: &str) -> String {
    let mut clean = String::with_capacity(text.len());
    let mut last_was_space = false;

    for ch in text.chars() {
        if ch.is_whitespace() {
            if !last_was_space && !clean.is_empty() {
                clean.push(' ');
                last_was_space = true;
            }
        } else {
            clean.push(ch);
            last_was_space = false;
        }
    }

    clean
}

/// Truncate cleanly at a word boundary with an ellipsis.
pub fn truncate_clean(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        return text.to_string();
    }

    let mut end = max_len.saturating_sub(3);
    while end > 0 && !text.is_char_boundary(end) {
        end -= 1;
    }

    let slice = &text[..end];
    if let Some(last_space) = slice.rfind(' ') {
        format!("{}...", &slice[..last_space])
    } else {
        format!("{slice}...")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn test_truncate_clean() {
        let text = "This is a very long string that definitely needs some truncation";
        let res = truncate_clean(text, 25);
        assert!(res.len() <= 25);
        assert!(res.ends_with("..."));
    }

    #[test]
    fn test_payment_preview() {
        let p = crate::models::PaymentExtraction {
            amount: Some(Decimal::from_str("2500").unwrap()),
            currency: Some("INR".to_string()),
            recipient: Some("QuickMart Retail".to_string()),
            transaction_status: Some("Successful".to_string()),
            ..Default::default()
        };
        let prev = generate_preview(
            Category::Payment,
            &ExtractedData::Payment(p),
            "Payment done",
            "Details...",
        );
        assert_eq!(prev, "₹2500 paid to QuickMart Retail · Successful");
    }
}
