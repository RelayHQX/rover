use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// High-level purpose category classification (Blog stage 4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Payment,
    Otp,
    Job,
    Shopping,
    Invoice,
    Subscription,
    Travel,
    Newsletter,
    Social,
    SecurityAlert,
    General,
}

impl Category {
    /// Return the canonical display name of the category.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Payment => "Payment",
            Self::Otp => "OTP",
            Self::Job => "Jobs",
            Self::Shopping => "Shopping",
            Self::Invoice => "Invoices",
            Self::Subscription => "Subscriptions",
            Self::Travel => "Travel",
            Self::Newsletter => "Newsletters",
            Self::Social => "Social",
            Self::SecurityAlert => "Security Alerts",
            Self::General => "General",
        }
    }

    /// Return all specialized categories in priority order (excluding General fallback).
    #[must_use]
    pub const fn all_specialized() -> &'static [Category] {
        &[
            Category::Otp,
            Category::SecurityAlert,
            Category::Payment,
            Category::Invoice,
            Category::Shopping,
            Category::Subscription,
            Category::Travel,
            Category::Job,
            Category::Social,
            Category::Newsletter,
        ]
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Traditional general email category (Primary, Social, Promotions, Updates, Forums, Spam).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum GeneralCategory {
    #[default]
    Primary,
    Social,
    Promotions,
    Updates,
    Forums,
    Spam,
}

impl GeneralCategory {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Primary => "Primary",
            Self::Social => "Social",
            Self::Promotions => "Promotions",
            Self::Updates => "Updates",
            Self::Forums => "Forums",
            Self::Spam => "Spam",
        }
    }
}

/// Email urgency and importance level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Importance {
    Urgent,
    High,
    #[default]
    Normal,
    Low,
}

impl Importance {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Urgent => "Urgent",
            Self::High => "High",
            Self::Normal => "Normal",
            Self::Low => "Low",
        }
    }
}

/// Security & Authentication Classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SecurityClassification {
    Authentication,
    PasswordReset,
    SecurityAlert,
    AccountChange,
    BuildFailure,
    ActionRequired,
    #[default]
    None,
}

impl SecurityClassification {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Authentication => "Authentication",
            Self::PasswordReset => "Password Reset",
            Self::SecurityAlert => "Security Alert",
            Self::AccountChange => "Account Change",
            Self::BuildFailure => "Build Failure",
            Self::ActionRequired => "Action Required",
            Self::None => "None",
        }
    }
}

/// Traceable confidence factors backing detection decisions (Blog Section: Why Confidence Matters).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConfidenceFactors {
    pub authenticated_sender: bool,
    pub has_structured_entities: bool,
    pub pattern_specificity: String,
    pub subject_body_aligned: bool,
}

/// Rich classification metadata reflecting Rover's dual-layer intelligence.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmailClassification {
    /// Traditional general folder category (Primary, Social, Promotions, Updates, Forums)
    pub general_category: GeneralCategory,
    /// Purpose-based category (Deliveries, Transactions, Authentication, Jobs, etc.)
    pub purpose: Category,
    /// Importance level (Urgent, High, Normal, Low)
    pub importance: Importance,
    /// Whether email requires an alert banner / instant notification
    pub is_alert: bool,
    /// Security classification
    pub security: SecurityClassification,
    /// Calibrated confidence score (0.00 to 1.00)
    pub confidence: f32,
    /// Factors supporting the confidence calculation
    pub factors: ConfidenceFactors,
    /// Secondary matched categories
    pub secondary_purposes: Vec<Category>,
}

impl Default for EmailClassification {
    fn default() -> Self {
        Self {
            general_category: GeneralCategory::Primary,
            purpose: Category::General,
            importance: Importance::Normal,
            is_alert: false,
            security: SecurityClassification::None,
            confidence: 0.50,
            factors: ConfidenceFactors::default(),
            secondary_purposes: Vec::new(),
        }
    }
}

/// Raw incoming email record loaded from storage/network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailRecord {
    pub id: String,
    pub sender: String,
    pub sender_name: String,
    pub subject: String,
    pub received_at: String,
    pub body: String,
    /// Target category used strictly for benchmark accuracy verification.
    /// Engine inference never reads this field!
    pub expected_category: Category,
    /// Target extraction fields used strictly for benchmark accuracy verification.
    /// Engine inference never reads this field!
    pub expected_extraction: serde_json::Value,
}

impl EmailRecord {
    /// Compute total raw byte size of the email for throughput calculations.
    #[must_use]
    pub fn byte_len(&self) -> usize {
        self.sender.len() + self.sender_name.len() + self.subject.len() + self.body.len()
    }
}

