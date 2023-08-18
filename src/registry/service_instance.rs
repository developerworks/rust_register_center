// filename: service_instance.rs
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ServiceInstance {
    pub name: String,
    pub url: String,
    pub metadata: HashMap<String, String>,
}

impl ServiceInstance {

    #[allow(dead_code)]
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url,
            metadata: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_metadata(self, metadata: HashMap<String, String>) -> Self {
        Self {
            name: self.name,
            url: self.url,
            metadata,
        }
    }
}

impl Clone for ServiceInstance {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            url: self.url.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let instance = ServiceInstance::new("foo".to_string(), "http://foo.com".to_string());
        assert_eq!(instance.name, "foo");
        assert_eq!(instance.url, "http://foo.com");
        assert!(instance.metadata.is_empty());
    }

    #[test]
    fn test_with_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), "1.0".to_string());

        let instance = ServiceInstance::new("bar".to_string(), "http://bar.com".to_string());
        let instance = instance.with_metadata(metadata);
        
        assert_eq!(instance.metadata["version"], "1.0");
    }

    #[test]
    fn test_clone() {
        let instance = ServiceInstance::new("baz".to_string(), "http://baz.com".to_string());
        let cloned = instance.clone();

        assert_eq!(instance.name, cloned.name);
        assert_eq!(instance.url, cloned.url);
        assert_eq!(instance.metadata, cloned.metadata);
    }
}