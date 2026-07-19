use RustedVault::key_value::KeyValue;
use RustedVault::vault_error::{KeyValueError, VaultError};
use std::fs;

#[test]
fn test_key_value() {
    let test_file = "test_kv.log";
    let _ = fs::remove_file(test_file);

    let mut kv = KeyValue::new(test_file).unwrap();
    kv.open().unwrap();

    let key = vec![36, 64];
    let value = vec![64, 100];

    let inserted = kv.set(key.clone(), value.clone()).unwrap();
    assert!(inserted, "Expected true on first insert");

    let inserted_again = kv.set(key.clone(), value.clone()).unwrap();
    assert!(!inserted_again, "Expected false on identical insert");

    kv.close().unwrap();

    let mut kv2 = KeyValue::new(test_file).unwrap();
    kv2.open().unwrap();

    let val = kv2.get(key.clone()).unwrap();
    assert_eq!(val, value);

    kv.delete(key.clone()).unwrap();

    let err = kv.get(key.clone()).unwrap_err();
    assert!(
        matches!(
            err,
            VaultError::KeyValueError(KeyValueError::KeyNotFound { .. })
        ),
        "Expected KeyNotFound error, got {:?}",
        err
    );

    kv2.close().unwrap();

    let mut kv3 = KeyValue::new(test_file).unwrap();
    kv3.open().unwrap();

    let err2 = kv3.get(key.clone()).unwrap_err();
    assert!(
        matches!(
            err2,
            VaultError::KeyValueError(KeyValueError::KeyNotFound { .. })
        ),
        "Expected KeyNotFound error after reopening deleted key"
    );

    kv3.close().unwrap();

    let _ = fs::remove_file(test_file);
}
