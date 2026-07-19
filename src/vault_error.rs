#[derive(Debug, thiserror::Error)]
pub enum KeyValueError {
    #[error("{key:?} not found")]
    KeyNotFound { key: Vec<u8> },
}
#[derive(Debug, thiserror::Error)]
pub enum EntryError {
    #[error("Invalid header")]
    InvalidHeader,
    #[error("buffer too short to decode entry")]
    BufferTooShort,
    #[error("Checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: u32, actual: u32 },
}
#[derive(Debug, thiserror::Error)]
pub enum LogError {
    #[error("file error regarding {file}")]
    FileError { file: String },
}

#[derive(Debug, thiserror::Error)]
pub enum VaultError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Entry error: {0}")]
    Entry(#[from] EntryError),
    #[error("Log error: {0}")]
    Log(#[from] LogError),
    #[error("KeyValueError: {0}")]
    KeyValueError(#[from] KeyValueError),
}
