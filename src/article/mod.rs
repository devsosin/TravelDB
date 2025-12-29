pub mod entity;
pub mod model;
pub mod record;

use sqlx::PgPool;

use crate::{
    RepositoryResult,
    article::{
        model::{NewArticleDetail, NewArticleList, NewArticleRelavance},
        record::{ArticleLinkRecord, ArticleSummaryRecord},
    },
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

    fn find_no_relavnce(&self)
    -> impl Future<Output = RepositoryResult<Vec<ArticleSummaryRecord>>>;
    fn find_related_but_no_detail(
        &self,
    ) -> impl Future<Output = RepositoryResult<Vec<ArticleLinkRecord>>>;
    fn save_relavance(
        &self,
        new_article_relavance: NewArticleRelavance,
    ) -> impl Future<Output = RepositoryResult<i64>>;
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

    async fn save_relavance(
        &self,
        new_article_relavance: NewArticleRelavance,
    ) -> RepositoryResult<i64> {
        let record = sqlx::query!(
            "INSERT INTO tb_article_relavance(article_id, is_related) VALUES($1, $2) RETURNING id",
            new_article_relavance.get_article_id(),
            new_article_relavance.get_is_related()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(record.id)
    }

    async fn find_no_relavnce(&self) -> RepositoryResult<Vec<ArticleSummaryRecord>> {
        let articles = sqlx::query_as!(
            ArticleSummaryRecord,
            r#"
            SELECT a.id, title, description 
            FROM tb_article AS a
                LEFT JOIN tb_article_relavance AS r ON a.id = r.article_id
            WHERE r.id IS NULL
            ORDER BY writed_at ASC
            LIMIT 20
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(articles)
    }

    async fn find_related_but_no_detail(&self) -> RepositoryResult<Vec<ArticleLinkRecord>> {
        let articles = sqlx::query_as!(
            ArticleLinkRecord,
            r#"
            SELECT a.id, link
            FROM tb_article AS a
                JOIN tb_article_relavance AS r ON a.id = r.article_id AND r.is_related = TRUE
                LEFT JOIN tb_article_detail AS d ON a.id = d.article_id
            WHERE d.id IS NULL
            ORDER BY writed_at ASC
            LIMIT 20
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(articles)
    }
}
