use std::{env, time::Duration};

use sqlx::{PgPool, postgres::PgPoolOptions};

pub struct DatabaseServerConfig {
    host: String,
    port: String,
    user: String,
    password: String,
    database: String,
}

impl DatabaseServerConfig {
    pub fn from_env() -> Self {
        let host = env::var("DB_HOST").expect("Failed to get env variable");
        let port = env::var("DB_PORT").expect("Failed to get env variable");
        let user = env::var("DB_USER").expect("Failed to get env variable");
        let password = env::var("DB_PASSWORD").expect("Failed to get env variable");
        let database = env::var("DB_DATABASE").expect("Failed to get env variable");

        Self {
            host,
            port,
            user,
            password,
            database,
        }
    }

    pub async fn get_pool(&self) -> PgPool {
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database
        );

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&database_url)
            .await
            .expect("Unable to connect to database");

        pool
    }
}
