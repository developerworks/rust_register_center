// filename: mysql.rs

use sqlx::{MySql, Pool, MySqlConnection, Error};
use std::env;

pub struct MysqlStore {
    pool: Pool<MySql>,
}

impl MysqlStore {
    pub async fn new() -> Result<Self, Error> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = Pool::new(&database_url).await?;
        Ok(Self { pool })
    }

    pub async fn get(&self, service: &str, key: &str) -> Result<Option<ServiceConfigValue>, Error> {
        let mut conn = self.pool.acquire().await?;
        let result = sqlx::query!(
            r#"SELECT value FROM service_config WHERE service = ? AND key = ?"#,
            service,
            key
        )
        .fetch_optional(&mut conn)
        .await?;

        match result {
            Some(row) => {
                let value: String = row.value;
                let config_value = serde_json::from_str(&value)?;
                Ok(Some(config_value))
            }
            None => Ok(None),
        }
    }

    pub async fn set(&self, service: &str, key: &str, value: ServiceConfigValue) -> Result<(), Error> {
        let mut conn = self.pool.acquire().await?;
        let value_str = serde_json::to_string(&value)?;
        sqlx::query!(
            r#"INSERT INTO service_config (service, key, value) VALUES (?, ?, ?) ON DUPLICATE KEY UPDATE value = ?"#,
            service,
            key,
            value_str,
            value_str
        )
        .execute(&mut conn)
        .await?;
        Ok(())
    }
}
