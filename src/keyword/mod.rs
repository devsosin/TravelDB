pub mod entity;

use sqlx::PgPool;

use crate::{RepositoryResult, keyword::entity::Keyword};

pub struct KeywordRepositoryImpl {
    pool: PgPool,
}

impl KeywordRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

pub trait KeywordRepository: Send {
    fn save(&self, query: &str) -> impl Future<Output = RepositoryResult<i32>>;
    fn find_by_query(&self, query: &str) -> impl Future<Output = RepositoryResult<Keyword>>;
}

impl KeywordRepository for KeywordRepositoryImpl {
    async fn save(&self, query: &str) -> RepositoryResult<i32> {
        let record = sqlx::query!(
            "INSERT INTO tb_keyword(query) VALUES($1) RETURNING id",
            query
        )
        .fetch_one(&self.pool)
        .await?;

        let id = record.id;
        Ok(id)
    }
    async fn find_by_query(&self, query: &str) -> RepositoryResult<Keyword> {
        let keyword = sqlx::query_as!(Keyword, "SELECT * FROM tb_keyword WHERE query = $1", query)
            .fetch_one(&self.pool)
            .await?;

        Ok(keyword)
    }
}
