
use std::sync::{RwLock, Arc};

use rust_register_center::registry::{Registry, ServiceInstance};

#[test]
fn test_register() {

    let registry: Arc<RwLock<Registry>> = Arc::new(RwLock::new(Registry::new()));
    // let mut registry = Registry::new();
    let instance = ServiceInstance::new("foo".to_string(), "http://foo.com".to_string());

    registry.write().unwrap().register(instance.clone());

    // registry.register(instance.clone());

    assert_eq!(registry.read().unwrap().query("foo"), vec![instance]);
}

#[test]
fn test_register_duplicate() {
    let mut registry = Registry::new();

    let instance = ServiceInstance::new("bar".to_string(), "http://bar.com".to_string());
    registry.register(instance.clone());
    registry.register(instance.clone());

    assert_eq!(registry.query("bar").len(), 2);
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
