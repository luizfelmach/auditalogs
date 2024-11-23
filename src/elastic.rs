use elasticsearch::{
    auth::Credentials,
    http::{
        request::JsonBody,
        transport::{SingleNodeConnectionPool, TransportBuilder},
        Url,
    },
    BulkParts, Elasticsearch,
};
use futures_lite::future::block_on;
use serde_json::{json, Value};
use std::error::Error;

#[derive(Debug)]
pub struct Elastic {
    client: Elasticsearch,
}

impl Elastic {
    pub fn new(url: &str, user: &str, password: &str) -> Result<Self, Box<dyn Error>> {
        let url = Url::parse(url)?;
        let pool = SingleNodeConnectionPool::new(url);
        let credentials = Credentials::Basic(user.into(), password.into());
        let transport = TransportBuilder::new(pool).auth(credentials).build()?;
        Ok(Elastic {
            client: Elasticsearch::new(transport),
        })
    }

    pub fn store_data(&self, index: &String, data: &Vec<Vec<u8>>) -> Result<(), Box<dyn Error>> {
        let mut bulk_operations: Vec<JsonBody<Value>> = Vec::new();
        for (i, d) in data.iter().enumerate() {
            bulk_operations.push(
                json!({
                    "index": {"_index": index, "_id": i},
                })
                .into(),
            );
            let json: Value = serde_json::from_slice(d)?;
            bulk_operations.push(json.into());
        }
        let result = block_on(async {
            self.client
                .bulk(BulkParts::Index(index))
                .body(bulk_operations)
                .send()
                .await
        });

        if let Err(error) = result {
            return Err(Box::new(error));
        }

        Ok(())
    }
}
