use crate::entry::{Entry, EntryError};
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};

#[derive(Debug, thiserror::Error)]
pub enum LogError {
    #[error("file error regarding {file}")]
    FileError { file: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Entry error: {0}")]
    Entry(#[from] EntryError),
}
pub struct Log {
    filename: String,
    file: File,
}
impl Log {
    pub fn new(filename: &str) -> Result<Self, LogError> {
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
    pub fn open(&mut self) -> Result<(), LogError> {
        let file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&self.filename)
            .map_err(|_| LogError::FileError {
                file: self.filename.clone(),
            })?;
        Ok(())
    }
    pub fn close(&mut self) -> Result<(), LogError> {
        self.file.sync_all()?;
        Ok(())
    }
    pub fn write(&mut self, entry: &Entry) -> Result<(), LogError> {
        let data = entry.encode();
        self.file.write_all(&data)?;
        Ok(())
    }
    pub fn rewind(&mut self) -> Result<(), LogError> {
        self.file.seek(SeekFrom::Start(0))?;
        Ok(())
    }
    pub fn read(&mut self) -> Result<Entry, LogError> {
        let entry = Entry::decode(&mut self.file)?;
        Ok(entry)
    }
}
