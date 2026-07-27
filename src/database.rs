use std::collections::{HashMap, hash_map::Entry};

use crate::table::Table;

#[derive(Debug, PartialEq, Eq)]
pub enum CreateTableError {
    AlreadyExists(String),
}

#[derive(Debug, Default)]
pub struct Database {
    tables: HashMap<String, Table>,
}

impl Database {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_table(&mut self, table: Table) -> Result<(), CreateTableError> {
        let name = table.name().to_owned();
        match self.tables.entry(name.clone()) {
            Entry::Vacant(slot) => {
                slot.insert(table);
                Ok(())
            }
            Entry::Occupied(_) => Err(CreateTableError::AlreadyExists(name)),
        }
    }

    pub fn table(&self, name: &str) -> Option<&Table> {
        self.tables.get(name)
    }

    pub fn table_mut(&mut self, name: &str) -> Option<&mut Table> {
        self.tables.get_mut(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::table::Column;

    #[test]
    fn stores_and_rejects_duplicate_tables() {
        let mut database = Database::new();
        database
            .create_table(Table::new("users", vec![Column::new("id")]))
            .unwrap();
        assert!(database.table("users").is_some());
        assert_eq!(
            database
                .create_table(Table::new("users", vec![]))
                .unwrap_err(),
            CreateTableError::AlreadyExists("users".into())
        );
    }
}
