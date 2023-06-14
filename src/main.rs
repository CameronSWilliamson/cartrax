mod database;
mod handlers;
mod models;

use actix_web::{web::Data, App, HttpServer};
use database::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let database = Database::new().await;
    HttpServer::new(move || App::new().configure(handlers::config(database.clone())))
        .bind(("localhost", 8080))?
        .run()
        .await
}
