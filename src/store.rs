// filename: store.rs

use std::collections::HashMap;

#[derive(serde::Serialize)]
pub enum ConfigValue {
    /// 字符串值
    String(String),
    /// 整数值
    Int(i64),
    /// 浮点值
    Float(f64),
    /// 布尔值
    Bool(bool),
    /// 空值, 只有 Key, 没有 Value
    None,
}
impl Clone for ConfigValue {
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
    configs: HashMap<String, HashMap<String, ConfigValue>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
        }
    }

    // 获取配置值
    pub fn get(&self, service: &str, key: &str) -> Option<ConfigValue> {
        self.configs
            .get(service)
            .and_then(|config| config.get(key).cloned())
    }

    /// 保存配置值
    #[allow(unused)]
    pub fn set(&mut self, service: &str, key: &str, value: ConfigValue) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_new() {
        let store = Store::new();
        assert!(store.configs.is_empty());
    }

    #[test]
    fn test_store_set_none() {
        let mut store = Store::new();
        store.set("service", "key", ConfigValue::None);

        assert!(matches!(
            store.get("service", "key").unwrap(),
            ConfigValue::None
        ));
    }

    #[test]
    fn test_store_get_not_found() {
        let store = Store::new();

        let result = matches!(store.get("service", "key"), None);
        assert!(result);
    }

    #[test]
    fn test_store_set_and_get() {
        let mut store = Store::new();
        store.set("service", "key", ConfigValue::String("value".to_owned()));

        match store.get("service", "key").unwrap() {
            ConfigValue::String(v) => assert_eq!(v, "value"),
            _ => panic!("Unexpected config value"),
        }
    }

    #[test]
    fn test_store_get_multiple_services() {
        let mut store = Store::new();
        store.set("service1", "key1", ConfigValue::String("value1".to_owned()));
        store.set("service2", "key2", ConfigValue::String("value2".to_owned()));
        store.set("service1", "number", ConfigValue::Float(1.0));
        store.set("service1", "integer", ConfigValue::Int(1));
        store.set("service1", "boolean", ConfigValue::Bool(false));
        store.set("service1", "none", ConfigValue::None);

        match store.get("service1", "key1").unwrap() {
            ConfigValue::String(v) => assert_eq!(v, "value1"),
            ConfigValue::Int(_) => todo!(),
            ConfigValue::Float(_) => todo!(),
            ConfigValue::Bool(_) => todo!(),
            ConfigValue::None => todo!(),
        }
        match store.get("service2", "key2").unwrap() {
            ConfigValue::String(v) => assert_eq!(v, "value2"),
            ConfigValue::Int(_) => todo!(),
            ConfigValue::Float(_) => todo!(),
            ConfigValue::Bool(_) => todo!(),
            ConfigValue::None => todo!(),
        }
        match store.get("service1", "number").unwrap() {
            ConfigValue::String(_) => todo!(),
            ConfigValue::Int(_) => todo!(),
            ConfigValue::Float(v) => assert_eq!(v, 1.0),
            ConfigValue::Bool(_) => todo!(),
            ConfigValue::None => todo!(),
        }

        match store.get("service1", "integer").unwrap() {
            ConfigValue::Float(_) => todo!(),
            ConfigValue::Int(v) => assert_eq!(v, 1),
            ConfigValue::String(_) => todo!(),
            ConfigValue::Bool(_) => todo!(),
            ConfigValue::None => todo!(),
        }

        match store.get("service1", "boolean").unwrap() {
            ConfigValue::Float(_) => todo!(),
            ConfigValue::Int(_) => todo!(),
            ConfigValue::String(_) => todo!(),
            ConfigValue::Bool(v) => assert!(!v),
            ConfigValue::None => todo!(),
        }
    }
}
