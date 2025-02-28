use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use alloy::{hex, primitives::B256, providers::ProviderBuilder, sol};
use chrono::Utc;
use elasticsearch::{
    auth::Credentials,
    http::{
        transport::{SingleNodeConnectionPool, TransportBuilder},
        Url,
    },
    BulkOperation, BulkParts, Elasticsearch,
};
use rand::Rng;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc;

const WORKER_NAME: &str = "firewall";
const BATCH_SIZE: usize = 5;
const THREADS_WORKERS: usize = 1;
const THREADS_DISPATCHERS: usize = 2;

const ELASTIC_URL: &str = "http://localhost:9200";
const ELASTIC_USERNAME: &str = "elastic";
const ELASTIC_PASSWORD: &str = "changeme";

const RPC_URL: &str = "http://localhost:8545";
const RPC_CONTRACT: &str = "0x5FbDB2315678afecb367f032d93F642f64180aa3";

struct AppState {
    sender: mpsc::Sender<Value>,
}

#[post("/")]
async fn receive(app: web::Data<Arc<AppState>>, data: web::Json<Value>) -> impl Responder {
    if let Err(_) = app.sender.send(data.clone()).await {
        return HttpResponse::InternalServerError().body("Failed to enqueue message.");
    }
    HttpResponse::Ok().body("Data received and being processed.")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (sender, receiver) = mpsc::channel(1024);
    let receiver = Arc::new(tokio::sync::Mutex::new(receiver));

    let app = web::Data::new(Arc::new(AppState { sender }));

    for i in 0..THREADS_WORKERS {
        let receiver = Arc::clone(&receiver);
        tokio::spawn(async move {
            let mut buffer = Vec::new();

            println!("Worker {} started.", i);

            while let Some(msg) = receiver.lock().await.recv().await {
                buffer.push(msg);

                if buffer.len() >= BATCH_SIZE {
                    println!(
                        "Worker {} processing batch of {} messages.",
                        i,
                        buffer.len()
                    );
                    proccess(&mut buffer).await;
                    buffer.clear();
                }
            }

            println!("Worker {} finished.", i);
        });
    }

    println!(
        "Starting HTTP server on 127.0.0.1:8080 with {} dispatchers.",
        THREADS_DISPATCHERS
    );

    HttpServer::new(move || App::new().app_data(app.clone()).service(receive))
        .workers(THREADS_DISPATCHERS)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn proccess(content: &mut Vec<Value>) {
    let index = generate_index_name();
    let mut hash = String::new();

    println!(
        "Processing batch of {} documents for index: {}",
        content.len(),
        index
    );

    for doc in content.iter() {
        let mut hasher = Sha256::new();
        hasher.update(hash.as_bytes());
        hasher.update(doc.to_string());
        hash = format!("{:x}", hasher.finalize());
    }

    println!("Generated hash for batch: {}", hash);

    let storage = Storage {
        url: ELASTIC_URL.into(),
        username: ELASTIC_USERNAME.into(),
        password: ELASTIC_PASSWORD.into(),
    };

    let ethereum = Ethereum {
        url: RPC_URL.into(),
        contract: RPC_CONTRACT.into(),
    };

    println!("Storing batch in Elasticsearch for index: {}", index);
    let _ = storage.store(&index, &content).await.unwrap();

    println!("Storing hash in Ethereum for index: {}", index);
    let receipt = ethereum.store(&index, &hash).await.unwrap();

    println!(
        "Batch processed successfully. Index: {}, Hash: {}, Ethereum Receipt: {}",
        index, hash, receipt
    );
}

fn generate_index_name() -> String {
    let current_time = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let random_string: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    format!("{}-{}-{}", current_time, WORKER_NAME, random_string).to_lowercase()
}

struct Storage {
    url: String,
    username: String,
    password: String,
}

impl Storage {
    async fn store(&self, index: &String, content: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let url = Url::parse(&self.url).unwrap();
        let pool: SingleNodeConnectionPool = SingleNodeConnectionPool::new(url);
        let credentials = Credentials::Basic(self.username.clone(), self.password.clone());
        let transport = TransportBuilder::new(pool)
            .auth(credentials)
            .build()
            .unwrap();
        let elastic = Elasticsearch::new(transport);

        let mut ops: Vec<BulkOperation<Value>> = Vec::new();

        for (i, d) in content.iter().enumerate() {
            ops.push(BulkOperation::create(d.clone()).id(i.to_string()).into());
        }

        let response = elastic
            .bulk(BulkParts::Index(&index))
            .body(ops)
            .send()
            .await?;

        if !response.status_code().is_success() {
            let error_body: Value = response.json().await?;
            eprintln!("Bulk request failed: {:?}", error_body);
        }

        Ok(())
    }
}

struct Ethereum {
    url: String,
    contract: String,
}

impl Ethereum {
    async fn store(&self, index: &String, hash: &String) -> Result<String, Box<dyn Error>> {
        let url = self.url.parse().unwrap();
        let provider = ProviderBuilder::new().on_http(url);
        let contract = self.contract.parse().unwrap();
        let contract = Auditability::new(contract, provider);

        let hash = B256::from_slice(&hex::decode(hash).unwrap());
        let tx = contract.store(index.clone(), hash).send().await.unwrap();
        let receipt = tx.get_receipt().await.unwrap();

        Ok(format!("{}", receipt.transaction_hash))
    }
}

sol! {
    #[sol(rpc)]
    contract Auditability {
        function store(string index, bytes32 root) external;
        function proof(string index, bytes32 root) external view returns (bool);
    }
}
