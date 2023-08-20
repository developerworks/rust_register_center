// filename: discovery.rs

use crate::registry::{Registry, ServiceInstance};

pub struct ServiceDiscovery {
    pub registry: Registry,
}

impl ServiceDiscovery {
    #[allow(unused)]
    pub fn new(registry: Registry) -> Self {
        Self { registry }
    }

    pub fn register(&mut self, service_instance: ServiceInstance) {
        self.registry.register(service_instance);
    }

    #[allow(unused)]
    pub fn query(&self, name: &str) -> Vec<ServiceInstance> {
        self.registry.query(name)
    }
}
