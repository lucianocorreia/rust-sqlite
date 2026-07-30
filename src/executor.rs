use crate::{
    database::{CreateTableError, Database},
    statement::{Projection, Statement},
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
    ColumnNotFound(String),
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
    statement: Statement,
) -> Result<ExecutionResult, ExecutionError> {
    match statement {
        Statement::CreateTable {
            table_name,
            columns,
        } => {
            let columns = columns
                .into_iter()
                .map(|column| Column::new(column.name))
                .collect();
            database.create_table(Table::new(table_name, columns))?;
            Ok(ExecutionResult::TableCreated)
        }
        Statement::Insert { table_name, values } => {
            database
                .table_mut(&table_name)
                .ok_or_else(|| ExecutionError::TableNotFound(table_name.clone()))?
                .insert(Row::new(values))?;
            Ok(ExecutionResult::RowInserted)
        }
        Statement::Select {
            table_name,
            projection,
        } => select_rows(
            database
                .table(&table_name)
                .ok_or_else(|| ExecutionError::TableNotFound(table_name.clone()))?,
            projection,
        ),
    }
}

fn select_rows(table: &Table, projection: Projection) -> Result<ExecutionResult, ExecutionError> {
    match projection {
        Projection::All => Ok(ExecutionResult::Rows(table.rows().to_vec())),
        Projection::Columns(names) => {
            let indices = names
                .into_iter()
                .map(|name| {
                    table
                        .column_index(&name)
                        .ok_or(ExecutionError::ColumnNotFound(name))
                })
                .collect::<Result<Vec<_>, _>>()?;

            let rows = table
                .rows()
                .iter()
                .map(|row| {
                    let values = indices
                        .iter()
                        .map(|index| row.values()[*index].clone())
                        .collect();
                    Row::new(values)
                })
                .collect();

            Ok(ExecutionResult::Rows(rows))
        }
    }
}
