use crate::vault_error::KeyValueError;
use crate::vault_error::VaultError;
use std::collections::HashMap;

#[derive(Default)]
pub struct KeyValue {
    memory: HashMap<Vec<u8>, Vec<u8>>,
}
impl KeyValue {
    pub fn new() -> Self {
        Self {
            memory: HashMap::new(),
        }
    }
    pub fn open(&mut self) -> Result<(), VaultError> {
        self.memory.clear();
        Ok(())
    }
    pub fn close(&self) -> Result<(), VaultError> {
        Ok(())
    }
    pub fn delete(&mut self, key: Vec<u8>) -> Result<(), VaultError> {
        match self.memory.remove(&key) {
            Some(_) => Ok(()),
            None => Err(VaultError::KeyValueError(KeyValueError::KeyNotFound {
                key,
            })),
        }
    }
    pub fn get(&self, key: Vec<u8>) -> Result<Vec<u8>, VaultError> {
        self.memory
            .get(&key)
            .cloned()
            .ok_or(VaultError::KeyValueError(KeyValueError::KeyNotFound {
                key,
            }))
    }
    pub fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<bool, VaultError> {
        match self.memory.entry(key) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                if entry.get() == &value {
                    Ok(false)
                } else {
                    entry.insert(value);
                    Ok(true)
                }
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(value);
                Ok(true)
            }
        }
    }
}
