mod database;
mod handlers;
mod models;

use std::{fs::File, io::BufReader, process::exit};

use actix_web::{App, HttpServer};
use clap::{Parser, Subcommand};
use database::DataPool;
use models::GasInfo;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Creates new tables in the database
    Migrate {
        #[arg(short, long)]
        filename: Option<String>,
    },
    /// Starts the API. This is also the default mode
    Api {},
}

pub async fn run_api() -> std::io::Result<()> {
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
            println!("{:?}", record);
            match pool.insert_gas_info(&record).await {
                Err(_) => println!("Failed to add entry on line {counter}"),
                Ok(id) => println!("Successfully added entry {id}"),
            }
            counter += 1;
        }
    }
    Ok(())
}

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
