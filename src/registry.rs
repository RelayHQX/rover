use regex::RegexSet;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, Copy)]
struct Instant;

#[cfg(target_arch = "wasm32")]
impl Instant {
    #[inline]
    fn now() -> Self {
        Self
    }
    #[inline]
    fn elapsed(&self) -> Duration {
        Duration::ZERO
    }
}

use crate::filters::{
    EmailFilter, GeneralFilter, InvoiceFilter, JobFilter, NewsletterFilter, OtpFilter,
    PaymentFilter, SecurityFilter, ShoppingFilter, SocialFilter, SubscriptionFilter, TravelFilter,
};
use crate::models::Category;

#[derive(Debug, Clone, Copy)]
pub struct InitTimings {
    pub regex_set_compilation: Duration,
    pub extraction_compilation: Duration,
    pub total_initialization: Duration,
}

/// Central compiled regex registry powering category detection and extraction dispatch.
pub struct RegexRegistry {
    regex_set: RegexSet,
    pattern_categories: Vec<Category>,
    filters: HashMap<Category, Box<dyn EmailFilter>>,
    timings: InitTimings,
}

impl RegexRegistry {
    /// Create a builder to configure default or custom filters and regex rules.
    #[must_use]
    pub fn builder() -> RegistryBuilder {
        RegistryBuilder::new()
    }

    /// Initialize default registry with all 10 standard category filters.
    pub fn init() -> Result<Self, anyhow::Error> {
        Self::builder().build()
    }

    /// Access the pre-measured initialization timings.
    #[must_use]
    pub fn timings(&self) -> &InitTimings {
        &self.timings
    }

    /// Total number of detection patterns compiled into RegexSet.
    #[must_use]
    pub fn detection_pattern_count(&self) -> usize {
        self.pattern_categories.len()
    }

    /// Execute single-pass RegexSet detection against input text.
    /// Returns all matched categories in order of priority.
    /// If no specialized pattern matches, falls back to `Category::General`.
    pub fn detect_categories(&self, search_text: &str) -> Vec<Category> {
        let matches = self.regex_set.matches(search_text);

        if !matches.matched_any() {
            return vec![Category::General];
        }

        let mut matched_cats = Vec::new();
        for index in matches.into_iter() {
            if let Some(&cat) = self.pattern_categories.get(index)
                && !matched_cats.contains(&cat)
            {
                matched_cats.push(cat);
            }
        }

        if matched_cats.is_empty() {
            vec![Category::General]
        } else {
            matched_cats
        }
    }

    /// Retrieve the filter for a specific category.
    #[must_use]
    pub fn get_filter(&self, category: Category) -> Option<&dyn EmailFilter> {
        self.filters.get(&category).map(|f| f.as_ref())
    }
}

/// Builder allowing users to add, override, or extend regex filters and custom patterns.
pub struct RegistryBuilder {
    filters: HashMap<Category, Box<dyn EmailFilter>>,
    custom_patterns: Vec<(&'static str, Category)>,
}

impl Default for RegistryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl RegistryBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            filters: HashMap::new(),
            custom_patterns: Vec::new(),
        }
    }

    /// Register or replace a filter implementation for a given category.
    #[must_use]
    pub fn register_filter<F: EmailFilter + 'static>(mut self, filter: F) -> Self {
        self.filters.insert(filter.category(), Box::new(filter));
        self
    }

    /// Add an extra detection regex pattern mapped to a specific category.
    #[must_use]
    pub fn add_detection_pattern(mut self, pattern: &'static str, category: Category) -> Self {
        self.custom_patterns.push((pattern, category));
        self
    }

    /// Build the compiled `RegexRegistry`. Missing default filters will be filled automatically.
    pub fn build(mut self) -> Result<RegexRegistry, anyhow::Error> {
        let total_start = Instant::now();
        let extraction_start = Instant::now();

        // Populate default filters if not overridden by caller
        if let Entry::Vacant(e) = self.filters.entry(Category::Payment) {
            e.insert(Box::new(PaymentFilter::new()?));
        }
        if let Entry::Vacant(e) = self.filters.entry(Category::Otp) {
            e.insert(Box::new(OtpFilter::new()?));
        }
        if let Entry::Vacant(e) = self.filters.entry(Category::Job) {
            e.insert(Box::new(JobFilter::new()?));
        }
        if let Entry::Vacant(e) = self.filters.entry(Category::Shopping) {
            e.insert(Box::new(ShoppingFilter::new()?));
        }
        if let Entry::Vacant(e) = self.filters.entry(Category::Invoice) {
            e.insert(Box::new(InvoiceFilter::new()?));
        }
        if let Entry::Vacant(e) = self.filters.entry(Category::Subscription) {
            e.insert(Box::new(SubscriptionFilter::new()?));
        }
        if let Entry::Vacant(e) = self.filters.entry(Category::Travel) {
            e.insert(Box::new(TravelFilter::new()?));
        }
        if let Entry::Vacant(e) = self.filters.entry(Category::Newsletter) {
            e.insert(Box::new(NewsletterFilter::new()?));
        }
        if let Entry::Vacant(e) = self.filters.entry(Category::Social) {
            e.insert(Box::new(SocialFilter::new()?));
        }
        if let Entry::Vacant(e) = self.filters.entry(Category::SecurityAlert) {
            e.insert(Box::new(SecurityFilter::new()?));
        }
        self.filters
            .entry(Category::General)
            .or_insert_with(|| Box::new(GeneralFilter::new()));

        let extraction_compilation = extraction_start.elapsed();

        // Compile unified RegexSet
        let regex_set_start = Instant::now();
        let mut patterns = Vec::new();
        let mut pattern_categories = Vec::new();

        // 1. Register filter detection patterns in priority order
        for category in Category::all_specialized() {
            if let Some(filter) = self.filters.get(category) {
                for &pattern in filter.detection_patterns() {
                    patterns.push(pattern);
                    pattern_categories.push(*category);
                }
            }
        }

        // 2. Register custom user-provided patterns
        for (pattern, category) in self.custom_patterns {
            patterns.push(pattern);
            pattern_categories.push(category);
        }

        let regex_set = RegexSet::new(&patterns)?;
        let regex_set_compilation = regex_set_start.elapsed();
        let total_initialization = total_start.elapsed();

        let timings = InitTimings {
            regex_set_compilation,
            extraction_compilation,
            total_initialization,
        };

        Ok(RegexRegistry {
            regex_set,
            pattern_categories,
            filters: self.filters,
            timings,
        })
    }
}
