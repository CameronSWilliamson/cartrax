use crate::{
    models::{GasInfo, GasInfoStats},
    Environment,
};
use bigdecimal::BigDecimal;
use sqlx::{postgres::PgPoolOptions, Postgres};

/// The Arctix Database
#[derive(Clone)]
pub struct Database {
    /// A client containing a DataPool
    pub client: Pool,
}

impl Database {
    /// Returns a new Database instance
    pub async fn new() -> Result<Database> {
        Ok(Database {
            client: new().await?,
        })
    }
}

type Pool = sqlx::Pool<Postgres>;
type Result<T> = std::result::Result<T, sqlx::Error>;

/// Returns a new DataPool containing a PostgreSQL connection pool.
///
/// # Arguments
///
/// * `force` - Whether or not all operations on the DataPool should be forced
pub async fn new() -> Result<Pool> {
    let env = Environment::new();

    let conn_string = format!(
        "postgres://{}:{}@{}/{}",
        env.db_username, env.db_password, env.db_hostname, env.db_database
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_string)
        .await?;
    Ok(pool)
}

/// Returns whether or not the provided table_name exists.
///
/// # Arguments
///
/// * `table_name` - The name of the table
pub async fn _table_exists(pool: &Pool, table_name: &str) -> Result<bool> {
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
    .fetch_one(pool)
    .await?;
    Ok(exists.0)
}

/// Creates a table in the database with the provided name and schema.
///
/// # Arguments
///
/// * `table_name` - The name of the table to be created
/// * `fields` - The names and types of all fields to be created
pub async fn create_table(pool: &Pool, table_name: &str, fields: &str, force: bool) -> Result<()> {
    let command = if force {
        sqlx::query(format!("DROP TABLE IF EXISTS {}", table_name).as_str())
            .execute(pool)
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
    sqlx::query(&sql_string).execute(pool).await?;
    Ok(())
}

/// Adds an entry to the gas info table. The id field on the GasInfo struct
/// will be ignored.
///
/// # Arguments
///
/// * `gas_info` - The entry to be added.
pub async fn insert_gas_info(pool: &Pool, gas_info: &GasInfo) -> Result<i32> {
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
    .fetch_one(pool)
    .await?;
    Ok(id.0)
}

pub async fn get_stats(pool: &Pool) -> Result<GasInfoStats> {
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
    .fetch_one(pool)
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

/// Creates the required tables in the database.
///
/// # Arguments
///
/// * `pool` - The SQLX data pool
pub async fn ensure_tables_exist(pool: &Pool, force: bool) -> Result<()> {
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

    create_table(&pool, "cartrax", fields, force).await?;
    Ok(())
}
