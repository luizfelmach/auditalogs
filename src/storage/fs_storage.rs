use std::fs::{File, OpenOptions};
use std::io::Write;

use super::StorageStore;

pub struct FsStorage {
    file: File,
}

impl Default for FsStorage {
    fn default() -> Self {
        let path = "storage.txt";
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .unwrap();

        FsStorage { file }
    }
}

impl StorageStore for FsStorage {
    async fn store(
        &mut self,
        id: &String,
        data: &Vec<Vec<u8>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        writeln!(self.file, "ID: {id}")?;
        for i in data {
            let s = String::from_utf8(i.to_vec())?;
            writeln!(self.file, "{s}")?;
        }
        writeln!(self.file, "\n")?;
        Ok(())
    }
}
