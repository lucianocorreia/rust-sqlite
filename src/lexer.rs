#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexError {
    Unexpected { position: usize, character: char },
    UnterminatedString { position: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Create,
    Table,
    Insert,
    Into,
    Values,
    Select,
    From,
    Int,
    Text,
    Identifier(String),
    Integer(i64),
    String(String),
    LeftParen,
    RightParen,
    Comma,
    Semicolon,
    Star,
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut characters = input.char_indices().peekable();
    let mut tokens = Vec::new();
    while let Some((position, character)) = characters.next() {
        match character {
            character if character.is_whitespace() => {}
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            ',' => tokens.push(Token::Comma),
            ';' => tokens.push(Token::Semicolon),
            '*' => tokens.push(Token::Star),
            '\'' => tokens.push(string(&mut characters, position)?),
            character if character.is_ascii_alphabetic() || character == '_' => {
                let mut word = character.to_string();
                while matches!(characters.peek(), Some((_, next)) if next.is_ascii_alphanumeric() || *next == '_')
                {
                    word.push(characters.next().unwrap().1);
                }
                tokens.push(keyword(word));
            }
            character if character.is_ascii_digit() => {
                let mut number = character.to_string();
                while matches!(characters.peek(), Some((_, next)) if next.is_ascii_digit()) {
                    number.push(characters.next().unwrap().1);
                }
                tokens.push(Token::Integer(number.parse().unwrap()));
            }
            character => {
                return Err(LexError::Unexpected {
                    position,
                    character,
                });
            }
        }
    }

    Ok(tokens)
}

fn string(
    characters: &mut std::iter::Peekable<std::str::CharIndices<'_>>,
    position: usize,
) -> Result<Token, LexError> {
    let mut value = String::new();
    for (_, character) in characters.by_ref() {
        if character == '\'' {
            return Ok(Token::String(value));
        }
        value.push(character);
    }
    Err(LexError::UnterminatedString { position })
}

fn keyword(word: String) -> Token {
    match word.to_ascii_uppercase().as_str() {
        "CREATE" => Token::Create,
        "TABLE" => Token::Table,
        "INSERT" => Token::Insert,
        "INTO" => Token::Into,
        "VALUES" => Token::Values,
        "SELECT" => Token::Select,
        "FROM" => Token::From,
        "INT" => Token::Int,
        "TEXT" => Token::Text,
        _ => Token::Identifier(word),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_data_is_comparable() {
        assert_eq!(
            Token::Identifier("users".into()),
            Token::Identifier("users".into())
        );
        assert_eq!(Token::Integer(32), Token::Integer(32));
    }

    #[test]
    fn lex_create_table_strings() {
        assert_eq!(
            lex("CREATE TABLE users (id INT, name TEXT);").unwrap(),
            vec![
                Token::Create,
                Token::Table,
                Token::Identifier("users".into()),
                Token::LeftParen,
                Token::Identifier("id".into()),
                Token::Int,
                Token::Comma,
                Token::Identifier("name".into()),
                Token::Text,
                Token::RightParen,
                Token::Semicolon
            ]
        );

        assert_eq!(
            lex("'Luciano Correial'").unwrap(),
            vec![Token::String("Luciano Correial".into())]
        );
    }

    #[test]
    fn rejects_invalid_input() {
        assert_eq!(
            lex("@").unwrap_err(),
            LexError::Unexpected {
                position: 0,
                character: '@'
            }
        );
        assert_eq!(
            lex("'open").unwrap_err(),
            LexError::UnterminatedString { position: 0 }
        );
    }
}
