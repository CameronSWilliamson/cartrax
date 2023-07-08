use std::sync::{Arc, Mutex};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

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
}

pub async fn migrate() -> Result<(), sqlx::Error> {
    let pool = DataPool::new(true).await?;

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
