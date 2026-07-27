use crate::value::Value;

#[derive(Debug, PartialEq, Eq)]
pub enum InsertError {
    WrongValueCount { expected: usize, actual: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Column {
    pub name: String,
}

impl Column {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub values: Vec<Value>,
}

impl Row {
    pub fn new(values: Vec<Value>) -> Self {
        Self { values }
    }

    pub fn values(&self) -> &[Value] {
        &self.values
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    name: String,
    columns: Vec<Column>,
    rows: Vec<Row>,
}

impl Table {
    pub fn new(name: impl Into<String>, columns: Vec<Column>) -> Self {
        Self {
            name: name.into(),
            columns,
            rows: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn columns(&self) -> &[Column] {
        &self.columns
    }

    pub fn rows(&self) -> &[Row] {
        &self.rows
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn column_index(&self, name: &str) -> Option<usize> {
        self.columns.iter().position(|col| col.name == name)
    }

    pub fn insert(&mut self, row: Row) -> Result<(), InsertError> {
        if row.values.len() != self.columns.len() {
            return Err(InsertError::WrongValueCount {
                expected: self.columns.len(),
                actual: row.values.len(),
            });
        }

        self.rows.push(row);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors_keep_owned_data_in_order() {
        let column = Column::new("name");
        let row = Row::new(vec![Value::Integer(1), Value::Text("Ana".into())]);

        assert_eq!(column.name, "name");
        assert_eq!(row.values()[0], Value::Integer(1));
    }

    #[test]
    fn finds_columns_or_none() {
        let table = Table::new("users", vec![Column::new("id")]);
        assert_eq!(table.column_index("id"), Some(0));
        assert_eq!(table.column_index("name"), None);
    }

    #[test]
    fn inserts_valid_rows_and_preserves_invalid_state() {
        let mut table = Table::new("users", vec![Column::new("id")]);
        table.insert(Row::new(vec![Value::Integer(1)])).unwrap();

        assert_eq!(table.row_count(), 1);
        assert_eq!(
            table.insert(Row::new(vec![])).unwrap_err(),
            InsertError::WrongValueCount {
                expected: 1,
                actual: 0
            }
        );

        assert_eq!(table.row_count(), 1);
    }
}
