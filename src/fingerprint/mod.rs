use sha2::{Digest, Sha256};

pub trait Fingerprint {
    fn fingerprint(&self) -> String;
}

impl Fingerprint for Vec<Vec<u8>> {
    fn fingerprint(&self) -> String {
        let flat = self.concat();
        let mut hasher = Sha256::new();
        hasher.update(flat);
        let result = hasher.finalize();
        format!("0x{:x}", result)
    }
}
