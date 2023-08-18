// filename: registry.rs

pub mod service_instance;
use std::collections::HashMap;

pub use service_instance::ServiceInstance;
pub struct Registry {
    services: HashMap<String, Vec<ServiceInstance>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    // 注册服务实例
    pub(crate) fn register(&mut self, instance: ServiceInstance) {
        // 先检查是否存在
        if let Some(service) = self.services.get_mut(&instance.name) {
            if service.contains(&instance) {
                // 实例已存在,直接返回
                return;
            }
        }
        self.services
            .entry(instance.name.clone())
            .or_insert_with(Vec::new)
            .push(instance);
    }

    // 查询服务
    pub fn query(&self, name: &str) -> Vec<ServiceInstance> {
        match self.services.get(name) {
            Some(instances) => instances.to_vec(),
            None => Vec::new(),
        }
    }
}

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
