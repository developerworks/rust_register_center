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
    configs: HashMap<String, HashMap<String, ServiceConfigValue>>,
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
            .and_then(|config| config.get(key).cloned())
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
        store.set("service", "key", ServiceConfigValue::None);

        assert!(matches!(
            store.get("service", "key").unwrap(),
            ServiceConfigValue::None
        ));
    }

    #[test]
    fn test_store_get_not_found() {
        let store = Store::new();
        let result  = matches!(store.get("service", "key"), None);
        assert!(result);
    }

    #[test]
    fn test_store_set_and_get() {
        let mut store = Store::new();
        store.set("service", "key", ServiceConfigValue::String("value".to_owned()));

        match store.get("service", "key").unwrap() {
            ServiceConfigValue::String(v) => assert_eq!(v, "value"),
            _ => panic!("Unexpected config value"),
        }
    }

    #[test]
    fn test_store_get_multiple_services() {
        let mut store = Store::new();
        store.set("service1", "key1", ServiceConfigValue::String("value1".to_owned()));
        store.set("service2", "key2", ServiceConfigValue::String("value2".to_owned()));
        store.set("service1", "number", ServiceConfigValue::Float(1.0));
        store.set("service1", "integer", ServiceConfigValue::Int(1));
        store.set("service1", "boolean", ServiceConfigValue::Bool(false));
        store.set("service1", "none", ServiceConfigValue::None);

        match store.get("service1", "key1").unwrap() {
            ServiceConfigValue::String(v) => assert_eq!(v, "value1"),
            ServiceConfigValue::Int(_) => todo!(),
            ServiceConfigValue::Float(_) => todo!(),
            ServiceConfigValue::Bool(_) => todo!(),
            ServiceConfigValue::None => todo!(),
        }
        match store.get("service2", "key2").unwrap() {
            ServiceConfigValue::String(v) => assert_eq!(v, "value2"),
            ServiceConfigValue::Int(_) => todo!(),
            ServiceConfigValue::Float(_) => todo!(),
            ServiceConfigValue::Bool(_) => todo!(),
            ServiceConfigValue::None => todo!(),
        }
        match store.get("service1", "number").unwrap() {
            ServiceConfigValue::String(_) => todo!(),
            ServiceConfigValue::Int(_) => todo!(),
            ServiceConfigValue::Float(v) => assert_eq!(v, 1.0),
            ServiceConfigValue::Bool(_) => todo!(),
            ServiceConfigValue::None => todo!(),
        }

        match store.get("service1", "integer").unwrap() {
            ServiceConfigValue::Float(_) => todo!(),
            ServiceConfigValue::Int(v) => assert_eq!(v, 1),
            ServiceConfigValue::String(_) => todo!(),
            ServiceConfigValue::Bool(_) => todo!(),
            ServiceConfigValue::None => todo!(),
        }

        match store.get("service1", "boolean").unwrap() {
            ServiceConfigValue::Float(_) => todo!(),
            ServiceConfigValue::Int(_) => todo!(),
            ServiceConfigValue::String(_) => todo!(),
            ServiceConfigValue::Bool(v) => assert!(!v),
            ServiceConfigValue::None => todo!(),
        }
    }
}
