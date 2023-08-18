use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database: Database,
    pub redis: Redis,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            database: Database {
                url: "mysql://root:root@127.0.0.1:3306/test".to_string(),
                user: "root".to_string(),
                password: "root".to_string(),
                db: "test".to_string(),
            },
            redis: Redis {
                host: "127.0.0.1".to_string(),
                port: 6379,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Database {
    pub url: String,
    pub user: String,
    pub password: String,
    pub db: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Redis {
    pub host: String,
    pub port: u16,
}
