// #[macro_use] extern crate log;
extern crate env_logger;
extern crate actix_web;

mod endpoints;
use endpoints::*;
use actix_web::{ App, HttpServer };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(init)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
