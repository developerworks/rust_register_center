// // filename: main.rs

// mod config;
// mod registry;
// mod server;

// async fn main() {
//   let config = config::load_config().await;
//   let registry = registry::ServiceRegistry::new(&config);
//   let server = server::serve(registry).await;
// }
mod discovery;
mod registry;
mod server;
mod store;

use actix_web::{web, App, HttpServer};

use registry::Registry;
use store::Store;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(Registry::new()))
            .app_data(web::Data::new(Store::new()))
            .service(server::register)
            .service(server::query)
            .service(server::get_config)
        // 其他接口
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
