use cartrax::{run_api, run_backup, run_migration, Cli, Commands};
use clap::Parser;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mut cli = Cli::parse();

    if cli.command.is_none() {
        cli.command = Some(Commands::Api {})
    }

    match &cli.command.unwrap() {
        Commands::Api {} => run_api().await?,
        Commands::Migrate { filename } => run_migration(filename).await?,
        Commands::Backup { filename } => run_backup(filename).await?,
    }
    Ok(())
}
