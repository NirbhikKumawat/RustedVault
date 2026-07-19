use crate::vault_error::VaultError;
use std::io::Read;

#[derive(Debug)]
pub struct Entry {
    pub key: Vec<u8>,
    pub val: Vec<u8>,
    pub deleted: bool,
}

impl Entry {
    pub fn new(key: &Vec<u8>, val: &Vec<u8>, deleted: bool) -> Self {
        Self {
            key: key.clone(),
            val: val.clone(),
            deleted,
        }
    }
    pub fn encode(&self) -> Vec<u8> {
        let kl = self.key.len() as u32;
        let vl = self.val.len() as u32;
        let mut data = Vec::with_capacity(4 + 4 + 1 + kl as usize + vl as usize);
        data.extend_from_slice(&kl.to_le_bytes());
        data.extend_from_slice(&vl.to_le_bytes());
        if self.deleted {
            data.extend_from_slice(&[1u8]);
        } else {
            data.extend_from_slice(&[0u8]);
        }
        data.extend_from_slice(&self.key);
        data.extend_from_slice(&self.val);
        data
    }
    pub fn decode(file: &mut impl Read) -> Result<Self, VaultError> {
        let mut header = [0u8; 9];
        file.read_exact(&mut header)?;

        let kl = u32::from_le_bytes(header[0..4].try_into().unwrap()) as usize;
        let vl = u32::from_le_bytes(header[4..8].try_into().unwrap()) as usize;
        let deleted = header[8] != 0;

        let req_len = kl.saturating_add(vl);
        let mut payload = vec![0u8; req_len];
        file.read_exact(&mut payload)?;
        let key = payload[0..kl].to_vec();
        let val = payload[kl..kl + vl].to_vec();
        Ok(Entry::new(&key, &val, deleted))
    }
}

mod tests {
    use crate::entry::Entry;
    use crate::vault_error::VaultError;
    use std::io::ErrorKind;

    #[test]
    fn test_entry() {
        let key = vec![36, 64];
        let val = vec![64, 100];
        let ent = Entry::new(&key, &val, false);

        let data = ent.encode();
        assert_eq!(data.len(), 13, "Encoded data should be exactly 12 bytes");

        let mut valid_reader = data.as_slice();
        let decoded = Entry::decode(&mut valid_reader).unwrap();
        assert_eq!(decoded.key, key);
        assert_eq!(decoded.val, val);

        let short_header = vec![0, 1, 2, 3, 4];
        let mut short_reader = short_header.as_slice();
        let err = Entry::decode(&mut short_reader).unwrap_err();

        assert!(
            matches!(err,VaultError::Io(ref e) if e.kind() == ErrorKind::UnexpectedEof),
            "Expected UnexpectedEof IO error for short header, got {:?}",
            err
        );

        let mut corrupted_data = data;
        corrupted_data.pop();

        let mut corrupted_reader = corrupted_data.as_slice();
        let err = Entry::decode(&mut corrupted_reader).unwrap_err();

        assert!(
            matches!(err,VaultError::Io(ref e) if e.kind() == ErrorKind::UnexpectedEof),
            "Expected UnexpectedEof IO error for short header, got {:?}",
            err
        );
    }
}
