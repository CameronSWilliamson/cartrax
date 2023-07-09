pub mod database;
pub mod handlers;
pub mod models;

use std::{fs::File, io::BufReader, process::exit};

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use clap::{Parser, Subcommand};
use database::DataPool;
use models::GasInfo;

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

/// Configures and runs the API.
pub async fn run_api() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    if let Ok(database) = database::Database::new().await {
        HttpServer::new(move || {
            let cors = Cors::permissive();
            App::new()
                .configure(handlers::config(database.clone()))
                .service(handlers::index)
                .wrap(cors)
        })
        .bind(("localhost", 8080))?
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
    let pool = DataPool::new(true).await;
    if let Err(error) = pool {
        println!("Failed to connect to database: {}", error.to_string());
        exit(1);
    }
    let pool = pool.unwrap();
    let mut file = File::create(filename)?;
    let mut csv_writer = csv::Writer::from_writer(&mut file);
    let detail_list = sqlx::query_as::<_, GasInfo>("SELECT * FROM cartrax ORDER BY id")
        .fetch_all(&pool.pg)
        .await;
    if let Err(error) = detail_list {
        println!("Failed to fetch data: {}", error.to_string());
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
    let pool = DataPool::new(true).await;
    if let Err(error) = pool {
        println!("Failed to connect to database: {}", error.to_string());
        exit(1);
    }
    let pool = pool.unwrap();

    let tables_created = create_tables(&pool).await;
    if let Err(error) = tables_created {
        println!("Failed to migrate database: {}", error.to_string());
        exit(1);
    }

    if let Some(filename) = filename {
        let file = File::open(filename)?;
        let buf_reader = BufReader::new(file);
        let mut csv_reader = csv::Reader::from_reader(buf_reader);
        let mut counter = 2;
        for entry in csv_reader.deserialize() {
            let record: GasInfo = entry?;
            match pool.insert_gas_info(&record).await {
                Err(_) => println!("Failed to add entry on line {counter}"),
                Ok(_) => (),
            }
            counter += 1;
        }
    }
    Ok(())
}

/// Creates the required tables in the database.
///
/// # Arguments
///
/// * `pool` - The SQLX data pool
async fn create_tables(pool: &DataPool) -> Result<(), sqlx::Error> {
    let fields = "
        id SERIAL PRIMARY KEY NOT NULL,
        price_per_gallon NUMERIC(5, 3) NOT NULL,
        total_cost NUMERIC(6, 2) NOT NULL,
        gallons NUMERIC(5, 3) NOT NULL,
        a_tripometer NUMERIC(8, 1) NOT NULL,
        b_tripometer NUMERIC(8, 1) NOT NULL,
        total_tripometer INTEGER NOT NULL,
        time_recorded TIMESTAMPTZ NOT NULL,
        city TEXT NOT NULL,
        state TEXT NOT NULL
        ";

    pool.create_table("cartrax", fields).await?;
    Ok(())
}
