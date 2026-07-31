use std::path::PathBuf;

use dblite::{
    database::Database,
    executor::{ExecutionResult, execute},
    parser::parse,
    statement::Statement,
    storage::{deserialize_database, save_atomically, serialize_database},
    value::Value,
};

#[test]
fn storage_roundtrip_keeps_data() {
    let mut database = Database::new();

    for sql in [
        "CREATE TABLE users (id INT, name TEXT);",
        "INSERT INTO users VALUES (1, 'Luciano');",
        "INSERT INTO users VALUES (2, 'Ana');",
    ] {
        let statement: Statement = parse(sql).unwrap();
        execute(&mut database, statement).unwrap();
    }

    let bytes = serialize_database(&database).unwrap();
    let restored = deserialize_database(&bytes).unwrap();

    let mut restored = restored;
    let result = execute(&mut restored, parse("SELECT name, id FROM users;").unwrap()).unwrap();
    let rows = match result {
        ExecutionResult::Rows(rows) => rows,
        other => panic!("expected rows, got {other:?}"),
    };

    assert_eq!(rows.len(), 2);
    assert_eq!(
        rows[0].values(),
        &[Value::Text("Luciano".into()), Value::Integer(1)]
    );
    assert_eq!(
        rows[1].values(),
        &[Value::Text("Ana".into()), Value::Integer(2)]
    );
}

#[test]
fn atomic_save_writes_file() {
    let path = PathBuf::from("target/test-data/atomic-save.dbl");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();

    save_atomically(&path, b"hello").unwrap();
    assert_eq!(std::fs::read(&path).unwrap(), b"hello");
}
