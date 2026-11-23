pub mod benchmark;
pub mod engine;
pub mod extractors;
pub mod filters;
pub mod models;
pub mod preview;
pub mod registry;
pub mod wasm;

// Public high-level crate re-exports for easy integration
pub use engine::{EmailEngine, EngineBuilder, ExecutionMode, Rover, RoverBuilder, RoverEngine};
pub use filters::EmailFilter;
pub use models::{
    Category, EmailRecord, ExtractedData, GeneralExtraction, InvoiceExtraction, JobExtraction,
    NewsletterExtraction, OtpExtraction, PaymentExtraction, ProcessedEmail, ShoppingExtraction,
    SocialExtraction, SubscriptionExtraction, TravelExtraction,
};
pub use registry::{InitTimings, RegexRegistry, RegistryBuilder};
