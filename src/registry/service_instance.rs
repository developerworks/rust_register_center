// filename: service_instance.rs
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ServiceInstance {
    /// Service name
    pub name: String,
    /// Service url, for example a rest api server: https://example.com/api
    pub url: String,
    /// Other information about the service
    pub metadata: HashMap<String, String>,
}

impl ServiceInstance {

    #[allow(unused)]
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url,
            metadata: HashMap::new(),
        }
    }

    #[allow(unused)]
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
