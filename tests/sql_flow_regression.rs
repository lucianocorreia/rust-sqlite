use dblite::{
    database::Database,
    executor::{ExecutionError, ExecutionResult, execute},
    parser::parse,
    value::Value,
};

#[test]
fn sql_flow_regression() {
    let mut database = Database::new();

    assert_eq!(
        execute(
            &mut database,
            parse("CREATE TABLE users (id INT, name TEXT)").unwrap()
        )
        .unwrap(),
        ExecutionResult::TableCreated
    );

    assert_eq!(
        execute(
            &mut database,
            parse("INSERT INTO users VALUES (1, 'Luciano');").unwrap()
        )
        .unwrap(),
        ExecutionResult::RowInserted
    );

    assert_eq!(
        execute(
            &mut database,
            parse("INSERT INTO users VALUES (2, 'Ana');").unwrap()
        )
        .unwrap(),
        ExecutionResult::RowInserted
    );

    assert_eq!(
        execute(
            &mut database,
            parse("INSERT INTO users VALUES (3);").unwrap()
        )
        .unwrap_err(),
        ExecutionError::WrongValueCount {
            expected: 2,
            actual: 1
        }
    );

    let rows = match execute(&mut database, parse("SELECT name, id FROM users;").unwrap()).unwrap()
    {
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
