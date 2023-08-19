// filename: main.rs

mod config;
mod discovery;
mod events;
mod logger;
mod registry;
mod rest_api;
mod service_configuration;
mod store;
mod task;

use actix_web::{web, App, HttpServer};
use futures::join;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    logger::init_logger();

    // Load config
    let config = load_config();
    let host = format!("{}:{}", config.host, config.port);

    // Move into HttpServer::new()
    let config_clone = config.clone();

    // Initialize the HttpServer
    let server = HttpServer::new(move || {
        let discovery = discovery::ServiceDiscovery::new(registry::Registry::new());
        App::new()
            .app_data(web::Data::new(config_clone.clone()))
            .app_data(web::Data::new(discovery))
            .app_data(web::Data::new(store::Store::new()))
            .app_data(web::Data::new(events::default::Events::new))
            .service(rest_api::register)
            .service(rest_api::query)
            .service(rest_api::get_config)
    })
    .bind(host)?;

    // Start the server
    let future_on_start = async { on_start(config.clone()) };
    let future_server = async { server.run().await };

    let _result = join!(future_server, future_on_start);

    Ok(())
}

/// Server start callback
fn on_start(config: config::Config) -> std::io::Result<()> {
    println!("Server started at http://{}:{}", config.host, config.port);
    Ok(())
}

/// Load config
fn load_config() -> config::Config {
    let config_file = vec![config::PROJECT_ROOT, config::CONFIG_FILE]
        .join("/")
        .to_string();
    println!("config: {}", config_file);
    config::load_config(&config_file).unwrap()
}
