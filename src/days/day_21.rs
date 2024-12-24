use std::{fs::File, path::PathBuf};
use std::env::temp_dir;
use std::fs::{remove_file, OpenOptions};
use std::io::{Read, Write};
use std::sync::atomic::{AtomicU32, Ordering};

static COUNT: AtomicU32 = AtomicU32::new(0);
pub struct TempFile {
    file_path: PathBuf,
    file: File
}

impl Drop for TempFile {
    fn drop(&mut self) {
        remove_file(&self.file_path);
    }
}

impl TempFile {
    pub fn new() -> Result<Self, std::io::Error> {
        let n = COUNT.fetch_add(1, Ordering::AcqRel);
        let mut file_path = std::env::temp_dir();
        file_path.push(&format!("xxx{}", n));
        let file = File::create(&file_path)?;
        Ok(Self{ file_path, file })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        self.file.write_all(data)
    }

    pub fn read_to_string(&mut self) -> Result<String, std::io::Error> {
        let mut file = OpenOptions::new().read(true).open(&self.file_path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(buffer)
    }

    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn file(&self) -> &File {
        &self.file
    }
}
