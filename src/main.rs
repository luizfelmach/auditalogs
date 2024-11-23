use sha2::{Digest, Sha256};

fn fingerprint(data: Vec<Vec<u8>>) -> String {
    let flat = data.concat();
    let mut hasher = Sha256::new();
    hasher.update(flat);
    let result = hasher.finalize();
    format!("0x{:x}", result)
}

fn main() {
    let nested: Vec<Vec<u8>> = vec![
        b"1 mensagem".to_vec(),
        b"2 mensagem".to_vec(),
        b"3 mensagem1".to_vec(),
    ];

    let hash = fingerprint(nested);

    println!("{hash}")
}
