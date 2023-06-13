use mongodb::{Client, Collection, error::Result, options::IndexOptions};
use crate::models::GasInfo;

const DB_NAME: &str = "HomeDB";
const COLL_NAME: &str = "cartrax";

struct Database {
    client: Client,
}

impl Database {
    async fn new() -> Database {
        let uri =
            std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
        let client = Client::with_uri_str(uri).await.expect("failed to connect");
        Database { client }
    }

    async fn create_idx(&self) {
        let options = IndexOptions::builder().unique(true);
    }

    async fn add_data(&self, data: GasInfo) -> Result<()> {
        let collection = self.client.database(DB_NAME).collection(COLL_NAME);
        collection.insert_one(data, None).await?;
        Ok(())
    }

    async fn get_data(&self, data: GasInfo) -> Result<Vec<GasInfo>> {
        let collection: Collection<GasInfo> = self.client.database(DB_NAME).collection(COLL_NAME);
        let data = collection.find(None, None).await?;
        data.collect();
    }
}
