use relay_rover::engine::EmailEngine;
use relay_rover::models::{Category, EmailRecord, ExtractedData};
use rust_decimal::Decimal;
use std::str::FromStr;

fn make_test_email(subject: &str, body: &str) -> EmailRecord {
    EmailRecord {
        id: "test_email".to_string(),
        sender: "service@example.test".to_string(),
        sender_name: "Test Sender".to_string(),
        subject: subject.to_string(),
        received_at: "2026-11-05T10:00:00Z".to_string(),
        body: body.to_string(),
        expected_category: Category::General,
        expected_extraction: serde_json::Value::Null,
    }
}

#[test]
fn test_payment_category_and_extraction() {
    let engine = EmailEngine::new().unwrap();
    let email = make_test_email(
        "Transaction Alert: INR 2,500.00 debited from A/C XX4912",
        "Dear Customer, your account XX4912 has been debited by INR 2500.00 on 2026-11-05 for payment to QuickMart Retail. Txn Ref: TXN-89210492. Status: Successful.",
    );

    let processed = engine.process(&email);
    assert_eq!(processed.primary_category, Category::Payment);

    if let ExtractedData::Payment(p) = processed.extraction {
        assert_eq!(p.amount, Some(Decimal::from_str("2500.00").unwrap()));
        assert_eq!(p.currency.as_deref(), Some("INR"));
        assert_eq!(p.recipient.as_deref(), Some("QuickMart Retail"));
        assert_eq!(p.transaction_id.as_deref(), Some("TXN-89210492"));
        assert_eq!(p.transaction_date.as_deref(), Some("2026-11-05"));
        assert_eq!(p.transaction_status.as_deref(), Some("Successful"));
    } else {
        panic!("Expected Payment extraction");
    }

    assert!(processed.preview.contains("QuickMart Retail"));
}

#[test]
fn test_otp_category_and_extraction() {
    let engine = EmailEngine::new().unwrap();
    let email = make_test_email(
        "Your verification code is 482913",
        "Your one-time verification code for Acme Portal is 482913. This code expires in 10 minutes. Do not share this OTP.",
    );

    let processed = engine.process(&email);
    assert_eq!(processed.primary_category, Category::Otp);

    if let ExtractedData::Otp(o) = processed.extraction {
        assert_eq!(o.otp_code.as_deref(), Some("482913"));
        assert_eq!(o.service_name.as_deref(), Some("Acme Portal"));
        assert_eq!(o.expiration_time.as_deref(), Some("10 minutes"));
    } else {
        panic!("Expected OTP extraction");
    }

    assert_eq!(
        processed.preview,
        "Acme Portal verification code: 482913 (Expires in 10 minutes)"
    );
}

#[test]
fn test_job_category_and_extraction() {
    let engine = EmailEngine::new().unwrap();
    let email = make_test_email(
        "Interview Invitation: Senior Rust Systems Engineer at NovaTech",
        "We invite you for an interview for the Senior Rust Systems Engineer role at NovaTech. Interview date: 2026-11-12. Location: Remote. Salary: $180,000. Confirm by deadline 2026-11-08.",
    );

    let processed = engine.process(&email);
    assert_eq!(processed.primary_category, Category::Job);

    if let ExtractedData::Job(j) = processed.extraction {
        assert_eq!(j.company.as_deref(), Some("NovaTech"));
        assert_eq!(j.position.as_deref(), Some("Senior Rust Systems Engineer"));
        assert_eq!(j.location.as_deref(), Some("Remote"));
        assert_eq!(j.salary.as_deref(), Some("$180,000"));
        assert_eq!(j.application_deadline.as_deref(), Some("2026-11-08"));
        assert_eq!(j.interview_date.as_deref(), Some("2026-11-12"));
    } else {
        panic!("Expected Job extraction");
    }

    assert!(processed.preview.starts_with("Interview invitation"));
}

