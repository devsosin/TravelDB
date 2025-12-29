pub struct NewAgentReport {
    article_id: i64,
    report_type: String,
    content: String,
}

impl NewAgentReport {
    pub fn new(article_id: i64, report_type: &str, content: &str) -> Self {
        Self {
            article_id,
            report_type: report_type.into(),
            content: content.into(),
        }
    }

    pub fn get_article_id(&self) -> i64 {
        self.article_id
    }
    pub fn get_report_type(&self) -> &str {
        &self.report_type
    }
    pub fn get_content(&self) -> &str {
        &self.content
    }
}
