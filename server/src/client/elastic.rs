use elasticsearch::{
    auth::Credentials,
    cert::CertificateValidation,
    http::transport::{SingleNodeConnectionPool, Transport, TransportBuilder},
    Elasticsearch, IndexParts, ScrollParts, SearchParts,
};
use serde_json::{json, Value};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ElasticClientError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Transport creation failed: {0}")]
    TransportError(String),

    #[error("Response JSON parsing failed: {0}")]
    ResponseParseError(String),

    #[error("Indexing failed: {0}")]
    IndexRequestError(String),
}

#[derive(Clone)]
pub struct ElasticClient {
    client: Elasticsearch,
}

impl ElasticClient {
    pub fn new(
        url: String,
        username: String,
        password: String,
    ) -> Result<Self, ElasticClientError> {
        let transport = Self::build_transport(&url, &username, &password)?;
        Ok(Self {
            client: Elasticsearch::new(transport),
        })
    }

    fn build_transport(
        url: &str,
        username: &str,
        password: &str,
    ) -> Result<Transport, ElasticClientError> {
        let parsed_url = url
            .parse()
            .map_err(|_| ElasticClientError::InvalidUrl(url.to_owned()))?;

        let pool = SingleNodeConnectionPool::new(parsed_url);
        let credentials = Credentials::Basic(username.to_owned(), password.to_owned());

        TransportBuilder::new(pool)
            .auth(credentials)
            .cert_validation(CertificateValidation::None)
            .build()
            .map_err(|e| ElasticClientError::TransportError(e.to_string()))
    }

    pub async fn store(&self, index: &str, content: &Value) -> Result<(), ElasticClientError> {
        let response = self
            .client
            .index(IndexParts::Index(index))
            .body(content)
            .send()
            .await
            .map_err(|e| ElasticClientError::TransportError(e.to_string()))?;

        if !response.status_code().is_success() {
            let error_body: Value = response
                .json()
                .await
                .map_err(|e| ElasticClientError::ResponseParseError(e.to_string()))?;
            return Err(ElasticClientError::IndexRequestError(
                error_body.to_string(),
            ));
        }

        Ok(())
    }

    pub async fn retrieve(&self, index: &str) -> Result<Vec<Value>, ElasticClientError> {
        let mut docs = Vec::new();

        let response = self
            .client
            .search(SearchParts::Index(&[index]))
            .scroll("1m")
            .body(json!({ "query": { "match_all": {} } }))
            .send()
            .await
            .map_err(|e| ElasticClientError::IndexRequestError(e.to_string()))?;

        let response_body = response
            .json::<Value>()
            .await
            .map_err(|e| ElasticClientError::ResponseParseError(e.to_string()))?;

        let mut scroll_id = response_body["_scroll_id"]
            .as_str()
            .ok_or_else(|| {
                ElasticClientError::ResponseParseError("Missing _scroll_id".to_string())
            })?
            .to_string();

        if let Some(hits) = response_body["hits"]["hits"].as_array() {
            docs.extend(hits.clone());
        }

        loop {
            let scroll_response = self
                .client
                .scroll(ScrollParts::None)
                .scroll("1m")
                .body(json!({ "scroll_id": scroll_id }))
                .send()
                .await
                .map_err(|e| ElasticClientError::IndexRequestError(e.to_string()))?;

            let scroll_body = scroll_response
                .json::<Value>()
                .await
                .map_err(|e| ElasticClientError::ResponseParseError(e.to_string()))?;

            let hits = scroll_body["hits"]["hits"].as_array().ok_or_else(|| {
                ElasticClientError::ResponseParseError("Missing hits".to_string())
            })?;

            if hits.is_empty() {
                break;
            }

            docs.extend(hits.clone());

            scroll_id = scroll_body["_scroll_id"]
                .as_str()
                .ok_or_else(|| {
                    ElasticClientError::ResponseParseError("Missing _scroll_id".to_string())
                })?
                .to_string();
        }

        Ok(docs)
    }
}
