pub struct ArticleSummaryRecord {
    pub id: i64,
    pub title: Option<String>,
    pub description: Option<String>,
}

pub struct ArticleLinkRecord {
    pub id: i64,
    pub link: String,
}
