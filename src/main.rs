mod database;
mod handlers;
mod models;

use actix_web::{App, HttpServer, web::Data};
use database::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let database = Database::new().await;
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(database.clone()))
            .configure(handlers::config)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
