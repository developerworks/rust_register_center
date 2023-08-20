// filename: server.rs

use std::sync::RwLock;

use actix_web::{get, post, web, Responder};

use crate::{
    // discovery::ServiceDiscovery,
    registry::{Registry, ServiceInstance},
    store::Store,
};

#[post("/registry")]
async fn register(
    registry: web::Data<RwLock<Registry>>,
    instance: web::Json<ServiceInstance>,
) -> impl Responder {
    let instance = instance.into_inner();
    {
        registry.write().unwrap().register(instance.clone());
    }
    web::Json(instance)
}

#[get("/registries")]
async fn query_all(registry: web::Data<RwLock<Registry>>,) -> impl Responder {
    let r = registry.read().unwrap().services.clone();
    web::Json(r)
}

#[get("/registry/{name}")]
async fn query(registry: web::Data<RwLock<Registry>>, name: web::Path<String>) -> impl Responder {

    println!("service name: {}", name);
    // let instances = discovery.query(&name);
    let instances = registry.read().unwrap().query(&name);
    println!("service instances: {:?}", instances);
    web::Json(instances)
}



// 配置获取接口
#[get("/config/{service}/{key}")]
async fn get_config(
    store: web::Data<Store>,
    service: web::Path<String>,
    key: web::Path<String>,
) -> impl Responder {
    let value = store.get(&service, &key);

    match value {
        Some(value) => web::Json(value),
        None => web::Json(crate::store::ServiceConfigValue::String("".to_string())),
    }
}
