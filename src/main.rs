mod database;
mod handlers;
mod models;

use actix_web::{App, HttpServer};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates new tables in the database
    Migrate {},
    /// Starts the API. This is also the default mode
    Api {},
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut cli = Cli::parse();

    if let None = &cli.command {
        cli.command = Some(Commands::Api {})
    }

    match &cli.command.unwrap() {
        Commands::Api {} => {}
        Commands::Migrate {} => {}
    }

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
