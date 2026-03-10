use async_trait::async_trait;
use crate::tools::Tool;
use mongodb::{Client, options::ClientOptions, bson::Document};
use futures::stream::StreamExt;

#[derive(Debug)]
pub struct MongoTool {
    url: String,
    db_name: String,
}

impl MongoTool {
    pub fn new(url: &str, db_name: &str) -> Self {
        Self { url: url.to_string(), db_name: db_name.to_string() }
    }
}

#[async_trait]
impl Tool for MongoTool {
    fn name(&self) -> &str {
        "mongodb"
    }

    fn description(&self) -> &str {
        "Query a MongoDB document store for JSON data retrieval."
    }

    async fn execute(&self, collection_query: &str) -> Result<String, String> {
        let options = ClientOptions::parse(&self.url).await.map_err(|e| e.to_string())?;
        let client = Client::with_options(options).map_err(|e| e.to_string())?;
        let db = client.database(&self.db_name);
        
        // Input format: "collection_name"
        let collection = db.collection::<Document>(collection_query);
        let mut cursor = collection.find(mongodb::bson::doc! {}).await.map_err(|e| e.to_string())?;

        let mut results = Vec::new();
        while let Some(result) = cursor.next().await {
            match result {
                Ok(doc) => results.push(doc.to_string()),
                Err(e) => return Err(e.to_string()),
            }
        }
        Ok(results.join("\n"))
    }
}
