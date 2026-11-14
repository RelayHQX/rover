pub mod general;
pub mod invoice;
pub mod job;
pub mod newsletter;
pub mod otp;
pub mod payment;
pub mod security;
pub mod shopping;
pub mod social;
pub mod subscription;
pub mod travel;

use crate::models::{Category, EmailRecord, ExtractedData};

pub use general::GeneralFilter;
pub use invoice::InvoiceFilter;
pub use job::JobFilter;
pub use newsletter::NewsletterFilter;
pub use otp::OtpFilter;
pub use payment::PaymentFilter;
pub use security::SecurityFilter;
pub use shopping::ShoppingFilter;
pub use social::SocialFilter;
pub use subscription::SubscriptionFilter;
pub use travel::TravelFilter;

/// Uniform interface for category-specific filters.
pub trait EmailFilter: Send + Sync {
    fn category(&self) -> Category;
    fn detection_patterns(&self) -> &[&'static str];
    fn extract(&self, email: &EmailRecord) -> ExtractedData;
    fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String;
}

impl EmailFilter for PaymentFilter {
    fn category(&self) -> Category {
        self.category()
    }
    fn detection_patterns(&self) -> &[&'static str] {
        self.detection_patterns()
    }
    fn extract(&self, email: &EmailRecord) -> ExtractedData {
        self.extract(email)
    }
    fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String {
        self.generate_preview(email, data)
    }
}

impl EmailFilter for OtpFilter {
    fn category(&self) -> Category {
        self.category()
    }
    fn detection_patterns(&self) -> &[&'static str] {
        self.detection_patterns()
    }
    fn extract(&self, email: &EmailRecord) -> ExtractedData {
        self.extract(email)
    }
    fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String {
        self.generate_preview(email, data)
    }
}

impl EmailFilter for JobFilter {
    fn category(&self) -> Category {
        self.category()
    }
    fn detection_patterns(&self) -> &[&'static str] {
        self.detection_patterns()
    }
    fn extract(&self, email: &EmailRecord) -> ExtractedData {
        self.extract(email)
    }
    fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String {
        self.generate_preview(email, data)
    }
}

impl EmailFilter for ShoppingFilter {
    fn category(&self) -> Category {
        self.category()
    }
    fn detection_patterns(&self) -> &[&'static str] {
        self.detection_patterns()
    }
    fn extract(&self, email: &EmailRecord) -> ExtractedData {
        self.extract(email)
    }
    fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String {
        self.generate_preview(email, data)
    }
}

impl EmailFilter for InvoiceFilter {
    fn category(&self) -> Category {
        self.category()
    }
    fn detection_patterns(&self) -> &[&'static str] {
        self.detection_patterns()
    }
    fn extract(&self, email: &EmailRecord) -> ExtractedData {
        self.extract(email)
    }
    fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String {
        self.generate_preview(email, data)
    }
}

impl EmailFilter for SubscriptionFilter {
    fn category(&self) -> Category {
        self.category()
    }
    fn detection_patterns(&self) -> &[&'static str] {
        self.detection_patterns()
    }
    fn extract(&self, email: &EmailRecord) -> ExtractedData {
        self.extract(email)
    }
    fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String {
        self.generate_preview(email, data)
    }
}

impl EmailFilter for TravelFilter {
    fn category(&self) -> Category {
        self.category()
    }
    fn detection_patterns(&self) -> &[&'static str] {
        self.detection_patterns()
    }
    fn extract(&self, email: &EmailRecord) -> ExtractedData {
        self.extract(email)
    }
    fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String {
        self.generate_preview(email, data)
    }
}

impl EmailFilter for NewsletterFilter {
    fn category(&self) -> Category {
        self.category()
    }
    fn detection_patterns(&self) -> &[&'static str] {
        self.detection_patterns()
    }
    fn extract(&self, email: &EmailRecord) -> ExtractedData {
        self.extract(email)
    }
    fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String {
        self.generate_preview(email, data)
    }
}

impl EmailFilter for SocialFilter {
    fn category(&self) -> Category {
        self.category()
    }
    fn detection_patterns(&self) -> &[&'static str] {
        self.detection_patterns()
    }
    fn extract(&self, email: &EmailRecord) -> ExtractedData {
        self.extract(email)
    }
    fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String {
        self.generate_preview(email, data)
    }
}

impl EmailFilter for SecurityFilter {
    fn category(&self) -> Category {
        self.category()
    }
    fn detection_patterns(&self) -> &[&'static str] {
        self.detection_patterns()
    }
    fn extract(&self, email: &EmailRecord) -> ExtractedData {
        self.extract(email)
    }
    fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String {
        self.generate_preview(email, data)
    }
}

impl EmailFilter for GeneralFilter {
    fn category(&self) -> Category {
        self.category()
    }
    fn detection_patterns(&self) -> &[&'static str] {
        self.detection_patterns()
    }
    fn extract(&self, email: &EmailRecord) -> ExtractedData {
        self.extract(email)
    }
    fn generate_preview(&self, email: &EmailRecord, data: &ExtractedData) -> String {
        self.generate_preview(email, data)
    }
}
