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

