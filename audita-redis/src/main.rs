use chrono::Utc;
use rand::Rng;
use redis::Commands;
use std::{thread, time};

const REDIS_KEY: &str = "logs";

fn generate_random_ip() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "{}.{}.{}.{}",
        rng.gen_range(0..256),
        rng.gen_range(0..256),
        rng.gen_range(0..256),
        rng.gen_range(0..256)
    )
}

fn generate_random_mac() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "00:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        rng.gen_range(0..256),
        rng.gen_range(0..256),
        rng.gen_range(0..256),
        rng.gen_range(0..256),
        rng.gen_range(0..256)
    )
}

fn generate_log() -> serde_json::Value {
    let ip = generate_random_ip();
    let port = rand::thread_rng().gen_range(0..65536);
    let timestamp = Utc::now().format("%d/%b/%Y:%H:%M:%S %z").to_string();
    let mac = generate_random_mac();
    serde_json::json!({
        "ip": ip,
        "timestamp": timestamp,
        "port": port,
        "mac": mac
    })
}

fn connect_redis() -> redis::Connection {
    let client = redis::Client::open("redis://127.0.0.1:6379").expect("Invalid Redis URL");
    client.get_connection().expect("Failed to connect to Redis")
}

fn send_logs_to_redis(redis_connection: &mut redis::Connection, delay: Option<u64>) {
    loop {
        let log = generate_log();
        let log_string = log.to_string();
        let _: () = redis_connection
            .lpush(REDIS_KEY, log_string.clone())
            .expect("Failed to push log to Redis");
        println!("Sent to Redis: {}", log_string);

        if let Some(delay_ms) = delay {
            thread::sleep(time::Duration::from_millis(delay_ms));
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let delay = if args.len() > 1 {
        args[1].parse::<u64>().ok()
    } else {
        None
    };

    let mut redis_connection = connect_redis();
    send_logs_to_redis(&mut redis_connection, delay);
}
