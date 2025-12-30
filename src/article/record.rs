use chrono::{DateTime, Utc};

pub struct ArticleSummaryRecord {
    pub id: i64,
    pub title: Option<String>,
    pub description: Option<String>,
}

pub struct ArticleInfoRecord {
    pub id: Option<i64>,
    pub article_id: Option<String>,
    pub writer: Option<String>,
    pub writed_at: Option<DateTime<Utc>>,
}

pub struct ArticleDetailRecord {
    pub id: i64,
    pub title: Option<String>,
    pub content: Option<String>,
}
