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
    pub fn get_descriptions(&self) -> Vec<String> {
        self.articles
            .iter()
            .map(|a| a.description.clone())
            .collect()
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
    description: String,
    link: String,
    writer: String,
    writed_at: DateTime<Utc>,
}

impl NewArticle {
    pub fn new(
        article_id: &str,
        title: &str,
        description: &str,
        link: &str,
        writer: &str,
        writed_at: DateTime<Utc>,
    ) -> Self {
        Self {
            article_id: article_id.into(),
            title: title.into(),
            description: description.into(),
            link: link.into(),
            writer: writer.into(),
            writed_at,
        }
    }
}

pub struct NewArticleDetail {
    article_id: i64,
    content: String,
    hashtags: String,
    likes: i32,
    comments: i32,
}

impl NewArticleDetail {
    pub fn new(article_id: i64, content: &str, hashtags: &str, likes: i32, comments: i32) -> Self {
        Self {
            article_id,
            content: content.into(),
            hashtags: hashtags.into(),
            likes,
            comments,
        }
    }

    pub fn get_article_id(&self) -> i64 {
        self.article_id
    }
    pub fn get_content(&self) -> &str {
        &self.content
    }
    pub fn get_hashtags(&self) -> &str {
        &self.hashtags
    }
    pub fn get_likes(&self) -> i32 {
        self.likes
    }
    pub fn get_comments(&self) -> i32 {
        self.comments
    }
}

pub struct NewArticleRelavance {
    article_id: i64,
    is_related: bool,
    continent: Option<String>,
    country: Option<String>,
    city: Option<String>,
}

impl NewArticleRelavance {
    pub fn new(
        article_id: i64,
        is_related: bool,
        continent: Option<String>,
        country: Option<String>,
        city: Option<String>,
    ) -> Self {
        Self {
            article_id,
            is_related,
            continent,
            country,
            city,
        }
    }

    pub fn get_article_id(&self) -> i64 {
        self.article_id
    }
    pub fn get_is_related(&self) -> bool {
        self.is_related
    }
    pub fn get_continent(&self) -> &Option<String> {
        &self.continent
    }
    pub fn get_country(&self) -> &Option<String> {
        &self.country
    }
    pub fn get_city(&self) -> &Option<String> {
        &self.city
    }
}
