pub mod entity;
pub mod model;
pub mod record;

use sqlx::PgPool;

use crate::{
    RepositoryResult,
    article::{
        model::{NewArticleDetail, NewArticleList, NewArticleRelavance},
        record::{ArticleDetailRecord, ArticleInfoRecord, ArticleSummaryRecord},
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
    fn update_article(&self, article_id: i64) -> impl Future<Output = RepositoryResult<()>>;

    fn find_no_relavnce(&self)
    -> impl Future<Output = RepositoryResult<Vec<ArticleSummaryRecord>>>;
    fn save_relavance(
        &self,
        new_article_relavance: NewArticleRelavance,
    ) -> impl Future<Output = RepositoryResult<i64>>;
    fn find_no_detail(&self) -> impl Future<Output = RepositoryResult<Vec<ArticleInfoRecord>>>;
    fn save_quality(
        &self,
        article_id: i64,
        quality: f32,
    ) -> impl Future<Output = RepositoryResult<i64>>;
    fn find_detail_with_no_metadata(
        &self,
    ) -> impl Future<Output = RepositoryResult<Vec<ArticleDetailRecord>>>;
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
            "INSERT INTO tb_article_detail(article_id, content, hashtags, likes, comments) VALUES($1, $2, $3, $4, $5) RETURNING id",
            new_article_detail.get_article_id(),
            new_article_detail.get_content(),
            new_article_detail.get_hashtags(),
            new_article_detail.get_likes(),
            new_article_detail.get_comments(),
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
            r#"
            WITH SelectedCountry AS (
                SELECT id
                FROM tb_country
                WHERE name = $4
            ),
            SelectedCity AS (
                SELECT id
                FROM tb_city
                WHERE name = $5
            )
            INSERT INTO tb_article_relavance (article_id, is_related, continent, country_id, city_id) 
                VALUES ($1, $2, $3, (SELECT id FROM SelectedCountry), (SELECT id FROM SelectedCity))
            RETURNING id
            "#,
            new_article_relavance.get_article_id(),
            new_article_relavance.get_is_related(),
            new_article_relavance.get_continent() as _,
            new_article_relavance.get_country() as _,
            new_article_relavance.get_city() as _,


        )
        .fetch_one(&self.pool)
        .await?;

        Ok(record.id)
    }

    // do not using (quality check)
    async fn find_no_relavnce(&self) -> RepositoryResult<Vec<ArticleSummaryRecord>> {
        let articles = sqlx::query_as!(
            ArticleSummaryRecord,
            r#"
            SELECT a.id, title, description 
            FROM tb_article AS a
                LEFT JOIN tb_article_relavance AS r ON a.id = r.article_id
            WHERE r.id IS NULL
            ORDER BY writed_at ASC
            LIMIT 1000
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(articles)
    }

    async fn find_no_detail(&self) -> RepositoryResult<Vec<ArticleInfoRecord>> {
        let articles = sqlx::query_as!(
            ArticleInfoRecord,
            r#"
            SELECT a.id, title, article_id, writer, writed_at, k.query
            FROM tb_article AS a
                JOIN tb_keyword AS k ON a.keyword_id = k.id
            WHERE has_detail=FALSE
            ORDER BY id ASC 
            LIMIT 20
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(articles)
    }
    async fn update_article(&self, article_id: i64) -> RepositoryResult<()> {
        sqlx::query!(
            "UPDATE tb_article SET has_detail=TRUE WHERE id=$1",
            article_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn save_quality(&self, article_id: i64, quality: f32) -> RepositoryResult<i64> {
        let record = sqlx::query!(
            "INSERT INTO tb_article_quality(article_id, quality) VALUES($1, $2) RETURNING id",
            article_id,
            quality,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(record.id)
    }

    async fn find_detail_with_no_metadata(&self) -> RepositoryResult<Vec<ArticleDetailRecord>> {
        // after quality check
        let articles = sqlx::query_as!(
            ArticleDetailRecord,
            r#"
            SELECT a.id, a.title, d.content
            FROM (
                SELECT d.article_id, d.content
                FROM tb_article_detail AS d
                    LEFT JOIN tb_metadata AS m ON d.article_id = m.article_id
                WHERE m.id IS NULL
                LIMIT 20
            ) AS d
                JOIN tb_article AS a ON a.id = d.article_id
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(articles)
    }
}
