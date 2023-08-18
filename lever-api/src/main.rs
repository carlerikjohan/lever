use std::env;
use actix_web::{App, HttpServer};

mod feature;
mod response;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .service(feature::create)
            .service(feature::list)
            .service(feature::get)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}