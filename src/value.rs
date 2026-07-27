#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Integer(i64),
    Text(String),
}

impl Value {
    pub fn as_display(&self) -> String {
        match self {
            Value::Integer(i) => i.to_string(),
            Value::Text(s) => s.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_value_can_be_compared() {
        assert_eq!(Value::Integer(42), Value::Integer(42))
    }

    #[test]
    fn text_value_can_be_compared() {
        assert_eq!(
            Value::Text("hello".to_string()),
            Value::Text("hello".to_string())
        )
    }

    #[test]
    fn display_integer_values() {
        assert_eq!(Value::Integer(42).as_display(), "42");
    }

    #[test]
    fn display_text_values() {
        assert_eq!(Value::Text("hello".to_string()).as_display(), "hello");
    }
}
