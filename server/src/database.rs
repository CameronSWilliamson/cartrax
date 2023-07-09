use crate::models::{GasInfo, GasInfoStats};
use bigdecimal::BigDecimal;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{
    env,
    process::exit,
    sync::{Arc, Mutex},
};

/// The Arctix Database
#[derive(Clone)]
pub struct Database {
    /// A client containing a DataPool
    pub client: Arc<Mutex<DataPool>>,
}

impl Database {
    /// Returns a new Database instance
    pub async fn new() -> Result<Database, sqlx::Error> {
        Ok(Database {
            client: Arc::new(Mutex::new(DataPool::new(false).await?)),
        })
    }
}

/// A PostgreSQL database connection
#[derive(Clone)]
pub struct DataPool {
    /// The pool that connects to postgres
    pub pg: Pool<Postgres>,
    /// Determines if all operations should be forced.
    /// This should be set to false while running the API
    force: bool,
}

impl DataPool {
    /// Returns a new DataPool containing a PostgreSQL connection pool.
    ///
    /// # Arguments
    ///
    /// * `force` - Whether or not all operations on the DataPool should be forced
    pub async fn new(force: bool) -> Result<DataPool, sqlx::Error> {
        let server_name = env::var("DB_NAME");
        let username = env::var("DB_USERNAME");
        let password = env::var("DB_PASSWORD");
        let database = env::var("DB_DATABASE");

        if server_name.is_err() || username.is_err() || password.is_err() || database.is_err() {
            println!("Something went wrong parsing database environment variables");
            exit(1);
        }

        let conn_string = format!(
            "postgres://{}:{}@{}/{}",
            username.unwrap(),
            password.unwrap(),
            server_name.unwrap(),
            database.unwrap()
        );

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&conn_string)
            .await?;
        Ok(DataPool { pg: pool, force })
    }

    /// Returns whether or not the provided table_name exists.
    ///
    /// # Arguments
    ///
    /// * `table_name` - The name of the table
    pub async fn _table_exists(&self, table_name: &str) -> Result<bool, sqlx::Error> {
        let exists: (bool,) = sqlx::query_as(
            "
            SELECT EXISTS (
                SELECT
                FROM information_schema.tables
                WHERE table_schema = 'test'
                AND   table_name = $1
            )",
        )
        .bind(table_name)
        .fetch_one(&self.pg)
        .await?;
        Ok(exists.0)
    }

    /// Creates a table in the database with the provided name and schema.
    ///
    /// # Arguments
    ///
    /// * `table_name` - The name of the table to be created
    /// * `fields` - The names and types of all fields to be created
    pub async fn create_table(&self, table_name: &str, fields: &str) -> Result<(), sqlx::Error> {
        let command = if self.force {
            sqlx::query(format!("DROP TABLE IF EXISTS {}", table_name).as_str())
                .execute(&self.pg)
                .await?;
            "CREATE TABLE"
        } else {
            "CREATE TABLE IF NOT EXISTS"
        };
        let sql_string = format!(
            "{} {} (
                    {}
                );",
            command, table_name, fields
        );
        sqlx::query(&sql_string).execute(&self.pg).await?;
        Ok(())
    }

    /// Adds an entry to the gas info table. The id field on the GasInfo struct
    /// will be ignored.
    ///
    /// # Arguments
    ///
    /// * `gas_info` - The entry to be added.
    pub async fn insert_gas_info(&self, gas_info: &GasInfo) -> Result<i32, sqlx::Error> {
        let id: (i32,) = sqlx::query_as(
            "INSERT INTO cartrax
        (price_per_gallon, total_cost, gallons, a_tripometer,
         b_tripometer, total_tripometer, time_recorded, city, state)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id;",
        )
        .bind(&gas_info.price_per_gallon)
        .bind(&gas_info.total_cost)
        .bind(&gas_info.gallons)
        .bind(&gas_info.a_tripometer)
        .bind(&gas_info.b_tripometer)
        .bind(&gas_info.total_tripometer)
        .bind(&gas_info.time_recorded)
        .bind(&gas_info.city)
        .bind(&gas_info.state)
        .fetch_one(&self.pg)
        .await?;
        Ok(id.0)
    }

    pub async fn get_stats(&self) -> Result<GasInfoStats, sqlx::Error> {
        let (total_cost, total_gallons, avg_ppg, avg_mpg, avg_a_trip, avg_fill_size): (
            BigDecimal,
            BigDecimal,
            BigDecimal,
            BigDecimal,
            BigDecimal,
            BigDecimal,
        ) = sqlx::query_as(
            "SELECT SUM(total_cost), 
                        SUM(gallons), 
                        AVG(price_per_gallon), 
                        MAX(total_tripometer) / SUM(gallons),
                        AVG(a_tripometer),
                        AVG(gallons)
                 FROM cartrax
                ",
        )
        .fetch_one(&self.pg)
        .await?;

        Ok(GasInfoStats {
            total_cost,
            total_gallons,
            avg_ppg,
            avg_mpg,
            avg_a_trip,
            avg_fill_size,
        })
    }
}
