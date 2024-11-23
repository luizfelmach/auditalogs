use sha2::{Digest, Sha256};

pub fn fingerprint(data: &Vec<Vec<u8>>) -> String {
    let flat = data.concat();
    let mut hasher = Sha256::new();
    hasher.update(flat);
    let result = hasher.finalize();
    format!("0x{:x}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(
            fingerprint(&vec![
                b"Some data".to_vec(),
                b"Another some data".to_vec(),
                b"Yet another some data".to_vec()
            ]),
            "0x73f88e82a5e709f2c4321bd53a7ddb5aef07a004a93e7fd807fdee69cbae8c22"
        );
    }
}
