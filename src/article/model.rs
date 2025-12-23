use chrono::{DateTime, Utc};

pub struct NewArticleList {
    platform: String,
    keyword: String,
    articles: Vec<NewArticle>,
}

impl NewArticleList {
    pub fn new(platform: &str, keyword: &str, articles: Vec<NewArticle>) -> Self {
        Self {
            platform: platform.into(),
            keyword: keyword.into(),
            articles,
        }
    }

    pub fn get_platform(&self) -> &str {
        &self.platform
    }
    pub fn get_keyword(&self) -> &str {
        &self.keyword
    }

    pub fn get_article_ids(&self) -> Vec<String> {
        self.articles.iter().map(|a| a.article_id.clone()).collect()
    }
    pub fn get_titles(&self) -> Vec<String> {
        self.articles.iter().map(|a| a.title.clone()).collect()
    }
    pub fn get_links(&self) -> Vec<String> {
        self.articles.iter().map(|a| a.link.clone()).collect()
    }
    pub fn get_writers(&self) -> Vec<String> {
        self.articles.iter().map(|a| a.writer.clone()).collect()
    }
    pub fn get_writed_ats(&self) -> Vec<DateTime<Utc>> {
        self.articles.iter().map(|a| a.writed_at).collect()
    }
}

pub struct NewArticle {
    article_id: String,
    title: String,
    link: String,
    writer: String,
    writed_at: DateTime<Utc>,
}

impl NewArticle {
    pub fn new(
        article_id: &str,
        title: &str,
        link: &str,
        writer: &str,
        writed_at: DateTime<Utc>,
    ) -> Self {
        Self {
            article_id: article_id.into(),
            title: title.into(),
            link: link.into(),
            writer: writer.into(),
            writed_at,
        }
    }
}

pub struct NewArticleDetail {
    article_id: i64,
    content: String,
}

impl NewArticleDetail {
    pub fn new(article_id: i64, content: &str) -> Self {
        Self {
            article_id,
            content: content.into(),
        }
    }

    pub fn get_article_id(&self) -> i64 {
        self.article_id
    }
    pub fn get_content(&self) -> &str {
        &self.content
    }
}
