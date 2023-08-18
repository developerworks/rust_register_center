// filename: main.rs

mod config;
mod discovery;
mod events;
mod logger;
mod registry;
mod rest_api;
mod service_configuration;
mod store;

use actix_web::{web, App, HttpServer};
use discovery::ServiceDiscovery;
use futures::join;
use registry::Registry;
use store::Store;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_config();
    let host = format!("{}:{}", config.host, config.port);

    let config_clone = config.clone();

    let server = HttpServer::new(move || {
        let discovery = ServiceDiscovery::new(Registry::new());
        App::new()
            .app_data(web::Data::new(config_clone.clone()))
            .app_data(web::Data::new(discovery))
            .app_data(web::Data::new(Store::new()))
            .service(rest_api::register)
            .service(rest_api::query)
            .service(rest_api::get_config)
    })
    .bind(host)?;

    let future_on_start = async { on_start(config.clone()) };
    let future_server = async { server.run().await };

    let _result = join!(future_server, future_on_start);

    Ok(())
}

fn on_start(config: config::Config) -> std::io::Result<()> {
    println!("Server started at http://{}:{}", config.host, config.port);
    Ok(())
}

fn load_config() -> config::Config {
    let config_file = vec![config::PROJECT_ROOT, config::CONFIG_FILE]
        .join("/")
        .to_string();
    println!("config: {}", config_file);
    config::load_config(&config_file).unwrap()
}
