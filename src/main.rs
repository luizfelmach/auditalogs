use alloy::{
    hex,
    primitives::{Address, B256},
    providers::ProviderBuilder,
    sol,
};
use chrono::Utc;
use elasticsearch::{
    auth::Credentials,
    http::{
        transport::{SingleNodeConnectionPool, TransportBuilder},
        Url,
    },
    Elasticsearch,
};
use rand::Rng;
use redis::Commands;
use serde_json::json;
use sha2::{Digest, Sha256};

const REDIS_URL: &str = "redis://127.0.0.1";
const REDIS_KEY: &str = "logs";
const BATCH_SIZE: usize = 2;

const ELASTIC_URL: &str = "http://localhost:9200";
const ELASTIC_USERNAME: &str = "elastic";
const ELASTIC_PASSWORD: &str = "changeme";

const RPC_URL: &str = "http://localhost:8545";
const RPC_CONTRACT: &str = "0x5FbDB2315678afecb367f032d93F642f64180aa3";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let redis = redis::Client::open(REDIS_URL)?;
    let mut redis = redis.get_connection()?;

    let url = Url::parse(ELASTIC_URL)?;
    let pool = SingleNodeConnectionPool::new(url);
    let credentials = Credentials::Basic(ELASTIC_USERNAME.into(), ELASTIC_PASSWORD.into());
    let transport = TransportBuilder::new(pool).auth(credentials).build()?;
    let elastic = Elasticsearch::new(transport);

    let mut batching: Vec<Vec<u8>> = vec![];

    loop {
        let msg: Option<(String, String)> = redis.brpop(REDIS_KEY, 0.0)?;
        if let Some((_, value)) = msg {
            batching.push(value.into());
        }
        if batching.len() < BATCH_SIZE {
            continue;
        }
        let flat: Vec<Vec<u8>> = batching.iter().map(|m| m.clone()).collect();
        let hash = fingerprint(flat);

        let index_name = generate_index_name();

        for msg in &batching {
            let res = elastic
                .index(elasticsearch::IndexParts::Index(&index_name))
                .body(json!({
                    "message": String::from_utf8_lossy(msg)
                }))
                .send()
                .await;

            match res {
                Ok(res) => {
                    if res.status_code().is_success() {
                        println!(
                            "Mensagem indexada com sucesso para o índice: {}",
                            index_name
                        );
                    } else {
                        eprintln!("Erro ao indexar a mensagem. Status: {}", res.status_code());
                        if let Ok(body) = res.json::<serde_json::Value>().await {
                            eprintln!("Detalhes do erro: {:?}", body);
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Erro ao tentar enviar a requisição para o Elasticsearch: {}",
                        e
                    );
                }
            }
        }

        let provider = ProviderBuilder::new().on_http(RPC_URL.parse()?);

        let contract_address: Address = RPC_CONTRACT.parse()?;

        let contract = Auditability::new(contract_address, provider);

        let hash = B256::from_slice(&hex::decode(&hash[2..])?);
        let _ = contract.store(index_name, hash).call().await?;

        batching.clear();
    }
}

fn fingerprint(data: Vec<Vec<u8>>) -> String {
    let flat = data.concat();
    let mut hasher = Sha256::new();
    hasher.update(flat);
    let result = hasher.finalize();
    format!("0x{:x}", result)
}

fn generate_index_name() -> String {
    let current_time = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let random_string: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    format!("teste-{}-{}", current_time, random_string).to_lowercase()
}

sol! {
    #[sol(rpc)]
    contract Auditability {
        function store(string index, bytes32 root) external;
        function proof(string index, bytes32 root) external view returns (bool);
    }
}
