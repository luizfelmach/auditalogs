use chrono::Utc;
use rand::{distributions::Alphanumeric, Rng};
use sha2::{Digest, Sha256};

pub fn fingerprint(first: &String, second: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(first.as_bytes());
    hasher.update(second.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn elastic_index(name: &String) -> String {
    let current_time = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    format!("{current_time}-{name}-{random_string}").to_lowercase()
}
