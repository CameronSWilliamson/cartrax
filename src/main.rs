mod database;
mod handlers;
mod models;

use actix_web::{App, HttpServer};
use clap::Parser;
use database::Database;

const DB_NAME: &str = "HomeDB";
const COLL_NAME: &str = "cartrax";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long)]
    migrate: bool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = CliArgs::parse();

    if args.migrate {

    }

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let gas_info_database: Database<models::GasInfo> = Database::new(DB_NAME.to_string(), COLL_NAME.to_string()).await;
    HttpServer::new(move || App::new().configure(handlers::config(gas_info_database.clone())))
        .bind(("localhost", 8080))?
        .run()
        .await
}
