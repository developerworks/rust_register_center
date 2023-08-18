// filename: server.rs

use std::sync::{Arc, RwLock};

use actix_web::{get, post, web, HttpResponse, Responder};

use crate::{
    discovery::ServiceDiscovery,
    registry::{Registry, ServiceInstance},
    store::Store,
};

#[post("/registry")]
async fn register(
    registry: web::Data<Arc<RwLock<Registry>>>,
    instance: web::Json<ServiceInstance>,
) -> impl Responder {
    let mut registry = registry.write().unwrap();

    let instance = instance.into_inner();

    registry.register(instance);

    HttpResponse::Ok().finish()
}

#[get("/registry/{name}")]
async fn query(discovery: web::Data<ServiceDiscovery>, name: web::Path<String>) -> impl Responder {
    let instances = discovery.query(&name);

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
