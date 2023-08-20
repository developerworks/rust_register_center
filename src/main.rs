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

use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
use futures::join;
use log::LevelFilter;
use registry::Registry;
use store::mysql::MysqlStore;

#[allow(unused)]
pub struct AppState {
    registry: Mutex<Registry>, // <- Mutex is necessary to mutate safely across threads
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    // logger::init_logger();
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    // Load config
    let config = load_config();
    let host = format!("{}:{}", config.host, config.port);

    // Move into HttpServer::new()
    let config_clone: config::Config = config.clone();
    let app_state: web::Data<AppState> = web::Data::new(AppState {
        registry: Mutex::new(Registry::new()),
    });

    
    // let discovery = discovery::ServiceDiscovery::new(registry_data);
    // Initialize the HttpServer
    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .app_data(web::Data::new(config_clone.clone()))
            // .app_data(web::Data::new(discovery))
            .app_data(web::Data::new(store::Store::new()))
            .app_data(web::Data::new(events::default::Events::new))
            .service(rest_api::register)
            .service(rest_api::query)
            .service(rest_api::get_config)
            .service(rest_api::query_all)
    })
    .bind(host)?;

    // Start the server
    let future_server = async { server.run().await };
    let future_on_start = async { on_start(config.clone()) };

    let future_mysql_test = async {
        let mysql_store = MysqlStore::new().await.unwrap();
        let rows = mysql_store.get("service", "key");
        
    };
    let _result = join!(
        future_server, 
        future_on_start,
        future_mysql_test
    );

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
