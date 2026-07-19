use crate::entry::Entry;
use crate::log::Log;
use crate::vault_error::KeyValueError;
use crate::vault_error::VaultError;
use std::collections::HashMap;
use std::io::ErrorKind;

pub struct KeyValue {
    memory: HashMap<Vec<u8>, Vec<u8>>,
    log: Log,
}
impl KeyValue {
    pub fn new(filename: &str) -> Result<Self, VaultError> {
        Ok(Self {
            memory: HashMap::new(),
            log: Log::new(filename)?,
        })
    }
    pub fn open(&mut self) -> Result<(), VaultError> {
        self.log.open()?;
        self.memory.clear();

        self.log.rewind()?;

        loop {
            match self.log.read() {
                Ok(entry) => {
                    if entry.deleted {
                        self.memory.remove(&entry.key);
                    } else {
                        self.memory.insert(entry.key, entry.val);
                    }
                }
                Err(VaultError::Io(err)) if err.kind() == ErrorKind::UnexpectedEof => {
                    break;
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }

        Ok(())
    }
    pub fn close(&mut self) -> Result<(), VaultError> {
        self.log.close()?;
        Ok(())
    }
    pub fn delete(&mut self, key: Vec<u8>) -> Result<(), VaultError> {
        match self.memory.remove(&key) {
            Some(val) => {
                let entry = Entry::new(&key, &val, true);
                self.log.write(&entry)?;
                Ok(())
            }
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
        let k = key.clone();
        match self.memory.entry(key) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                if entry.get() == &value {
                    Ok(false)
                } else {
                    self.log.write(&Entry::new(&k, &value, false))?;
                    entry.insert(value);
                    Ok(true)
                }
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                self.log.write(&Entry::new(&k, &value, false))?;
                entry.insert(value);
                Ok(true)
            }
        }
    }
}