#[test]
fn test_shopping_category_and_extraction() {
    let engine = EmailEngine::new().unwrap();
    let email = make_test_email(
        "Order Confirmed: #SQ-881920 for Wireless Mechanical Keyboard",
        "Thank you for shopping at ShopQuick! Your order #SQ-881920 has been placed. Product: Wireless Mechanical Keyboard. Total amount: $129.99. Delivery status: Processing. Tracking number: TRK-9901423.",
    );

    let processed = engine.process(&email);
    assert_eq!(processed.primary_category, Category::Shopping);

    if let ExtractedData::Shopping(s) = processed.extraction {
        assert_eq!(s.store.as_deref(), Some("ShopQuick"));
        assert_eq!(s.order_id.as_deref(), Some("SQ-881920"));
        assert_eq!(s.product.as_deref(), Some("Wireless Mechanical Keyboard"));
        assert_eq!(s.amount, Some(Decimal::from_str("129.99").unwrap()));
        assert_eq!(s.delivery_status.as_deref(), Some("Processing"));
        assert_eq!(s.tracking_number.as_deref(), Some("TRK-9901423"));
    } else {
        panic!("Expected Shopping extraction");
    }
}

#[test]
fn test_invoice_category_and_extraction() {
    let engine = EmailEngine::new().unwrap();
    let email = make_test_email(
        "Invoice INV-2026-8812 from CloudScale Networks: $420.50 due 2026-11-20",
        "Please find attached Invoice INV-2026-8812 from CloudScale Networks. Total amount due: $420.50 USD. Payment due date: 2026-11-20. Issuer: CloudScale Networks.",
    );

    let processed = engine.process(&email);
    assert_eq!(processed.primary_category, Category::Invoice);

    if let ExtractedData::Invoice(i) = processed.extraction {
        assert_eq!(i.invoice_number.as_deref(), Some("INV-2026-8812"));
        assert_eq!(i.amount, Some(Decimal::from_str("420.50").unwrap()));
        assert_eq!(i.currency.as_deref(), Some("USD"));
        assert_eq!(i.due_date.as_deref(), Some("2026-11-20"));
        assert_eq!(i.issuer.as_deref(), Some("CloudScale Networks"));
    } else {
        panic!("Expected Invoice extraction");
    }

    assert_eq!(
        processed.preview,
        "Invoice received · $420.50 due 2026-11-20"
    );
}

#[test]
fn test_subscription_category_and_extraction() {
    let engine = EmailEngine::new().unwrap();
    let email = make_test_email(
        "Your SoundWave Premium Subscription Renews on 2026-11-28",
        "Your monthly subscription to SoundWave Premium ($9.99/month) will renew on 2026-11-28. Provider: SoundWave Music. Amount: $9.99. Frequency: monthly. You can manage or cancel anytime in your account settings.",
    );

    let processed = engine.process(&email);
    assert_eq!(processed.primary_category, Category::Subscription);

    if let ExtractedData::Subscription(s) = processed.extraction {
        assert_eq!(s.provider.as_deref(), Some("SoundWave Music"));
        assert_eq!(
            s.subscription_amount,
            Some(Decimal::from_str("9.99").unwrap())
        );
        assert_eq!(s.billing_frequency.as_deref(), Some("monthly"));
        assert_eq!(s.renewal_date.as_deref(), Some("2026-11-28"));
    } else {
        panic!("Expected Subscription extraction");
    }

    assert_eq!(
        processed.preview,
        "SoundWave Music · monthly renewal on 2026-11-28"
    );
}

#[test]
fn test_travel_category_and_extraction() {
    let engine = EmailEngine::new().unwrap();
    let email = make_test_email(
        "Flight Booking Confirmation: PNR SK-78921 to Tokyo (HND)",
        "Thank you for booking with SkyWay Airlines. Booking ID: SK-78921. Destination: Tokyo (HND). Departure date: 2026-12-05. Return date: 2026-12-18. Booking status: Confirmed.",
    );

    let processed = engine.process(&email);
    assert_eq!(processed.primary_category, Category::Travel);

    if let ExtractedData::Travel(t) = processed.extraction {
        assert_eq!(t.booking_id.as_deref(), Some("SK-78921"));
        assert_eq!(t.destination.as_deref(), Some("Tokyo (HND)"));
        assert_eq!(t.departure_date.as_deref(), Some("2026-12-05"));
        assert_eq!(t.return_date.as_deref(), Some("2026-12-18"));
        assert_eq!(t.booking_status.as_deref(), Some("Confirmed"));
    } else {
        panic!("Expected Travel extraction");
    }

    assert_eq!(
        processed.preview,
        "Booking to Tokyo (HND) · Confirmed for 2026-12-05"
    );
}

