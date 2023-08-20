// filename: mysql.rs

use std::env;

use sqlx::{mysql::MySqlPoolOptions, MySql, MySqlPool, Pool};

use super::ServiceConfigValue;

pub struct MysqlStore {
    pub pool: Pool<sqlx::MySql>,
}

#[derive(Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct MicroService {
    pub name: String,
    pub url: String,
}

#[allow(unused)]
impl MysqlStore {
    pub async fn new() -> Result<Self, anyhow::Result<()>> {
        dotenv::dotenv().ok(); // Load the .env file
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .unwrap();
        Ok(Self { pool })
    }

    pub async fn set(
        &self,
        service: &str,
        key: &str,
        value: &ServiceConfigValue,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let value_str = serde_json::to_string(value)?;
        sqlx::query(
            "INSERT INTO service_config (service, key, value) VALUES (?, ?, ?)
            ON DUPLICATE KEY UPDATE value = ?",
        )
        .bind(service)
        .bind(key)
        .bind(&value_str)
        .bind(&value_str)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get(
        &self,
        name: &str,
        url: &str,
        // ) -> Result<Option<ServiceConfigValue>, Box<dyn std::error::Error>> {
    ) -> Vec<MicroService> {
        let stream = sqlx::query_as::<MySql, MicroService>(
            r#"SELECT name,url FROM service_config WHERE name = ? AND url = ?"#,
        )
        .bind(name)
        .bind(url)
        .fetch_all(&self.pool)
        .await;

        let a = stream
            .unwrap()
            .iter()
            .map(|x: &MicroService| MicroService {
                name: x.name.clone(),
                url: x.url.clone(),
            })
            .collect::<Vec<MicroService>>();
        println!("{:?}", a);
        a
    }

    async fn list_todos(&self, pool: &MySqlPool) -> anyhow::Result<()> {
        let recs = sqlx::query!(
            r#"
    SELECT id, description, done
    FROM todos
    ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await?;

        // NOTE: Booleans in MySQL are stored as `TINYINT(1)` / `i8`
        //       0 = false, non-0 = true
        for rec in recs {
            println!(
                "- [{}] {}: {}",
                if rec.done != 0 { "x" } else { " " },
                rec.id,
                &rec.description,
            );
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // 单元测试函数
    #[actix_rt::test]
    async fn test_get_micro_service() {
        // 创建一个内存数据库的连接池用于测试
        let pool = MySqlPoolOptions::new()
            .max_connections(2)
            .connect("mysql://root:root@localhost/test")
            .await
            .expect("Failed to create pool for testing");

        // 创建一个 MysqlStore 实例用于测试
        let store = MysqlStore { pool };

        // 调用 get 方法，传入测试用的 service 和 url
        let result = store.get("service_name_1", "service_url_1").await;

        // 预期的 MicroService 结构
        let expected_micro_service = MicroService {
            name: "service_name_1".to_string(),
            url: "service_url_1".to_string(),
        };

        // 断言返回结果是否与预期一致
        assert_eq!(result, vec![expected_micro_service]);
    }
}