// -----------------------------------------------------------------------------
// Typed Information Extraction Models
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentExtraction {
    pub amount: Option<Decimal>,
    pub currency: Option<String>,
    pub payer: Option<String>,
    pub recipient: Option<String>,
    pub transaction_id: Option<String>,
    pub transaction_date: Option<String>,
    pub transaction_time: Option<String>,
    pub payment_method: Option<String>,
    pub transaction_status: Option<String>,
    pub timeline_key: Option<String>,
    pub raw_evidence: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OtpExtraction {
    pub otp_code: Option<String>,
    pub service_name: Option<String>,
    pub expiration_time: Option<String>,
    pub auth_purpose: Option<String>,
    pub action_url: Option<String>,
    pub raw_evidence: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobExtraction {
    pub company: Option<String>,
    pub position: Option<String>,
    pub location: Option<String>,
    pub salary: Option<String>,
    pub job_count: Option<u32>,
    pub is_job_alert: bool,
    pub job_url: Option<String>,
    pub application_deadline: Option<String>,
    pub interview_date: Option<String>,
    pub raw_evidence: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShoppingExtraction {
    pub store: Option<String>,
    pub order_id: Option<String>,
    pub product: Option<String>,
    pub amount: Option<Decimal>,
    pub currency: Option<String>,
    pub tax: Option<Decimal>,
    pub order_date: Option<String>,
    pub delivery_status: Option<String>,
    pub tracking_number: Option<String>,
    pub courier: Option<String>,
    pub payment_method: Option<String>,
    pub timeline_key: Option<String>,
    pub raw_evidence: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvoiceExtraction {
    pub invoice_number: Option<String>,
    pub amount: Option<Decimal>,
    pub currency: Option<String>,
    pub due_date: Option<String>,
    pub issuer: Option<String>,
    pub timeline_key: Option<String>,
    pub raw_evidence: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriptionExtraction {
    pub provider: Option<String>,
    pub subscription_amount: Option<Decimal>,
    pub billing_frequency: Option<String>,
    pub renewal_date: Option<String>,
    pub cancellation_info: Option<String>,
    pub raw_evidence: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TravelExtraction {
    pub booking_id: Option<String>,
    pub destination: Option<String>,
    pub departure_date: Option<String>,
    pub return_date: Option<String>,
    pub booking_status: Option<String>,
    pub timeline_key: Option<String>,
    pub raw_evidence: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewsletterExtraction {
    pub publisher: Option<String>,
    pub main_topic: Option<String>,
    pub headline: Option<String>,
    pub raw_evidence: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SocialExtraction {
    pub platform: Option<String>,
    pub activity_type: Option<String>,
    pub relevant_entity: Option<String>,
    pub actor: Option<String>,
    pub post_url: Option<String>,
    pub raw_evidence: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecurityAlertExtraction {
    pub alert_type: Option<String>,
    pub target_resource: Option<String>,
    pub severity: Option<String>,
    pub action_required: Option<String>,
    pub timeline_key: Option<String>,
    pub raw_evidence: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneralExtraction {
    pub summary: Option<String>,
    pub sender_display: Option<String>,
    pub raw_evidence: Option<String>,
}

/// Unified typed container for category-specific extractions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "category", content = "data")]
pub enum ExtractedData {
    Payment(PaymentExtraction),
    Otp(OtpExtraction),
    Job(JobExtraction),
    Shopping(ShoppingExtraction),
    Invoice(InvoiceExtraction),
    Subscription(SubscriptionExtraction),
    Travel(TravelExtraction),
    Newsletter(NewsletterExtraction),
    Social(SocialExtraction),
    SecurityAlert(SecurityAlertExtraction),
    General(GeneralExtraction),
}

impl ExtractedData {
    /// Return the category this extraction corresponds to.
    #[must_use]
    pub const fn category(&self) -> Category {
        match self {
            Self::Payment(_) => Category::Payment,
            Self::Otp(_) => Category::Otp,
            Self::Job(_) => Category::Job,
            Self::Shopping(_) => Category::Shopping,
            Self::Invoice(_) => Category::Invoice,
            Self::Subscription(_) => Category::Subscription,
            Self::Travel(_) => Category::Travel,
            Self::Newsletter(_) => Category::Newsletter,
            Self::Social(_) => Category::Social,
            Self::SecurityAlert(_) => Category::SecurityAlert,
            Self::General(_) => Category::General,
        }
    }

    /// Extract universal timeline grouping key if applicable.
    #[must_use]
    pub fn timeline_key(&self) -> Option<String> {
        match self {
            Self::Payment(p) => p
                .timeline_key
                .clone()
                .or_else(|| p.transaction_id.as_ref().map(|id| format!("payment:{id}"))),
            Self::Shopping(s) => s
                .timeline_key
                .clone()
                .or_else(|| s.order_id.as_ref().map(|id| format!("order:{id}"))),
            Self::Invoice(i) => i
                .timeline_key
                .clone()
                .or_else(|| i.invoice_number.as_ref().map(|id| format!("invoice:{id}"))),
            Self::Travel(t) => t
                .timeline_key
                .clone()
                .or_else(|| t.booking_id.as_ref().map(|id| format!("travel:{id}"))),
            Self::SecurityAlert(s) => s.timeline_key.clone(),
            _ => None,
        }
    }
}

/// Final deterministic outcome of processing an email.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedEmail {
    pub id: String,
    pub primary_category: Category,
    pub matched_categories: Vec<Category>,
    pub classification: EmailClassification,
    pub extraction: ExtractedData,
    pub preview: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline_key: Option<String>,
}
