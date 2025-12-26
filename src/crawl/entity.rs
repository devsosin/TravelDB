use chrono::{DateTime, Utc};

pub struct CrawlChecker {
    pub crawled_at: DateTime<Utc>,
}
