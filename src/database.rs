use std::sync::{Arc, Mutex};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::models::GasInfo;

#[derive(Clone)]
pub struct Database {
    pub client: Arc<Mutex<DataPool>>,
}

impl Database {
    pub async fn new() -> Result<Database, sqlx::Error> {
        Ok(Database {
            client: Arc::new(Mutex::new(DataPool::new(false).await?)),
        })
    }

    pub async fn forced() -> Result<Database, sqlx::Error> {
        Ok(Database {
            client: Arc::new(Mutex::new(DataPool::new(true).await?)),
        })
    }
}

#[derive(Clone)]
pub struct DataPool {
    pub pg: Pool<Postgres>,
    force: bool,
}

impl DataPool {
    pub async fn new(force: bool) -> Result<DataPool, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:password@localhost/test")
            .await?;
        Ok(DataPool { pg: pool, force })
    }

    pub async fn table_exists(&self, table_name: &str) -> Result<bool, sqlx::Error> {
        let exists: (bool,) = sqlx::query_as(
            "
            SELECT EXISTS (
                SELECT 
                FROM information_schema.tables
                WHERE table_schema = 'test'
                AND   table_name = 'mytesttable'
            )",
        )
        .fetch_one(&self.pg)
        .await?;
        Ok(exists.0)
    }

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
}
