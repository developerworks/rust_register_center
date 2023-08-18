// filename: server.rs

use std::sync::{Arc, RwLock};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::{
    registry::{self, Registry, ServiceInstance},
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
async fn query(registry: web::Data<Registry>, name: web::Path<String>) -> impl Responder {
    let instances: Vec<ServiceInstance> = registry.query(&name);

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
        None => web::Json(crate::store::ConfigValue::String("".to_string())),
    }
}

#[actix_rt::main]
pub async fn serve(registry: registry::Registry) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(Arc::new(RwLock::new(registry.clone()))) // Pass a cloned Arc<RwLock<Registry>> to each worker
            .service(register) // Add your routes here
            .service(query)
            .service(get_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
