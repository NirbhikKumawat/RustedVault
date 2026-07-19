use crate::vault_error::{CellError, VaultError};

pub enum CellType {
    TypeI64(i64),
    TypeStr(String),
}
pub struct Cell {
    value: CellType,
}
impl Cell {
    pub fn new(value: CellType) -> Self {
        Self { value }
    }
    pub fn encode(&self, to_append: &mut Vec<u8>) {
        match &self.value {
            CellType::TypeI64(val) => {
                to_append.extend_from_slice(&val.to_le_bytes());
            }
            CellType::TypeStr(val) => {
                to_append.extend_from_slice(&(val.len() as u32).to_le_bytes());
                to_append.extend_from_slice(val.as_bytes());
            }
        }
    }
    pub fn decode(&mut self, data: Vec<u8>) -> Result<Vec<u8>, VaultError> {
        match &self.value {
            CellType::TypeI64(_) => {
                if data.len() < 8 {
                    return Err(VaultError::CellError(CellError::DataShortage {
                        expected: 8,
                        actual: data.len(),
                    }));
                }
                self.value = CellType::TypeI64(i64::from_le_bytes(data[0..8].try_into().unwrap()));
                Ok(data[8..].to_vec())
            }
            CellType::TypeStr(_) => {
                if data.len() < 4 {
                    return Err(VaultError::CellError(CellError::DataShortage {
                        expected: 4,
                        actual: data.len(),
                    }));
                }
                let size = u32::from_le_bytes(data[0..4].try_into().unwrap()) as usize;
                if data.len() < size + 4 {
                    return Err(VaultError::CellError(CellError::DataShortage {
                        expected: 4 + size,
                        actual: data.len(),
                    }));
                }
                self.value = CellType::TypeStr(
                    String::from_utf8(data[4..4 + size].to_vec())
                        .map_err(|_| VaultError::CellError(CellError::InvalidUTF8))?,
                );
                Ok(data[4 + size..].to_vec())
            }
        }
    }
}
