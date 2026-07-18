
#[derive(Debug,thiserror::Error)]
pub enum EntryError{
    #[error("Invalid header")]
    InvalidHeader,
    #[error("buffer too short to decode entry")]
    BufferTooShort
}
#[derive(Debug)]
pub struct Entry {
    key: Vec<u8>,
    val: Vec<u8>
}

impl Entry {
    pub fn new(key: Vec<u8>, val: Vec<u8>) -> Self {
        Self { key, val }
    }
    pub fn encode(&self) -> Vec<u8> {
        let kl = self.key.len() as u32;
        let vl = self.val.len() as u32;
        let mut data = Vec::with_capacity(4+4+kl as usize + vl as usize);
        data.extend_from_slice(&kl.to_le_bytes());
        data.extend_from_slice(&vl.to_le_bytes());
        data.extend_from_slice(&self.key);
        data.extend_from_slice(&self.val);
        data
    }
    pub fn decode(data: Vec<u8>) -> Result<Self, EntryError > {
        if data.len() < 8 {
            return Err(EntryError::InvalidHeader);
        }
        let kl = u32::from_le_bytes(data[0..4].try_into().unwrap()) as usize;
        let vl = u32::from_le_bytes(data[4..8].try_into().unwrap()) as usize;

        let req_len = 8usize.saturating_add(kl).saturating_add(vl);
        if data.len() < req_len {
            return Err(EntryError::BufferTooShort);
        }
        let key = data[8..8+ kl].to_vec();
        let val = data[(8+kl)..(8+kl+vl)].to_vec();
        Ok(Entry::new(key,val))
    }
}

mod tests {
    use crate::entry::{Entry, EntryError};

    #[test]
    fn test_entry() {
        let key = vec![36, 64];
        let val = vec![64, 100];
        let ent = Entry::new(key.clone(), val.clone());

        let data = ent.encode();
        assert_eq!(data.len(), 12, "Encoded data should be exactly 12 bytes");

        let decoded = Entry::decode(data.clone()).unwrap();
        assert_eq!(decoded.key, key);
        assert_eq!(decoded.val, val);

        let short_header = vec![0, 1, 2, 3, 4];
        let err = Entry::decode(short_header).unwrap_err();
        assert!(
            matches!(err, EntryError::InvalidHeader),
            "Expected InvalidHeader, got {:?}", err
        );

        let mut corrupted_data = data;
        corrupted_data.pop();

        let err = Entry::decode(corrupted_data).unwrap_err();
        assert!(
            matches!(err, EntryError::BufferTooShort),
            "Expected BufferTooShort, got {:?}", err
        );
    }
    
}