use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use rust_register_center::{
    discovery::ServiceDiscovery,
    registry::{Registry, ServiceInstance},
};

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
        registry: Registry {
            services: Arc::new(RwLock::new(HashMap::new())),
        },
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
