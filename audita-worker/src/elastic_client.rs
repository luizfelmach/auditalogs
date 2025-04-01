use elasticsearch::{
    auth::Credentials,
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
    BulkOperation, BulkParts, Elasticsearch,
};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ElasticClientError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Failed to create transport: {0}")]
    TransportError(String),

    #[error("Bulk request failed: {0}")]
    BulkRequestError(String),

    #[error("Failed to parse response JSON: {0}")]
    ResponseParseError(String),
}

pub struct ElasticClient {
    provider: Elasticsearch,
}

impl ElasticClient {
    pub fn new(
        url: String,
        username: String,
        password: String,
    ) -> Result<Self, ElasticClientError> {
        let url = url
            .parse()
            .map_err(|_| ElasticClientError::InvalidUrl(url))?;
        let pool = SingleNodeConnectionPool::new(url);
        let credentials = Credentials::Basic(username, password);
        let transport = TransportBuilder::new(pool)
            .auth(credentials)
            .cert_validation(elasticsearch::cert::CertificateValidation::None)
            .build()
            .map_err(|e| ElasticClientError::TransportError(e.to_string()))?;
        let provider = Elasticsearch::new(transport);

        Ok(Self { provider })
    }

    pub async fn store(
        &self,
        index: &String,
        content: &Vec<Value>,
    ) -> Result<(), ElasticClientError> {
        let mut ops: Vec<BulkOperation<Value>> = Vec::new();

        for (i, d) in content.iter().enumerate() {
            ops.push(BulkOperation::create(d.clone()).id(i.to_string()).into());
        }

        let response = self
            .provider
            .bulk(BulkParts::Index(index))
            .body(ops)
            .send()
            .await
            .map_err(|e| ElasticClientError::TransportError(e.to_string()))?;

        if !response.status_code().is_success() {
            let error_body: Value = response
                .json()
                .await
                .map_err(|e| ElasticClientError::ResponseParseError(e.to_string()))?;
            return Err(ElasticClientError::BulkRequestError(error_body.to_string()));
        }

        Ok(())
    }
}
