use crate::{
    command::Command,
    database::{CreateTableError, Database},
    table::{Column, InsertError, Row, Table},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionResult {
    TableCreated,
    RowInserted,
    Rows(Vec<Row>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExecutionError {
    TableNotFound(String),
    TableAlreadyExists(String),
    WrongValueCount { expected: usize, actual: usize },
}

impl From<CreateTableError> for ExecutionError {
    fn from(value: CreateTableError) -> Self {
        match value {
            CreateTableError::AlreadyExists(name) => Self::TableAlreadyExists(name),
        }
    }
}

impl From<InsertError> for ExecutionError {
    fn from(value: InsertError) -> Self {
        match value {
            InsertError::WrongValueCount { expected, actual } => {
                Self::WrongValueCount { expected, actual }
            }
        }
    }
}

pub fn execute(
    database: &mut Database,
    command: Command,
) -> Result<ExecutionResult, ExecutionError> {
    match command {
        Command::Create {
            table_name,
            columns,
        } => {
            let columns = columns.into_iter().map(Column::new).collect();
            database.create_table(Table::new(table_name, columns))?;
            Ok(ExecutionResult::TableCreated)
        }
        Command::Insert { table_name, values } => {
            database
                .table_mut(&table_name)
                .ok_or(ExecutionError::TableNotFound(table_name))?
                .insert(Row::new(values))?;
            Ok(ExecutionResult::RowInserted)
        }
        Command::Select { table_name } => {
            let rows = database
                .table(&table_name)
                .ok_or(ExecutionError::TableNotFound(table_name))?
                .rows()
                .to_vec();
            Ok(ExecutionResult::Rows(rows))
        }
    }
}
