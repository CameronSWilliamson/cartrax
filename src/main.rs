mod database;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(handlers::config))
        .bind(("localhost", 8080))?
        .run()
        .await
}
