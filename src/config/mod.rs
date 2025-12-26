use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();
        
        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://localhost/crud_test".to_string()),
            server_host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid port number"))?,
        })
    }
    
    pub async fn create_db_pool(&self) -> anyhow::Result<PgPool> {
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&self.database_url)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))
    }
} 