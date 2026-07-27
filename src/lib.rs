pub mod command;
pub mod database;
pub mod executor;
pub mod parser;
pub mod table;
pub mod value;

pub fn greeting() -> &'static str {
    "dblite"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeting() {
        assert_eq!(greeting(), "dblite");
    }
}
