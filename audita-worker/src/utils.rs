use chrono::Utc;
use rand::Rng;
use serde_json::Value;
use sha2::{Digest, Sha256};

pub fn fingerprint(content: &Vec<Value>) -> String {
    let mut hash = String::new();

    for doc in content.iter() {
        let mut hasher = Sha256::new();
        hasher.update(hash.as_bytes());
        hasher.update(doc.to_string());
        hash = format!("{:x}", hasher.finalize());
    }

    return hash;
}

pub fn generate_index(name: &String) -> String {
    let current_time = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let random_string: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    format!("{}-{}-{}", current_time, name, random_string).to_lowercase()
}
