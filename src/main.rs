use cartraxapi::{Cli, Commands};
use clap::Parser;
//use database::migrate;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut cli = Cli::parse();

    if let None = &cli.command {
        cli.command = Some(Commands::Api {})
    }

    match &cli.command.unwrap() {
        Commands::Api {} => cartraxapi::run_api().await?,
        Commands::Migrate {filename} => cartraxapi::run_migration(filename).await?,
    }
    Ok(())
}
