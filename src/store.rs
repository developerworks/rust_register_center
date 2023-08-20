// filename: store.rs

use std::collections::HashMap;

#[derive(serde::Serialize)]
pub enum ServiceConfigValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    None,
}
impl Clone for ServiceConfigValue {
    fn clone(&self) -> Self {
        match self {
            Self::String(v) => Self::String(v.clone()),
            Self::Int(v) => Self::Int(*v),
            Self::Float(v) => Self::Float(*v),
            Self::Bool(v) => Self::Bool(*v),
            Self::None => Self::None,
        }
    }
}

pub struct Store {
    pub configs: HashMap<String, HashMap<String, ServiceConfigValue>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
        }
    }

    // 获取配置值
    pub fn get(&self, service: &str, key: &str) -> Option<ServiceConfigValue> {
        self.configs
            .get(service)
            .and_then(|config: &HashMap<String, ServiceConfigValue>| config.get(key).cloned())
    }

    /// 保存配置值
    #[allow(unused)]
    pub fn set(&mut self, service: &str, key: &str, value: ServiceConfigValue) {
        self.configs
            .entry(service.to_owned())
            .or_insert_with(HashMap::new)
            .insert(key.to_owned(), value);
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}
