use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Config {
    /// The ip address to bind when service registry server started.
    pub host: String,
    /// The listen port of service registry server
    pub port: u16,
    /// The sql database configuration
    pub database: Database,
    /// Redis configuration
    pub redis: Redis,
}

/// The default value of config
impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 2312,
            database: Database {
                url: "mysql://root:root@127.0.0.1:3306/test".to_string(),
                user: "root".to_string(),
                password: "root".to_string(),
                db: "test".to_string(),
            },
            redis: Redis {
                host: "127.0.0.1".to_string(),
                port: 6379,
                db: 0,
                password: "".to_string(),
            },
        }
    }
}
/// The database configuration for store service information
#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
pub struct Database {
    /// The connection string of database
    pub url: String,
    /// User name
    pub user: String,
    /// Password
    pub password: String,
    /// Database name
    pub db: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
pub struct Redis {
    /// The redis server ip address listening
    pub host: String,
    /// The listen port of redis server
    pub port: u16,
    /// Database number
    pub db: u16,
    /// Login password
    pub password: String,
}
