use chrono::Utc;
use rand::Rng;
use redis::Commands;
use std::{thread, time};

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

fn generate_log() -> String {
    let ip = generate_random_ip();
    let port = rand::thread_rng().gen_range(0..65536);
    let timestamp = Utc::now().format("%d/%b/%Y:%H:%M:%S %z").to_string();
    let mac = generate_random_mac();
    format!("{} - - [{}] Port: {}, MAC: {}", ip, timestamp, port, mac)
}

fn connect_redis() -> redis::Connection {
    let client = redis::Client::open("redis://127.0.0.1:6379").expect("Invalid Redis URL");
    client.get_connection().expect("Failed to connect to Redis")
}

fn send_logs_to_redis(redis_connection: &mut redis::Connection) {
    loop {
        let log = generate_log();
        let _: () = redis_connection
            .lpush("logs", log.clone())
            .expect("Failed to push log to Redis");
        println!("Enviado para Redis: {}", log);
        //thread::sleep(time::Duration::from_secs_f64(
        //    rand::thread_rng().gen_range(0.1..1.0),
        //));
    }
}

fn main() {
    let mut redis_connection = connect_redis();
    send_logs_to_redis(&mut redis_connection);
}
