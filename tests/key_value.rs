use RustedVault::key_value::{KeyValue, KeyValueError};

#[test]
fn test_key_value() {
    let mut kv = KeyValue::new();
    kv.open().unwrap();

    let key = vec![36, 64];
    let value = vec![64, 100];

    let inserted = kv.set(vec![36, 64], vec![64, 100]).unwrap();
    assert!(inserted,"Expected true on first insert");

    let inserted_again = kv.set(key.clone(), value.clone()).unwrap();
    assert!(!inserted_again, "Expected false on identical insert");

    kv.delete(vec![36, 64]).unwrap();
    let err = kv.get(key.clone()).unwrap_err();
    assert!(
        matches!(err, KeyValueError::KeyNotFound { .. }),
        "Expected KeyNotFound error, got {:?}", err
    );

    let val = kv.get(vec![36, 64]).unwrap_or(vec![100,100]);
    assert_eq!(val, vec![100,100]);

    kv.close().unwrap();
}