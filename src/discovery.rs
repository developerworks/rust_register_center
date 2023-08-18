// filename: discovery.rs

// 服务发现模块

use crate::registry::{Registry, ServiceInstance};

pub struct ServiceDiscovery {
    registry: Registry,
}

impl ServiceDiscovery {
    
    #[allow(unused)]
    pub fn new(registry: Registry) -> Self {
        Self { registry }
    }

    // 根据服务名称查询
    #[allow(unused)]
    pub fn query(&self, name: &str) -> Vec<ServiceInstance> {
        self.registry.query(name)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_query_by_name() {
        let mut registry = Registry::new();
        registry.register(ServiceInstance::new(
            "svc1".to_owned(),
            "http://svc1.com".to_owned(),
        ));

        let discovery = ServiceDiscovery { registry };

        let instances = discovery.query("svc1");

        assert_eq!(instances.len(), 1);
        assert_eq!(instances[0].name, "svc1");
    }

    #[test]
    fn test_query_not_found() {
        let discovery = ServiceDiscovery {
            registry: Registry::new(),
        };

        let instances = discovery.query("unknown");

        assert!(instances.is_empty());
    }

    #[test]
    fn test_query_multiple_instances() {
        let mut registry = Registry::new();

        registry.register(ServiceInstance::new(
            "svc".to_owned(),
            "http://svc1.com".to_owned(),
        ));
        registry.register(ServiceInstance::new(
            "svc".to_owned(),
            "http://svc2.com".to_owned(),
        ));

        let discovery = ServiceDiscovery { registry };

        let instances = discovery.query("svc");

        assert_eq!(instances.len(), 2);
    }
}
