// filename: main.rs

mod discovery;
mod registry;
mod server;
mod store;

use actix_web::{web, App, HttpServer};
use futures::join;
use registry::Registry;
use store::Store;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(Registry::new()))
            .app_data(web::Data::new(Store::new()))
            .service(server::register)
            .service(server::query)
            .service(server::get_config)
        // 其他接口
    })
    .bind("127.0.0.1:8080")?;

    let future_on_start = async { on_start() };
    let future_server = async { server.run().await };

    let _result = join!(future_server, future_on_start);

    Ok(())
}

fn on_start() -> std::io::Result<()> {
    println!("Server started at http://127.0.0.1:8080");
    Ok(())
}
