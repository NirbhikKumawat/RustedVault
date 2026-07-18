use crate::entry::Entry;
use crate::vault_error::{LogError, VaultError};
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};

pub struct Log {
    filename: String,
    file: File,
}
impl Log {
    pub fn new(filename: &str) -> Result<Self, VaultError> {
        let file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(filename)
            .map_err(|_| LogError::FileError {
                file: filename.to_string(),
            })?;
        Ok(Self {
            file,
            filename: filename.to_string(),
        })
    }
    pub fn open(&mut self) -> Result<(), VaultError> {
        self.file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&self.filename)
            .map_err(|_| LogError::FileError {
                file: self.filename.clone(),
            })?;
        Ok(())
    }
    pub fn close(&mut self) -> Result<(), VaultError> {
        self.file.sync_all()?;
        Ok(())
    }
    pub fn write(&mut self, entry: &Entry) -> Result<(), VaultError> {
        let data = entry.encode();
        self.file.write_all(&data)?;
        Ok(())
    }
    pub fn rewind(&mut self) -> Result<(), VaultError> {
        self.file.seek(SeekFrom::Start(0))?;
        Ok(())
    }
    pub fn read(&mut self) -> Result<Entry, VaultError> {
        let entry = Entry::decode(&mut self.file)?;
        Ok(entry)
    }
}
