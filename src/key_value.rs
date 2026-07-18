use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum KeyValueError {
    #[error("{key:?} not found")]
    KeyNotFound { key: Vec<u8> },
}
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
    pub fn open(&mut self) -> Result<(), KeyValueError> {
        self.memory.clear();
        Ok(())
    }
    pub fn close(&self) -> Result<(), KeyValueError> {
        Ok(())
    }
    pub fn delete(&mut self, key: Vec<u8>) -> Result<(), KeyValueError> {
        match self.memory.remove(&key) {
            Some(_) => Ok(()),
            None => Err(KeyValueError::KeyNotFound { key }),
        }
    }
    pub fn get(&self, key: Vec<u8>) -> Result<Vec<u8>, KeyValueError> {
        self.memory
            .get(&key)
            .cloned()
            .ok_or(KeyValueError::KeyNotFound { key })
    }
    pub fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<bool, KeyValueError> {
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
