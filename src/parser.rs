use crate::{command::Command, value::Value};

#[derive(Debug, PartialEq, Eq)]
pub enum ParserError {
    Empty,
    Unknown(String),
    Missing(&'static str),
    InvalidInteger(String),
    Extra,
}

pub fn parse(input: &str) -> Result<Command, ParserError> {
    let mut tokens = input.split_whitespace();
    match tokens.next().ok_or(ParserError::Empty)? {
        "create" => {
            let table_name = tokens.next().ok_or(ParserError::Missing("table"))?.into();
            let columns: Vec<String> = tokens.map(str::to_owned).collect();
            if columns.is_empty() {
                return Err(ParserError::Missing("columns"));
            }
            Ok(Command::Create {
                table_name,
                columns,
            })
        }
        "insert" => {
            let table_name = tokens.next().ok_or(ParserError::Missing("table"))?.into();
            let id = integer(tokens.next())?;
            let name = tokens.next().ok_or(ParserError::Missing("name"))?.into();
            let age = integer(tokens.next())?;
            if tokens.next().is_some() {
                return Err(ParserError::Extra);
            }
            Ok(Command::Insert {
                table_name,
                values: vec![Value::Integer(id), Value::Text(name), Value::Integer(age)],
            })
        }
        "select" => {
            let table_name = tokens.next().ok_or(ParserError::Missing("table"))?.into();
            if tokens.next().is_some() {
                return Err(ParserError::Extra);
            }
            Ok(Command::Select { table_name })
        }
        command => Err(ParserError::Unknown(command.into())),
    }
}

fn integer(token: Option<&str>) -> Result<i64, ParserError> {
    let token = token.ok_or(ParserError::Missing("integer"))?;
    token
        .parse()
        .map_err(|_| ParserError::InvalidInteger(token.into()))
}
