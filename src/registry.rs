// filename: registry.rs

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub use service_instance::ServiceInstance;

pub mod service_instance;

/// Service registry:
/// A hash table, it's element key is service name, element value is a collection of service instances.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Registry {
    pub services: Arc<RwLock<HashMap<String, Vec<ServiceInstance>>>>,
}

impl Drop for Registry {
    fn drop(&mut self) {
        println!("Drop registry");
    }
}

/// Implement Default trait for Registry
///
impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

// /// Implement Clone trait for Registry
// impl Clone for Registry {
//     fn clone(&self) -> Self {
//         let mut cloned_services = HashMap::new();

//         for (name, instances) in &self.services {
//             let cloned_instances = instances.to_vec();
//             cloned_services.insert(name.clone(), cloned_instances);
//         }

//         Registry {
//             services: cloned_services,
//         }
//     }
// }

/// Implement Registry
#[allow(unused)]
impl Registry {
    /// Create a new Registry
    /// # Example
    /// ```rust
    /// let registry = Registry::new();
    /// ```
    /// # Returns
    /// A new Registry
    /// # Note
    /// The Registry is a thread-safe data structure.
    /// # See Also
    /// [`Registry::new`]
    /// [`Registry::default`]
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    ///
    /// Register a service instance
    ///
    /// ```rust
    /// let registry: Arc<RwLock<Registry>> = Arc::new(RwLock::new(Registry::new()));
    /// let instance = ServiceInstance::new("foo".to_string(), "https://foo.com".to_string());
    /// registry.write().unwrap().register(instance.clone());
    /// assert_eq!(registry.read().unwrap().query("foo"), vec![instance]);
    /// ```
    pub fn register(&mut self, service_instance: ServiceInstance) {
        let service_name: String = service_instance.name.clone();
        let mut services = self.services.write().unwrap();
        if let Some(instances) = services.get_mut(&service_name) {
            instances.push(service_instance);
        } else {
            services.insert(service_name, vec![service_instance]);
        }
    }

    /// Query service instances by service name
    ///
    pub fn query(&self, name: &str) -> Vec<ServiceInstance> {
        println!("### Query service instances by service name");

        let services = self.services.read().unwrap();
        println!("### self.services {:?}", services);

        let r: Vec<ServiceInstance> = if let Some(instances) = services.get(name) {
            println!("Query service {:?}", instances);
            instances.to_vec()
        } else {
            println!("Services not found");
            Vec::new()
        };

        println!("### r = {:?}", r);
        r
    }
}
