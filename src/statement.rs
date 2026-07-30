use crate::value::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColumnType {
    Integer,
    Text,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnDefinition {
    pub name: String,
    pub column_type: ColumnType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    CreateTable {
        table_name: String,
        columns: Vec<ColumnDefinition>,
    },
    Insert {
        table_name: String,
        values: Vec<Value>,
    },
    Select {
        projection: Projection,
        table_name: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Projection {
    All,
    Columns(Vec<String>),
}
