// filename: registry.rs

pub mod service_instance;
use std::collections::HashMap;

pub use service_instance::ServiceInstance;

/// Service registry:
/// A hash table, it's element key is service name, element value is a collection of service instances.
pub struct Registry {
    services: HashMap<String, Vec<ServiceInstance>>,
}

/// Implement Default trait for Registry
///
impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

/// Implement Clone trait for Registry
impl Clone for Registry {
    fn clone(&self) -> Self {
        let mut cloned_services = HashMap::new();

        for (name, instances) in &self.services {
            let cloned_instances = instances.to_vec();
            cloned_services.insert(name.clone(), cloned_instances);
        }

        Registry {
            services: cloned_services,
        }
    }
}

/// Implement Registry
///
impl Registry {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    /// Register a service instance
    ///
    #[allow(unused)]
    pub fn register(&mut self, instance: ServiceInstance) {
        // 先检查是否存在
        if let Some(service) = self.services.get_mut(&instance.name) {
            if service.contains(&instance) {
                // TODO:: Check if ip address and port is same, it is same service, update service metadata
                return;
            }
        }
        self.services
            .entry(instance.name.clone())
            .or_insert_with(Vec::new)
            .push(instance);
    }

    /// Query service instances by service name
    ///
    pub fn query(&self, name: &str) -> Vec<ServiceInstance> {
        match self.services.get(name) {
            Some(instances) => instances.to_vec(),
            None => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = Registry::new();
        let instance = ServiceInstance::new("foo".to_string(), "http://foo.com".to_string());

        registry.register(instance.clone());

        assert_eq!(registry.query("foo"), vec![instance]);
    }

    #[test]
    fn test_register_duplicate() {
        let mut registry = Registry::new();

        let instance = ServiceInstance::new("bar".to_string(), "http://bar.com".to_string());
        registry.register(instance.clone());
        registry.register(instance.clone());

        assert_eq!(registry.query("bar").len(), 1);
    }

    #[test]
    fn test_query_not_found() {
        let registry = Registry::new();
        assert!(registry.query("baz").is_empty());
    }

    #[test]
    fn test_register_multiple() {
        let mut registry = Registry::new();

        registry.register(ServiceInstance::new(
            "svc".to_string(),
            "http://svc1.com".to_string(),
        ));
        registry.register(ServiceInstance::new(
            "svc".to_string(),
            "http://svc2.com".to_string(),
        ));

        assert_eq!(registry.query("svc").len(), 2);
    }

    #[test]
    fn test_query_empty() {
        let registry = Registry::new();

        assert!(registry.query("none").is_empty());
    }

    #[test]
    fn test_different_services() {
        let mut registry = Registry::new();

        registry.register(ServiceInstance::new(
            "svc1".to_string(),
            "http://svc1.com".to_string(),
        ));
        registry.register(ServiceInstance::new(
            "svc2".to_string(),
            "http://svc2.com".to_string(),
        ));

        assert_eq!(registry.query("svc1").len(), 1);
        assert_eq!(registry.query("svc2").len(), 1);
    }
}
