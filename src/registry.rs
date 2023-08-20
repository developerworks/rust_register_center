// filename: registry.rs

pub mod service_instance;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub use service_instance::ServiceInstance;

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
///
impl Registry {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a service instance
    #[allow(unused)]
    pub fn register(&mut self, service_instance: ServiceInstance) {
        let service_name: String = service_instance.name.clone();
        let mut services: std::sync::RwLockWriteGuard<'_, HashMap<String, Vec<ServiceInstance>>> = self.services.write().unwrap();
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

        let services: std::sync::RwLockReadGuard<'_, HashMap<String, Vec<ServiceInstance>>> = self.services.read().unwrap();
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
