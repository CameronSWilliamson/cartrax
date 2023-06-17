use std::sync::{Arc, Mutex};

use crate::models::GasInfo;
use futures::StreamExt;
use mongodb::{Client, Collection};

const DB_NAME: &str = "HomeDB";
const COLL_NAME: &str = "cartrax";

#[derive(Clone)]
pub struct Database {
    client: Arc<Mutex<Client>>,
}

impl Database {
    pub async fn new() -> Database {
        let uri =
            std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
        let client = Client::with_uri_str(uri).await.expect("failed to connect");
        Database {
            client: Arc::new(Mutex::new(client)),
        }
    }

    pub async fn add_data(&self, data: &mut GasInfo) -> mongodb::error::Result<()> {
        let collection = self
            .client
            .lock()
            .unwrap()
            .database(DB_NAME)
            .collection(COLL_NAME);
        let count = collection.count_documents(None, None).await?;
        data.id = Some(count as u32);
        collection.insert_one(data.clone(), None).await?;
        Ok(())
    }

    pub async fn get_data(&self) -> Result<Vec<GasInfo>, Box<dyn std::error::Error>> {
        let collection: Collection<GasInfo> = self
            .client
            .lock()
            .unwrap()
            .database(DB_NAME)
            .collection(COLL_NAME);
        let mut cursor = collection.find(None, None).await?;
        let mut data = Vec::new();

        while let Some(result) = cursor.next().await {
            if let Ok(document) = result {
                data.push(document)
            }
        }

        Ok(data)
    }
}
