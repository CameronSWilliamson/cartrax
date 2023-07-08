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
    pg: Pool<Postgres>,
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
        let sql_string = format!(
            "
            CREATE TABLE IF NOT EXISTS {} (
                {}
            );",
            table_name, fields
        );
        sqlx::query(&sql_string).execute(&self.pg).await?;

        Ok(())
    }
}
