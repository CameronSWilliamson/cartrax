// use std::sync::{Arc, Mutex};

// use crate::models::GasInfo;
// use futures::StreamExt;
// use mongodb::{Client, Collection};

// const DB_NAME: &str = "HomeDB";
// const COLL_NAME: &str = "cartrax";

// #[derive(Clone)]
// pub struct Database {
//     client: Arc<Mutex<Client>>,
// }

// impl Database {
//     pub async fn new() -> Database {
//         let uri =
//             std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
//         let client = Client::with_uri_str(uri).await.expect("failed to connect");
//         Database {
//             client: Arc::new(Mutex::new(client)),
//         }
//     }

//     pub async fn add_data(&self, data: &mut GasInfo) -> mongodb::error::Result<()> {
//         let collection = self
//             .client
//             .lock()
//             .unwrap()
//             .database(DB_NAME)
//             .collection(COLL_NAME);
//         let count = collection.count_documents(None, None).await?;
//         data.id = Some(count as u32);
//         collection.insert_one(data.clone(), None).await?;
//         Ok(())
//     }

//     pub async fn get_data(&self) -> Result<Vec<GasInfo>, Box<dyn std::error::Error>> {
//         let collection: Collection<GasInfo> = self
//             .client
//             .lock()
//             .unwrap()
//             .database(DB_NAME)
//             .collection(COLL_NAME);
//         let mut cursor = collection.find(None, None).await?;
//         let mut data = Vec::new();

//         while let Some(result) = cursor.next().await {
//             if let Ok(document) = result {
//                 data.push(document)
//             }
//         }

//         Ok(data)
//     }
// }
//

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
