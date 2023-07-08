mod database;
mod handlers;
mod models;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    if let Ok(database) = database::Database::new().await {
        HttpServer::new(move || App::new().configure(handlers::config(database.clone())))
            .bind(("localhost", 8080))?
            .run()
            .await?
    } else {
        println!("Failed to connect to database");
    }
    Ok(())
}