#[test]
fn test_newsletter_category_and_extraction() {
    let engine = EmailEngine::new().unwrap();
    let email = make_test_email(
        "Rust Weekly Issue #512: Next-Gen Compiler Performance",
        "Welcome to Issue #512. Publisher: Rust Weekly Digest. Main topic: Systems Programming. Headline: Rust 2024 Edition stabilized and async closures ready. Unsubscribe anytime.",
    );

    let processed = engine.process(&email);
    assert_eq!(processed.primary_category, Category::Newsletter);

    if let ExtractedData::Newsletter(n) = processed.extraction {
        assert_eq!(n.publisher.as_deref(), Some("Rust Weekly Digest"));
        assert_eq!(n.main_topic.as_deref(), Some("Systems Programming"));
        assert_eq!(
            n.headline.as_deref(),
            Some("Rust 2024 Edition stabilized and async closures ready")
        );
    } else {
        panic!("Expected Newsletter extraction");
    }
}

#[test]
fn test_social_category_and_extraction() {
    let engine = EmailEngine::new().unwrap();
    let email = make_test_email(
        "Alex Vance sent you an invitation to connect on ProConnect",
        "Alex Vance wants to connect with you. Platform: ProConnect. Activity: Connection request. Person: Alex Vance.",
    );

    let processed = engine.process(&email);
    assert_eq!(processed.primary_category, Category::Social);

    if let ExtractedData::Social(s) = processed.extraction {
        assert_eq!(s.platform.as_deref(), Some("ProConnect"));
        assert_eq!(s.activity_type.as_deref(), Some("Connection request"));
        assert_eq!(s.relevant_entity.as_deref(), Some("Alex Vance"));
    } else {
        panic!("Expected Social extraction");
    }

    assert_eq!(
        processed.preview,
        "ProConnect · Connection request from Alex Vance"
    );
}

#[test]
fn test_general_category_and_preview() {
    let engine = EmailEngine::new().unwrap();
    let email = make_test_email(
        "Quick coffee tomorrow morning?",
        "Hey! Are you free to grab coffee tomorrow around 9:30 AM near the office? Let me know if that works.",
    );

    let processed = engine.process(&email);
    assert_eq!(processed.primary_category, Category::General);
    assert!(processed.preview.contains("coffee tomorrow around 9:30 AM"));
}

#[test]
fn test_custom_engine_builder_and_new_regex_rules() {
    use relay_rover::engine::EngineBuilder;

    // Caller adds a custom regex rule mapping "Project Alpha Deployment" to Job or custom category
    let custom_engine = EngineBuilder::new()
        .with_detection_pattern(r"(?i)\bProject Alpha Deployment\b", Category::Job)
        .build()
        .expect("Custom engine builds successfully");

    let email = make_test_email(
        "Project Alpha Deployment Ready",
        "We are pleased to inform that Project Alpha Deployment passed all gates.",
    );

    let processed = custom_engine.process(&email);
    assert_eq!(processed.primary_category, Category::Job);
}

#[test]
fn test_process_raw_convenience_api() {
    let engine = EmailEngine::new().unwrap();
    let processed = engine.process_raw(
        "Security Alert: 991823 is your login code",
        "Enter 991823 to authenticate to your CloudPortal account. Valid for 5 minutes.",
    );

    assert_eq!(processed.primary_category, Category::Otp);
    assert!(processed.preview.contains("991823"));
}

#[test]
fn test_distractors_and_edge_cases() {
    let engine = EmailEngine::new().unwrap();

    // Distractor 1: Random phone numbers or 6-digit order numbers should NOT trigger OTP
    let email1 = make_test_email(
        "Question about meeting at 5pm",
        "Hi, can you call me at 892104 or on my mobile +1-555-0199? Also my office zip code is 94103.",
    );
    let p1 = engine.process(&email1);
    assert_ne!(p1.primary_category, Category::Otp);
    assert_eq!(p1.primary_category, Category::General);

    // Distractor 2: General email mentioning a number should not be a payment
    let email2 = make_test_email(
        "Slide deck layout",
        "I added 5 new slides and changed 3 diagrams.",
    );
    let p2 = engine.process(&email2);
    assert_eq!(p2.primary_category, Category::General);
}

