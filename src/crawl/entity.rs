use chrono::{DateTime, Utc};

pub struct CrawlChecker {
    pub crawled_at: DateTime<Utc>,
}

pub struct CrawlDetailChecker {
    pub writed_at: DateTime<Utc>,
}
