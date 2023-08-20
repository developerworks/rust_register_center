// filename: redis.rs

use redis::{Client, Commands, Connection, RedisResult};
use std::env;

pub struct RedisStore {
    conn: Connection,
}

impl RedisStore {
    pub fn new() -> RedisResult<Self> {
        dotenv::dotenv().ok(); // Load the .env file
        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
        let client = Client::open(redis_url)?;
        let conn = client.get_connection()?;
        Ok(Self { conn })
    }

    pub fn get(&self, key: &str) -> RedisResult<Option<String>> {
        let result: Option<String> = self.conn.get(key)?;
        Ok(result)
    }

    pub fn set(&self, key: &str, value: &str) -> RedisResult<()> {
        self.conn.set(key, value)?;
        Ok(())
    }

    pub fn delete(&self, key: &str) -> RedisResult<()> {
        self.conn.del(key)?;
        Ok(())
    }
}
