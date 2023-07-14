pub mod database;
pub mod handlers;
pub mod models;

use std::{fs::File, io::BufReader, process::exit};

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use models::GasInfo;
use serde::{Deserialize, Serialize};

/// The commandline arguments allowed for this program
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    /// Subcommands that allow for different functionality
    pub command: Option<Commands>,
}

/// Subcommands that change the functionality of the program
#[derive(Subcommand)]
pub enum Commands {
    /// Creates new tables in the database. If a filename is provided
    /// then it will read a CSV file and store that in the database as well
    Migrate {
        /// The file to migrate data from
        filename: Option<String>,
    },
    /// Starts the API. This is also the default mode.
    Api {},
    /// Makes a copy of all of the data in the database in a CSV file with
    /// the provided filename.
    Backup {
        /// The file to backup data to
        filename: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VersionInfo {
    package_name: String,
    version: String,
    deploy_time: String,
}

impl VersionInfo {
    pub fn new() -> VersionInfo {
        let time = chrono::offset::Utc::now();

        VersionInfo {
            package_name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            deploy_time: time.to_string(),
        }
    }
}

impl Default for VersionInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Configures and runs the API.
pub async fn run_api() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    if let Ok(database) = database::Database::new().await {
        database::ensure_tables_exist(&database.client, false)
            .await
            .unwrap();

        HttpServer::new(move || {
            let cors = Cors::permissive();
            App::new()
                .app_data(web::Data::new(VersionInfo::new()))
                .configure(handlers::config(database.clone()))
                .service(handlers::index)
                .service(handlers::version)
                .wrap(cors)
        })
        .bind(("0.0.0.0", 5000))?
        .run()
        .await?
    } else {
        println!("Failed to connect to database");
        exit(1);
    }
    Ok(())
}

/// Copies all data in the database to a file with the provided filename
///
/// # Arguments
///
/// * `filename` - The name of the CSV file to store the data in
pub async fn run_backup(filename: &String) -> std::io::Result<()> {
    let pool = database::new().await;
    if let Err(error) = pool {
        println!("Failed to connect to database: {}", error);
        exit(1);
    }
    let pool = pool.unwrap();
    let mut file = File::create(filename)?;
    let mut csv_writer = csv::Writer::from_writer(&mut file);
    let detail_list = sqlx::query_as::<_, GasInfo>("SELECT * FROM cartrax ORDER BY id")
        .fetch_all(&pool)
        .await;
    if let Err(error) = detail_list {
        println!("Failed to fetch data: {}", error);
        exit(1);
    }
    for entry in detail_list.unwrap() {
        csv_writer.serialize(entry)?;
    }
    csv_writer.flush()?;
    Ok(())
}

/// Creates new tables in the database. If a filename is provided then
/// the file is read as a CSV file and all data from the file is uploaded to
/// the database
///
/// # Arguments
///
/// * `filename` - An optional string that holds the name of the file to read from
pub async fn run_migration(filename: &Option<String>) -> std::io::Result<()> {
    let pool = database::new().await;
    if let Err(error) = pool {
        println!("Failed to connect to database: {}", error);
        exit(1);
    }
    let pool = pool.unwrap();

    let tables_created = database::ensure_tables_exist(&pool, true).await;
    if let Err(error) = tables_created {
        println!("Failed to migrate database: {}", error);
        exit(1);
    }

    if let Some(filename) = filename {
        let file = File::open(filename)?;
        let buf_reader = BufReader::new(file);
        let mut csv_reader = csv::Reader::from_reader(buf_reader);
        let mut counter = 2;
        for entry in csv_reader.deserialize() {
            let record: GasInfo = entry?;
            if database::insert_gas_info(&pool, &record).await.is_err() {
                println!("Failed to add entry on line {counter}");
            }
            counter += 1;
        }
    }
    println!("Successfully migrated data");
    Ok(())
}

pub struct Environment {
    db_hostname: String,
    db_username: String,
    db_password: String,
    db_database: String,
}

impl Environment {
    fn new() -> Environment {
        dotenv().ok();

        Environment {
            db_hostname: Environment::parse_var("DB_HOSTNAME"),
            db_username: Environment::parse_optional_var("DB_USERNAME", "postgres"),
            db_password: Environment::parse_var("DB_PASSWORD"),
            db_database: Environment::parse_var("DB_DATABASE"),
        }
    }

    fn parse_var(value: &str) -> String {
        if let Ok(env_var) = std::env::var(value) {
            env_var
        } else {
            println!("Unable to find environment variable \"{}\"", value);
            exit(1);
        }
    }

    fn parse_optional_var(value: &str, default: &str) -> String {
        if let Ok(env_var) = std::env::var(value) {
            env_var
        } else {
            default.to_string()
        }
    }
}
