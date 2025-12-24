pub mod entity;

use sqlx::PgPool;

use crate::{RepositoryResult, city::entity::City};

pub struct CityRepositoryImpl {
    pool: PgPool,
}

impl CityRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

pub trait CityRepository: Send {
    fn find_all(&self) -> impl Future<Output = RepositoryResult<Vec<City>>>;
}

impl CityRepository for CityRepositoryImpl {
    async fn find_all(&self) -> RepositoryResult<Vec<City>> {
        let cities = sqlx::query_as!(City, "SELECT * FROM tb_city")
            .fetch_all(&self.pool)
            .await?;

        Ok(cities)
    }
}
