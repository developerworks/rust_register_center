// filename: discovery.rs

// 服务发现模块

use crate::registry::{Registry, ServiceInstance};

pub struct ServiceDiscovery {
    pub registry: Registry,
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
