pub mod entity;

use sqlx::PgPool;

use crate::{RepositoryResult, platform::entity::Platform};

pub struct PlatformRepositoryImpl {
    pool: PgPool,
}

impl PlatformRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

pub trait PlatformRepository: Send {
    fn save(&self, name: &str) -> impl Future<Output = RepositoryResult<i16>>;
    fn find_by_name(&self, name: &str) -> impl Future<Output = RepositoryResult<Platform>>;
}

impl PlatformRepository for PlatformRepositoryImpl {
    async fn save(&self, name: &str) -> RepositoryResult<i16> {
        let record = sqlx::query!(
            "INSERT INTO tb_platform(name) VALUES($1) RETURNING id",
            name
        )
        .fetch_one(&self.pool)
        .await?;

        let id = record.id;
        Ok(id)
    }

    async fn find_by_name(&self, name: &str) -> RepositoryResult<Platform> {
        let platform = sqlx::query_as!(Platform, "SELECT * FROM tb_platform WHERE name = $1", name)
            .fetch_one(&self.pool)
            .await?;

        Ok(platform)
    }
}
