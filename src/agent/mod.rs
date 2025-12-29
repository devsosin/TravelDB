pub mod model;

use sqlx::PgPool;

use crate::{RepositoryResult, agent::model::NewAgentReport};

pub struct AgentRepositoryImpl {
    pool: PgPool,
}

impl AgentRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn save(&self, new_agent_report: NewAgentReport) -> RepositoryResult<()> {
        sqlx::query!(
            "INSERT INTO tb_agent_report(article_id, report_type, content) VALUES ($1, $2, $3)",
            new_agent_report.get_article_id(),
            new_agent_report.get_report_type(),
            new_agent_report.get_content(),
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
