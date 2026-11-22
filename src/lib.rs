#![allow(unsafe_attr_outside_unsafe)]

pub mod benchmark;
pub mod engine;
pub mod extractors;
pub mod ffi;
pub mod filters;
pub mod models;
pub mod preview;
pub mod registry;
pub mod wasm;

// High-level UniFFI functions in crate root
pub use ffi::{
    classify, clean_email_body, generate_preview, html_to_text, process_batch_json, process_email,
    process_email_json,
};

uniffi::include_scaffolding!("rover");

// Public high-level crate re-exports for easy integration
pub use engine::{EmailEngine, EngineBuilder, ExecutionMode, Rover, RoverBuilder, RoverEngine};
pub use filters::EmailFilter;
pub use models::{
    Category, EmailRecord, ExtractedData, GeneralExtraction, InvoiceExtraction, JobExtraction,
    NewsletterExtraction, OtpExtraction, PaymentExtraction, ProcessedEmail, ShoppingExtraction,
    SocialExtraction, SubscriptionExtraction, TravelExtraction,
};
pub use registry::{InitTimings, RegexRegistry, RegistryBuilder};
