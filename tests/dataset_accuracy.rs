use relay_rover::Rover;
use relay_rover::models::{EmailRecord, ExtractedData};
use std::fs::File;
use std::io::BufReader;

#[test]
fn test_all_100_emails_accuracy() {
    let file = File::open("data/emails.json").expect("data/emails.json exists");
    let reader = BufReader::new(file);
    let emails: Vec<EmailRecord> = serde_json::from_reader(reader).expect("valid json");
    let rover = Rover::new().expect("Rover engine inits successfully");

    let mut category_mismatches = 0;
    let mut extraction_mismatches = 0;

    for email in &emails {
        let processed = rover.process(email);

        if processed.primary_category != email.expected_category {
            eprintln!(
                "[{}] Category mismatch: expected {:?}, got {:?}",
                email.id, email.expected_category, processed.primary_category
            );
            category_mismatches += 1;
        }

        let mismatches = count_all_extraction_mismatches(
            &email.id,
            &processed.extraction,
            &email.expected_extraction,
        );
        extraction_mismatches += mismatches;
    }

    assert_eq!(
        category_mismatches, 0,
        "Expected 0 category detection mismatches across all 100 emails"
    );
    assert_eq!(
        extraction_mismatches, 0,
        "Expected 0 field extraction mismatches across all 100 emails"
    );
}

fn count_all_extraction_mismatches(
    id: &str,
    data: &ExtractedData,
    expected: &serde_json::Value,
) -> usize {
    let obj = match expected.as_object() {
        Some(o) => o,
        None => return 0,
    };

    let mut mismatches = 0;

    match data {
        ExtractedData::Payment(p) => {
            mismatches += check_field(
                id,
                "amount",
                obj.get("amount"),
                p.amount.as_ref().map(|a| a.to_string()),
            );
            mismatches += check_field(id, "currency", obj.get("currency"), p.currency.clone());
            mismatches += check_field(id, "recipient", obj.get("recipient"), p.recipient.clone());
            mismatches += check_field(
                id,
                "transaction_id",
                obj.get("transaction_id"),
                p.transaction_id.clone(),
            );
            mismatches += check_field(id, "date", obj.get("date"), p.transaction_date.clone());
            mismatches += check_field(
                id,
                "status",
                obj.get("status"),
                p.transaction_status.clone(),
            );
        }
        ExtractedData::Otp(o) => {
            mismatches += check_field(id, "code", obj.get("code"), o.otp_code.clone());
            mismatches += check_field(id, "service", obj.get("service"), o.service_name.clone());
            mismatches += check_field(
                id,
                "expires_in",
                obj.get("expires_in"),
                o.expiration_time.clone(),
            );
        }
        ExtractedData::Job(j) => {
            mismatches += check_field(id, "company", obj.get("company"), j.company.clone());
            mismatches += check_field(id, "position", obj.get("position"), j.position.clone());
            mismatches += check_field(id, "location", obj.get("location"), j.location.clone());
            mismatches += check_field(id, "salary", obj.get("salary"), j.salary.clone());
            mismatches += check_field(
                id,
                "deadline",
                obj.get("deadline"),
                j.application_deadline.clone(),
            );
            mismatches += check_field(
                id,
                "interview_date",
                obj.get("interview_date"),
                j.interview_date.clone(),
            );
        }
        ExtractedData::Shopping(s) => {
            mismatches += check_field(id, "store", obj.get("store"), s.store.clone());
            mismatches += check_field(id, "order_id", obj.get("order_id"), s.order_id.clone());
            mismatches += check_field(id, "product", obj.get("product"), s.product.clone());
            mismatches += check_field(
                id,
                "amount",
                obj.get("amount"),
                s.amount.as_ref().map(|a| a.to_string()),
            );
            mismatches += check_field(id, "status", obj.get("status"), s.delivery_status.clone());
            mismatches += check_field(
                id,
                "tracking_number",
                obj.get("tracking_number"),
                s.tracking_number.clone(),
            );
        }
        ExtractedData::Invoice(i) => {
            mismatches += check_field(
                id,
                "invoice_number",
                obj.get("invoice_number"),
                i.invoice_number.clone(),
            );
            mismatches += check_field(
                id,
                "amount",
                obj.get("amount"),
                i.amount.as_ref().map(|a| a.to_string()),
            );
            mismatches += check_field(id, "currency", obj.get("currency"), i.currency.clone());
            mismatches += check_field(id, "due_date", obj.get("due_date"), i.due_date.clone());
            mismatches += check_field(id, "issuer", obj.get("issuer"), i.issuer.clone());
        }
        ExtractedData::Subscription(s) => {
            mismatches += check_field(id, "provider", obj.get("provider"), s.provider.clone());
            mismatches += check_field(
                id,
                "amount",
                obj.get("amount"),
                s.subscription_amount.as_ref().map(|a| a.to_string()),
            );
            mismatches += check_field(
                id,
                "frequency",
                obj.get("frequency"),
                s.billing_frequency.clone(),
            );
            mismatches += check_field(
                id,
                "renewal_date",
                obj.get("renewal_date"),
                s.renewal_date.clone(),
            );
            mismatches += check_field(
                id,
                "cancellation_info",
                obj.get("cancellation_info"),
                s.cancellation_info.clone(),
            );
        }
        ExtractedData::Travel(t) => {
            mismatches += check_field(
                id,
                "booking_id",
                obj.get("booking_id"),
                t.booking_id.clone(),
            );
            mismatches += check_field(
                id,
                "destination",
                obj.get("destination"),
                t.destination.clone(),
            );
            mismatches += check_field(
                id,
                "departure_date",
                obj.get("departure_date"),
                t.departure_date.clone(),
            );
            mismatches += check_field(
                id,
                "return_date",
                obj.get("return_date"),
                t.return_date.clone(),
            );
            mismatches += check_field(id, "status", obj.get("status"), t.booking_status.clone());
        }
        ExtractedData::Newsletter(n) => {
            mismatches += check_field(id, "publisher", obj.get("publisher"), n.publisher.clone());
            mismatches += check_field(
                id,
                "main_topic",
                obj.get("main_topic"),
                n.main_topic.clone(),
            );
            mismatches += check_field(id, "headline", obj.get("headline"), n.headline.clone());
        }
        ExtractedData::Social(s) => {
            mismatches += check_field(id, "platform", obj.get("platform"), s.platform.clone());
            mismatches += check_field(
                id,
                "activity_type",
                obj.get("activity_type"),
                s.activity_type.clone(),
            );
            mismatches += check_field(id, "actor", obj.get("actor"), s.relevant_entity.clone());
        }
        ExtractedData::General(_) => {}
        ExtractedData::SecurityAlert(sec) => {
            mismatches += check_field(id, "alert_type", obj.get("alert_type"), sec.alert_type.clone());
            mismatches += check_field(id, "target_resource", obj.get("target_resource"), sec.target_resource.clone());
        }
    }

    mismatches
}

fn check_field(
    id: &str,
    name: &str,
    expected: Option<&serde_json::Value>,
    actual: Option<String>,
) -> usize {
    if let Some(exp_val) = expected.and_then(|v| v.as_str())
        && actual.as_deref() != Some(exp_val)
    {
        eprintln!(
            "[{}] Field '{}' mismatch: expected {:?}, got {:?}",
            id, name, exp_val, actual
        );
        1
    } else {
        0
    }
}
