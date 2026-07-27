use crate::value::Value;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Create {
        table_name: String,
        columns: Vec<String>,
    },
    Insert {
        table_name: String,
        values: Vec<Value>,
    },
    Select {
        table_name: String,
    },
}