#[test]
fn test_relay_rover_aliases_and_builder() {
    use relay_rover::{Rover, RoverBuilder, RoverEngine};

    // Test Rover alias
    let rover: Rover = Rover::new().expect("Rover::new() should succeed");
    let res = rover.process_raw(
        "Payment Successful: USD 500.00 to Apex Labs",
        "Payment of USD 500.00 was processed successfully for invoice INV-1002.",
    );
    assert_eq!(res.primary_category, Category::Payment);

    // Test RoverEngine alias
    let rover_engine: RoverEngine = RoverEngine::new().expect("RoverEngine::new() should succeed");
    let res2 = rover_engine.process_raw(
        "Your OTP is 112233",
        "Login OTP 112233 expires in 5 minutes",
    );
    assert_eq!(res2.primary_category, Category::Otp);

    // Test RoverBuilder alias with custom rule
    let custom_rover = RoverBuilder::new()
        .with_detection_pattern(r"(?i)\bRelease Sprint 42\b", Category::Job)
        .build()
        .expect("RoverBuilder should build custom Rover instance");
    let res3 = custom_rover.process_raw("Release Sprint 42 Ready", "All tasks done.");
    assert_eq!(res3.primary_category, Category::Job);
}

#[test]
fn test_single_and_multi_threaded_execution_modes() {
    use relay_rover::{ExecutionMode, Rover, RoverBuilder};

    // 1. By default, Rover is single-threaded
    let default_rover = Rover::new().expect("default Rover inits");
    assert_eq!(
        default_rover.execution_mode(),
        ExecutionMode::SingleThreaded
    );
    assert_eq!(default_rover.num_threads(), 1);
    assert!(!default_rover.is_multi_threaded());

    // 2. User chooses multi-threaded mode with custom thread count (e.g. 4 threads)
    let multi_rover = RoverBuilder::new()
        .with_multi_threaded(Some(4))
        .build()
        .expect("multi-threaded Rover inits");
    assert_eq!(
        multi_rover.execution_mode(),
        ExecutionMode::MultiThreaded { num_threads: 4 }
    );
    assert_eq!(multi_rover.num_threads(), 4);
    assert!(multi_rover.is_multi_threaded());

    // 3. User chooses multi-threaded mode with default core count (None)
    let default_cores_rover = RoverBuilder::new()
        .with_multi_threaded(None)
        .build()
        .expect("default core multi-threaded Rover inits");
    assert!(default_cores_rover.is_multi_threaded());
    assert!(default_cores_rover.num_threads() >= 1);

    // 4. User uses dynamic with_threading(multithread: bool, threads: Option<usize>)
    let single_via_toggle = RoverBuilder::new()
        .with_threading(false, Some(8)) // multithread is false -> uses single-threaded
        .build()
        .expect("single-threaded via toggle inits");
    assert_eq!(
        single_via_toggle.execution_mode(),
        ExecutionMode::SingleThreaded
    );
    assert_eq!(single_via_toggle.num_threads(), 1);

    let multi_via_toggle = RoverBuilder::new()
        .with_threading(true, Some(3))
        .build()
        .expect("multi-threaded via toggle inits");
    assert_eq!(multi_via_toggle.num_threads(), 3);
    assert!(multi_via_toggle.is_multi_threaded());

    // 5. Test batch processing in both single and multi-threaded engines
    let batch = vec![
        make_test_email(
            "Transaction Alert: INR 2,500.00 debited from A/C XX4912",
            "Payment to QuickMart Retail. Txn Ref: TXN-89210492. Status: Successful.",
        ),
        make_test_email(
            "Your verification code is 123456",
            "Acme Portal verification code 123456 expires in 5 minutes",
        ),
        make_test_email(
            "Invoice INV-1002 from CloudCorp",
            "Amount $200 due 2026-11-20. Issuer: CloudCorp.",
        ),
    ];

    let single_results = default_rover.process_batch(&batch);
    let multi_results = multi_rover.process_batch(&batch);

    assert_eq!(single_results.len(), 3);
    assert_eq!(multi_results.len(), 3);
    assert_eq!(single_results[0].primary_category, Category::Payment);
    assert_eq!(multi_results[0].primary_category, Category::Payment);
    assert_eq!(single_results[1].primary_category, Category::Otp);
    assert_eq!(multi_results[1].primary_category, Category::Otp);
    assert_eq!(single_results[2].primary_category, Category::Invoice);
    assert_eq!(multi_results[2].primary_category, Category::Invoice);
}
