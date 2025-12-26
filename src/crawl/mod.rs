pub mod entity;

use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::{RepositoryResult, crawl::entity::CrawlChecker};

pub struct CrawlCheckRepositoryImpl {
    pool: PgPool,
}

impl CrawlCheckRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn update(&self) -> RepositoryResult<()> {
        sqlx::query!("UPDATE tb_crawl_checker SET crawled_at = NOW()")
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_last_crawl(&self) -> RepositoryResult<DateTime<Utc>> {
        let crawl_checker = sqlx::query_as!(CrawlChecker, "SELECT * FROM tb_crawl_checker")
            .fetch_one(&self.pool)
            .await?;

        Ok(crawl_checker.crawled_at)
    }
}
