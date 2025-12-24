pub mod entity;

use sqlx::PgPool;

use crate::{RepositoryResult, country::entity::Country};

pub struct CountryRepositoryImpl {
    pool: PgPool,
}

impl CountryRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

pub trait CountryRepository: Send {
    fn find_all(&self) -> impl Future<Output = RepositoryResult<Vec<Country>>>;
}

impl CountryRepository for CountryRepositoryImpl {
    async fn find_all(&self) -> RepositoryResult<Vec<Country>> {
        let countires = sqlx::query_as!(Country, "SELECT * FROM tb_country")
            .fetch_all(&self.pool)
            .await?;

        Ok(countires)
    }
}
