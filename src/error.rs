use crate::{executor::ExecutionError, lexer::LexError, parser::ParseError, storage::StorageError};

#[derive(Debug)]
pub enum DbError {
    Lex(LexError),
    Parse(ParseError),
    Execution(ExecutionError),
    Storage(StorageError),
}

impl std::fmt::Display for DbError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for DbError {}

impl From<LexError> for DbError {
    fn from(value: LexError) -> Self {
        Self::Lex(value)
    }
}

impl From<ParseError> for DbError {
    fn from(value: ParseError) -> Self {
        Self::Parse(value)
    }
}

impl From<ExecutionError> for DbError {
    fn from(value: ExecutionError) -> Self {
        Self::Execution(value)
    }
}

impl From<StorageError> for DbError {
    fn from(value: StorageError) -> Self {
        Self::Storage(value)
    }
}
