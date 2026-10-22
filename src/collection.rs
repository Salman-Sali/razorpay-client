/// The response of Razorpay's list endpoints.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Collection<T> {
    pub entity: String,
    pub count: u32,
    pub items: Vec<T>,
}

/// Pagination and time range filters accepted by the list endpoints.
#[derive(serde::Serialize, Debug, Clone, Default)]
pub struct ListOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip: Option<u32>,
}

impl ListOptions {
    pub fn new() -> Self {
        Self::default()
    }

    /// Unix timestamp (seconds) from which entities are returned.
    pub fn with_from(&mut self, from: u64) {
        self.from = Some(from);
    }

    /// Unix timestamp (seconds) till which entities are returned.
    pub fn with_to(&mut self, to: u64) {
        self.to = Some(to);
    }

    /// Number of entities to fetch; Razorpay defaults to 10 and allows at most 100.
    pub fn with_count(&mut self, count: u8) {
        self.count = Some(count.min(100));
    }

    /// Number of entities to skip, for pagination.
    pub fn with_skip(&mut self, skip: u32) {
        self.skip = Some(skip);
    }
}
