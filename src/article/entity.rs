use chrono::{DateTime, Utc};

pub struct Article {
    pub id: i64,

    pub article_id: String,
    pub title: String,
    pub description: String,
    pub link: String,
    pub writer: String,
    pub writed_at: Option<DateTime<Utc>>,

    pub has_detail: bool,

    pub platform_id: Option<i16>,
    pub keyword_id: Option<i32>,
}

pub struct ArticleDetail {
    pub id: i64,

    pub content: String,
    pub hashtags: String,
    pub likes: i32,
    pub comments: i32,

    pub article_id: Option<i64>,
}

pub struct ArticleRelavance {
    pub id: i64,
    pub article_id: Option<i64>,
    pub is_related: bool,
}

pub struct ArticleQuality {
    pub id: i64,
    pub article_id: Option<i64>,
    pub quality: Option<f32>,
}
