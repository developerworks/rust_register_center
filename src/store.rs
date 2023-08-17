// filename: store.rs

use std::collections::HashMap;

pub struct Store {
    configs: HashMap<String, HashMap<String, String>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
        }
    }

    // 获取配置值
    pub fn get(&self, service: &str, key: &str) -> Option<String> {
        self.configs
            .get(service)
            .and_then(|svc_conf| svc_conf.get(key).cloned())
    }

    // 保存配置值
    #[allow(unused)]
    pub fn set(&mut self, service: &str, key: &str, value: String) {
        self.configs
            .entry(service.to_owned())
            .or_insert_with(HashMap::new)
            .insert(key.to_owned(), value);
    }
}

#[cfg(test)]
mod tests {

    use super::Store;

    #[test]
    fn get_non_existing() {
        let store = Store::new();
        assert_eq!(store.get("svc", "key"), None);
    }

    #[test]
    fn get_existing() {
        let mut store = Store::new();
        store.set("svc", "key", "value".into());
        assert_eq!(store.get("svc", "key"), Some("value".into()));
    }

    #[test]
    fn override_key() {
        let mut store = Store::new();
        store.set("svc", "key", "value1".into());
        store.set("svc", "key", "value2".into());
        assert_eq!(store.get("svc", "key"), Some("value2".into()));
    }

    #[test]
    fn different_services() {
        let mut store = Store::new();
        store.set("svc1", "key1", "value1".into());
        store.set("svc2", "key2", "value2".into());
        assert_eq!(store.get("svc1", "key1"), Some("value1".into()));
        assert_eq!(store.get("svc2", "key2"), Some("value2".into()));
    }

    #[test]
    fn multiple_keys() {
        let mut store = Store::new();
        store.set("svc", "key1", "value1".into());
        store.set("svc", "key2", "value2".into());

        assert_eq!(store.get("svc", "key1"), Some("value1".into()));
        assert_eq!(store.get("svc", "key2"), Some("value2".into()));
    }

    #[test]
    fn override_default() {
        let mut store = Store::new();

        store.set("svc", "key", "default".into());
        assert_eq!(store.get("svc", "key"), Some("default".into()));

        store.set("svc", "key", "newvalue".into());
        assert_eq!(store.get("svc", "key"), Some("newvalue".into()));
    }
}
