pub mod entity;
pub mod model;

use sqlx::PgPool;

use crate::{
    RepositoryResult,
    article::model::{NewArticleDetail, NewArticleList},
};

pub struct ArticleRepositoryImpl {
    pool: PgPool,
}

impl ArticleRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

pub trait ArticleRepository: Send {
    fn save_many(
        &self,
        new_article_list: NewArticleList,
    ) -> impl Future<Output = RepositoryResult<Vec<i64>>>;
    fn save_detail(
        &self,
        new_article_detail: NewArticleDetail,
    ) -> impl Future<Output = RepositoryResult<i64>>;

    // find_no_details
}

impl ArticleRepository for ArticleRepositoryImpl {
    async fn save_many(&self, new_article_list: NewArticleList) -> RepositoryResult<Vec<i64>> {
        let records = sqlx::query!(
            r#"
            WITH TargetPlatform AS (
                SELECT id AS platform_id FROM tb_platform WHERE name = $1
            ),
            TargetKeyword AS (
                SELECT id AS keyword_id
                FROM tb_keyword
                WHERE query = $2
            ),
            ArticleDatas AS (
                SELECT article_id, title, description, link, writer, writed_at
                FROM UNNEST($3::text[], $4::text[], $5::text[], $6::text[], $7::text[], $8::timestamptz[])
                    AS u(article_id, title, description, link, writer, writed_at)
            )
            INSERT INTO tb_article(platform_id, keyword_id, article_id, title, description, link, writer, writed_at)
                SELECT platform_id, keyword_id, article_id, title, description, link, writer, writed_at
                FROM ArticleDatas
                    CROSS JOIN TargetPlatform
                    CROSS JOIN TargetKeyword
                RETURNING id
            "#,
            new_article_list.get_platform(),
            new_article_list.get_keyword(),
            &new_article_list.get_article_ids(),
            &new_article_list.get_titles(),
            &new_article_list.get_descriptions(),
            &new_article_list.get_links(),
            &new_article_list.get_writers(),
            &new_article_list.get_writed_ats(),
        )
        .fetch_all(&self.pool)
        .await?;

        let ids = records.iter().map(|r| r.id).collect();

        Ok(ids)
    }

    async fn save_detail(&self, new_article_detail: NewArticleDetail) -> RepositoryResult<i64> {
        let record = sqlx::query!(
            "INSERT INTO tb_article_detail(article_id, content) VALUES($1, $2) RETURNING id",
            new_article_detail.get_article_id(),
            new_article_detail.get_content()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(record.id)
    }
}
