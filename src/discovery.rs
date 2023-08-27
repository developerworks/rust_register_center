// filename: discovery.rs

use crate::registry::{Registry, ServiceInstance};

pub struct ServiceDiscovery {
    pub registry: Registry,
}

impl ServiceDiscovery {

    /// Initialize the service disovery with a registry
    #[allow(unused)]
    pub fn new(registry: Registry) -> Self {
        Self { registry }
    }

    /// Register a service instance with the discovery
    /// ```rust
    /// let service_discovery = ServiceDiscovery::new(Registry::new());
    /// service_discovery.register(ServiceInstance::new("service_name", "service_address", 8080));
    /// ```
    #[allow(unused)]
    pub fn register(&mut self, service_instance: ServiceInstance) {
        self.registry.register(service_instance);
    }

    #[allow(unused)]
    pub fn query(&self, name: &str) -> Vec<ServiceInstance> {
        self.registry.query(name)
    }
}
