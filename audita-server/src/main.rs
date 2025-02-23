use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use alloy::{hex, sol};
use alloy::{primitives::B256, providers::ProviderBuilder};
use elasticsearch::{
    auth::Credentials,
    http::{
        transport::{SingleNodeConnectionPool, TransportBuilder},
        Url,
    },
    Elasticsearch, SearchParts,
};
use serde_json::json;
use sha2::{Digest, Sha256};

const ELASTIC_URL: &str = "http://localhost:9200";
const ELASTIC_USERNAME: &str = "elastic";
const ELASTIC_PASSWORD: &str = "changeme";

const RPC_URL: &str = "http://localhost:8545";
const RPC_CONTRACT: &str = "0x5FbDB2315678afecb367f032d93F642f64180aa3";

#[get("/search")]
async fn search(
    es_client: web::Data<Elasticsearch>,
    body: web::Json<serde_json::Value>,
) -> impl Responder {
    let query = body.into_inner();

    let response = es_client
        .search(SearchParts::Index(&["*"]))
        .body(json!({
            "query": query,
            "size": 10000
        }))
        .send()
        .await;

    match response {
        Ok(resp) => match resp.json::<serde_json::Value>().await {
            Ok(json) => {
                let hits = json["hits"]["hits"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|doc| {
                        json!({
                            "index": doc["_index"],
                            "id": doc["_id"],
                            "source": doc["_source"]
                        })
                    })
                    .collect::<Vec<_>>();

                HttpResponse::Ok().json(hits)
            }
            Err(_) => HttpResponse::InternalServerError().body("Erro ao processar resposta"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Erro ao buscar no Elasticsearch"),
    }
}

#[get("/proof/{index}")]
async fn proof(es_client: web::Data<Elasticsearch>, index: web::Path<String>) -> impl Responder {
    let response = es_client
        .search(SearchParts::Index(&[index.as_str()]))
        .body(json!({
            "query": { "match_all": {} },
            "size": 10000
        }))
        .send()
        .await;

    match response {
        Ok(resp) => match resp.json::<serde_json::Value>().await {
            Ok(json) => {
                let aux = vec![];
                let hits = json["hits"]["hits"].as_array().unwrap_or(&aux);

                let mut accumulated_hash = String::new();

                for doc in hits {
                    let mut hasher = Sha256::new();
                    hasher.update(accumulated_hash.as_bytes());
                    hasher.update(doc["_source"].to_string().as_bytes());
                    accumulated_hash = format!("{:x}", hasher.finalize());
                }

                let provider = ProviderBuilder::new().on_http(RPC_URL.parse().unwrap());
                let contract_address = RPC_CONTRACT.parse().unwrap();
                let contract = Auditability::new(contract_address, provider);
                let blockchain_hash = contract
                    .proof(
                        index.clone(),
                        B256::from_slice(&hex::decode(&accumulated_hash).unwrap()),
                    )
                    .call()
                    .await
                    .unwrap();

                println!("{}", accumulated_hash);

                HttpResponse::Ok()
                    .json(json!({ "calculated_hash": accumulated_hash, "blockchain_match": blockchain_hash._0 }))
            }
            Err(_) => HttpResponse::InternalServerError().body("Erro ao processar resposta"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Erro ao buscar no Elasticsearch"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = Url::parse(ELASTIC_URL).unwrap();
    let pool: SingleNodeConnectionPool = SingleNodeConnectionPool::new(url);
    let credentials = Credentials::Basic(ELASTIC_USERNAME.into(), ELASTIC_PASSWORD.into());
    let transport = TransportBuilder::new(pool)
        .auth(credentials)
        .build()
        .unwrap();
    let es_client = Elasticsearch::new(transport);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(es_client.clone()))
            .service(search)
            .service(proof)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

sol! {
    #[sol(rpc)]
    contract Auditability {
        function store(string index, bytes32 root) external;
        function proof(string index, bytes32 root) external view returns (bool);
    }
}
