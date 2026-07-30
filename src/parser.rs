use crate::{
    lexer::{Token, lex},
    statement::{ColumnDefinition, ColumnType, Statement},
    value::Value,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    End(&'static str),
    Token {
        expected: &'static str,
        found: Token,
    },
    Trailing(Token),
}

pub struct Cursor {
    tokens: Vec<Token>,
    position: usize,
}

impl Cursor {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    pub fn next(&mut self) -> Option<Token> {
        let value = self.peek().cloned();
        self.position += usize::from(value.is_some());
        value
    }

    pub fn identifier(&mut self) -> Result<String, ParseError> {
        match self.next() {
            Some(Token::Identifier(value)) => Ok(value),
            Some(token) => Err(ParseError::Token {
                expected: "identifier",
                found: token,
            }),
            None => Err(ParseError::End("identifier")),
        }
    }

    pub fn expect(&mut self, expected: &'static str, token: Token) -> Result<(), ParseError> {
        match self.next() {
            Some(found) if found == token => Ok(()),
            Some(found) => Err(ParseError::Token { expected, found }),
            None => Err(ParseError::End(expected)),
        }
    }
}

pub fn parse(input: &str) -> Result<Statement, ParseError> {
    let mut cursor = Cursor::new(lex(input).map_err(|_| ParseError::End("valid SQL"))?);
    let statement = match cursor.next() {
        Some(Token::Create) => parse_create_table(&mut cursor)?,
        Some(Token::Insert) => parse_insert(&mut cursor)?,
        Some(Token::Select) => parse_select(&mut cursor)?,
        Some(found) => {
            return Err(ParseError::Token {
                expected: "statement",
                found,
            });
        }
        None => return Err(ParseError::End("statement")),
    };

    if let Some(found) = cursor.next() {
        return Err(ParseError::Trailing(found));
    };

    Ok(statement)
}

fn parse_insert(cursor: &mut Cursor) -> Result<Statement, ParseError> {
    cursor.expect("INTO", Token::Into)?;
    let table_name = cursor.identifier()?;
    cursor.expect("VALUES", Token::Values)?;
    cursor.expect("(", Token::LeftParen)?;
    let mut values = vec![literal(cursor)?];
    while matches!(cursor.peek(), Some(Token::Comma)) {
        cursor.next();
        values.push(literal(cursor)?);
    }
    cursor.expect("')'", Token::RightParen)?;
    cursor.expect("';'", Token::Semicolon)?;
    Ok(Statement::Insert { table_name, values })
}

fn literal(cursor: &mut Cursor) -> Result<Value, ParseError> {
    match cursor.next() {
        Some(Token::Integer(value)) => Ok(Value::Integer(value)),
        Some(Token::String(value)) => Ok(Value::Text(value)),
        Some(found) => Err(ParseError::Token {
            expected: "literal",
            found,
        }),
        None => Err(ParseError::End("literal")),
    }
}

fn parse_select(cursor: &mut Cursor) -> Result<Statement, ParseError> {
    let projection = projection(cursor)?;
    cursor.expect("FROM", Token::From)?;
    let table_name = cursor.identifier()?;
    cursor.expect("';'", Token::Semicolon)?;
    Ok(Statement::Select {
        table_name,
        projection,
    })
}

fn projection(cursor: &mut Cursor) -> Result<crate::statement::Projection, ParseError> {
    if matches!(cursor.peek(), Some(Token::Star)) {
        cursor.next();
        return Ok(crate::statement::Projection::All);
    }
    let mut names = vec![cursor.identifier()?];
    while matches!(cursor.peek(), Some(Token::Comma)) {
        cursor.next();
        names.push(cursor.identifier()?);
    }
    Ok(crate::statement::Projection::Columns(names))
}

fn parse_create_table(cursor: &mut Cursor) -> Result<Statement, ParseError> {
    cursor.expect("TABLE", Token::Table)?;
    let table_name = cursor.identifier()?;
    cursor.expect("(", Token::LeftParen)?;
    let mut columns = Vec::new();
    loop {
        let name = cursor.identifier()?;
        let column_type = match cursor.next() {
            Some(Token::Int) => ColumnType::Integer,
            Some(Token::Text) => ColumnType::Text,
            Some(found) => {
                return Err(ParseError::Token {
                    expected: "INT or TEXT",
                    found,
                });
            }
            None => return Err(ParseError::End("column type")),
        };
        columns.push(ColumnDefinition { name, column_type });
        if !matches!(cursor.peek(), Some(Token::Comma)) {
            break;
        }
        cursor.next();
    }
    cursor.expect(")", Token::RightParen)?;
    cursor.expect(";", Token::Semicolon)?;
    Ok(Statement::CreateTable {
        table_name,
        columns,
    })
}
