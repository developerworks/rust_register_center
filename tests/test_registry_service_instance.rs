use std::collections::HashMap;

use rust_register_center::registry::ServiceInstance;

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
