// filename: server.rs

use actix_web::{get, post, web, Responder};

use crate::{
    // discovery::ServiceDiscovery,
    registry::ServiceInstance,
    store::{ServiceConfigValue, Store},
    AppState,
};

#[post("/registry")]
async fn register(
    app_state: web::Data<AppState>,
    instance: web::Json<ServiceInstance>,
) -> impl Responder {
    let registry = &app_state.registry;
    let instance: ServiceInstance = instance.into_inner();
    registry.lock().unwrap().register(instance.clone());
    web::Json(instance)
}

#[get("/registries")]
async fn query_all(app_state: web::Data<AppState>) -> impl Responder {
    let registry = &app_state.registry;
    let r = registry.lock().unwrap().services.clone();
    web::Json(r)
}

#[get("/registry/{name}")]
async fn query(app_state: web::Data<AppState>, name: web::Path<String>) -> impl Responder {
    let registry = &app_state.registry;
    let instances: Vec<ServiceInstance> = registry.lock().unwrap().query(&name);
    web::Json(instances)
}

// 配置获取接口
#[get("/config/{service}/{key}")]
async fn get_config(
    store: web::Data<Store>,
    service: web::Path<String>,
    key: web::Path<String>,
) -> impl Responder {
    let value: Option<ServiceConfigValue> = store.get(&service, &key);

    match value {
        Some(value) => web::Json(value),
        None => web::Json(crate::store::ServiceConfigValue::String("".to_string())),
    }
}
