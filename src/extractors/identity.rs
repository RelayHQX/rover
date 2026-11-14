use regex::Regex;

/// Extractor for companies, recipients, issuers, providers, and persons.
pub struct IdentityExtractor {
    paid_to: Regex,
    receipt_from: Regex,
    issuer: Regex,
    provider: Regex,
    store: Regex,
    social_actor: Regex,
}

impl IdentityExtractor {
    pub fn new() -> Result<Self, regex::Error> {
        // "payment to QuickMart Retail", "paid to: Tokyo Books Inc", "Paid ₹850 to Chai Point on",
        // "transferred $220.00 to Elena Rostova", "disbursed $8450.00 USD to Nexus Systems"
        let paid_to = Regex::new(
            r"(?i)\b(?:payment to|paid to|transferred to|recipient|paid\s+[\$€£₹\w\d.,\s]+?\s+to|sent a payment of [^\n]+ to|transferred [^\n]+ to|disbursed [^\n]+ to|payment of [^\n]+ to|from Merchant)\s*[:]?\s*([A-Za-z0-9&.'\- ]{2,35}?)(?:\s+on|\s+was|\s+for|\s+at|\s+with|\.|\,|\;|\n|Txn|Ref|Status|Date|Transfer|Payment|Mandate)",
        )?;

        // "receipt from Blue Bottle Coffee: $6.75"
        let receipt_from = Regex::new(
            r"(?i)\b(?:digital receipt from|receipt from)\s+([A-Za-z0-9&.'\- ]{2,35}?)(?:\:|\.|\,|\;|\n|$)",
        )?;

        // "Issuer: CloudScale Networks", "Invoice from CloudScale Networks"
        let issuer = Regex::new(
            r"(?i)\b(?:Issuer\s*[:]\s*|Invoice(?:\s+[A-Za-z0-9\-#]+)?\s+from\s+)([A-Za-z0-9&.'\- ]{2,35}?)(?:\s+for|\.|\,|\;|\n|$)",
        )?;

        // "Provider: SoundWave Music", "subscription to SoundWave Premium"
        let provider = Regex::new(
            r"(?i)\b(?:Provider\s*[:]\s*|subscription to\s+)([A-Za-z0-9&.'\- ]{2,35}?)(?:\s+renews|\.|\,|\;|\n|$)",
        )?;

        // "from ShopQuick", "at Nordic Apparel", "from BookHaven", "ordering with KitchenCraft Supplies"
        let store = Regex::new(
            r"(?i)\b(?:shopping at|delivery from|package from|order from|order at|order with|ordering with|purchase from)\s+([A-Za-z0-9&.'\- ]{2,35}?)(?:\s+on|\s+is|\s+has|\s+will|\.|\,|\;|\n|!|$)",
        )?;

        // "Alex Vance sent you an invitation", "Sarah Jenkins reviewed pull request", "David Miller commented"
        let social_actor = Regex::new(
            r"(?i)\b([A-Za-z0-9_'\- ]{2,30}?)\s+(?:sent you|mentioned you|reviewed pull request|commented on|liked your|reacted to|shared a post)\b",
        )?;

        Ok(Self {
            paid_to,
            receipt_from,
            issuer,
            provider,
            store,
            social_actor,
        })
    }

    pub fn extract_paid_to(&self, text: &str) -> Option<String> {
        let extracted = self
            .paid_to
            .captures(text)
            .or_else(|| self.receipt_from.captures(text))
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()));

        // Filter out generic bank tokens mistakenly captured as merchant names
        if let Some(name) = &extracted
            && (name.starts_with("account ") || name.starts_with("IBAN"))
        {
            return None;
        }

        extracted
    }

    pub fn extract_issuer(&self, text: &str) -> Option<String> {
        self.issuer
            .captures(text)
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
    }

    pub fn extract_provider(&self, text: &str) -> Option<String> {
        self.provider
            .captures(text)
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
    }

    pub fn extract_store(&self, text: &str) -> Option<String> {
        self.store
            .captures(text)
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
    }

    pub fn extract_social_actor(&self, text: &str) -> Option<String> {
        self.social_actor
            .captures(text)
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
    }
}
