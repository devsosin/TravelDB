pub mod entity;
pub mod model;
pub mod subs;

use sqlx::PgPool;

use crate::{RepositoryResult, metadata::model::NewMetadata};

pub struct MetaRepositoryImpl {
    pool: PgPool,
}

impl MetaRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

pub trait MetaRepository {
    fn save(
        &self,
        article_id: i64,
        title: &str,
        new_metadata: NewMetadata,
    ) -> impl Future<Output = RepositoryResult<i64>>;
}

impl MetaRepository for MetaRepositoryImpl {
    async fn save(
        &self,
        article_id: i64,
        title: &str,
        new_metadata: NewMetadata,
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
            INSERT INTO tb_metadata(article_id, title, continent, country_id, city_id, 
                    post_type, companion, duration, budget_level, best_season, has_budget, keywords) 
                VALUES ($1, $2, $3, (SELECT id FROM SelectedCountry), (SELECT id FROM SelectedCity), 
                    $6, $7, $8, $9, $10, $11, $12) 
            RETURNING id
            "#,
            article_id,
            title,
            new_metadata.continent,
            new_metadata.country,
            new_metadata.city,
            &new_metadata.post_type,
            new_metadata.companion,
            new_metadata.duration,
            new_metadata.budget_level,
            new_metadata.best_season,
            new_metadata.has_budget,
            &new_metadata.keywords as _,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(record.id)
    }
}
