use std::{sync::{Arc, Mutex}, ops::Deref};

use crate::models::GasInfo;
use futures::{stream::TryStreamExt, future::ok};
use mongodb::{Client, Collection, Cursor};
use serde::Serialize;

#[derive(Clone)]
pub struct Database<T>
where
    T: Serialize,
    T: Clone,
    T: ?Sized
{
    client: Arc<Mutex<Client>>,
    db_name: Arc<Mutex<String>>,
    coll_name: Arc<Mutex<String>>,
    item: Option<T>
}

impl<T> Database<T>
where
    T: Serialize,
    T: Clone,
    T: ?Sized
{
    pub async fn new(db_name: String, collection_name: String) -> Database<T> {
        let uri =
            std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
        let client = Client::with_uri_str(uri).await.expect("failed to connect");
        Database {
            client: Arc::new(Mutex::new(client)),
            db_name: Arc::new(Mutex::new(db_name)),
            coll_name: Arc::new(Mutex::new(collection_name)),
            item: None
        }
    }

    pub async fn add_data(&self, data: &mut T) -> mongodb::error::Result<()> {
        let db_name_mutex = &self.db_name.lock().unwrap();
        let coll_name_mutex = &self.db_name.lock().unwrap();
        //let db_name = &self.db_name.lock().unwrap().as_ref();
        //let coll_name = &self.coll_name.lock().unwrap().deref();
        let collection = self
            .client
            .lock()
            .unwrap()
            .database(&db_name_mutex)
            .collection(&coll_name_mutex);
        let count = collection.count_documents(None, None).await?;
        //data.id = Some(count as u32);
        collection.insert_one(data.clone(), None).await?;
        Ok(())
    }

    pub async fn get_data(&self) -> Result<Vec<T>, Box<dyn std::error::Error>> {
        let db_name_mutex = &self.db_name.lock().unwrap();
        let coll_name_mutex = &self.db_name.lock().unwrap();
        let collection: Collection<T> = self
            .client
            .lock()
            .unwrap()
            .database(&db_name_mutex)
            .collection(&coll_name_mutex);
        let mut cursor = collection.find(None, None).await?;
        let mut data = Vec::new();
        while let Some(item) = cursor
    }
}
