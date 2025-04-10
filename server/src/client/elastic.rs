use crate::{channel::ElasticChannelItem, utils::fingerprint};
use anyhow::{anyhow, Result};
use elasticsearch::{
    auth::Credentials,
    cert::CertificateValidation,
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
    BulkOperation, BulkParts, Elasticsearch, SearchParts,
};
use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub struct ElasticClient {
    client: Elasticsearch,
}

impl ElasticClient {
    pub fn new(url: String, username: String, password: String) -> Result<Self> {
        let pool = SingleNodeConnectionPool::new(url.parse()?);
        let credentials = Credentials::Basic(username, password);
        let transport = TransportBuilder::new(pool)
            .auth(credentials)
            .cert_validation(CertificateValidation::None)
            .build()?;

        Ok(Self {
            client: Elasticsearch::new(transport),
        })
    }

    pub async fn store(&self, items: Vec<ElasticChannelItem>) -> Result<()> {
        if items.is_empty() {
            return Ok(());
        }

        let mut ops: Vec<BulkOperation<Value>> = Vec::new();

        for item in items.iter() {
            let content = serde_json::from_str(&item.content)?;
            ops.push(BulkOperation::create(content).index(&item.index).into());
        }

        let response = self.client.bulk(BulkParts::None).body(ops).send().await?;
        let status = response.status_code();

        if !response.status_code().is_success() {
            let error_body: Value = response.json().await?;
            return Err(anyhow!("bulk insert failed ({}): {}", status, error_body));
        }

        Ok(())
    }

    pub async fn exists(&self, index: &str) -> Result<bool> {
        let response = self
            .client
            .indices()
            .exists(elasticsearch::indices::IndicesExistsParts::Index(&[index]))
            .send()
            .await?;
        let status = response.status_code();
        Ok(status == 200)
    }

    pub async fn hash(&self, index: &str) -> Result<String> {
        match self.exists(index).await {
            Ok(true) => (),
            Ok(false) => return Err(anyhow!("index {} does not exists", index)),
            Err(err) => return Err(err),
        }

        let items = self.retrieve(index).await?;
        let mut hash = String::new();

        for item in items {
            let Some(source) = item.get("_source") else {
                continue;
            };
            hash = fingerprint(&hash, &source.to_string());
        }

        Ok(hash)
    }

    pub async fn retrieve(&self, index: &str) -> Result<Vec<Value>> {
        let mut docs = Vec::new();
        let mut last: Option<Value> = None;

        loop {
            let mut query = json!({
                "size": 10_000,
                "sort": [{"_doc": "asc"}],
                "query": {"match_all": {}}
            });

            if let Some(valor) = &last {
                query["search_after"] = valor.clone();
            }

            let response = self
                .client
                .search(SearchParts::Index(&[index]))
                .body(query)
                .send()
                .await?;

            let body = response.json::<Value>().await?;
            let hits = body["hits"]["hits"]
                .as_array()
                .ok_or_else(|| anyhow!("invalid search response for index '{}'", index))?;

            if hits.is_empty() {
                break;
            }

            match hits.last() {
                Some(last_hit) => last = Some(last_hit["sort"].clone()),
                None => {
                    return Err(anyhow!(
                        "error fetching documents: missing field sort in request"
                    ))
                }
            }

            docs.extend(hits.to_vec());
        }

        Ok(docs)
    }
}
