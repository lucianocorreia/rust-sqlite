use std::io::{self, BufRead, Write};

use dblite::{
    database::Database,
    executor::{ExecutionResult, execute},
    parser::parse,
};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut database = Database::new();
    let mut input = String::new();

    loop {
        print!("dblite> ");
        io::stdout().flush()?;
        input.clear();
        if stdin.lock().read_line(&mut input)? == 0 {
            break;
        }
        let input = input.trim();
        if input == ".exit" {
            break;
        }
        match parse(input) {
            Ok(command) => match execute(&mut database, command) {
                Ok(ExecutionResult::TableCreated) => println!("table created"),
                Ok(ExecutionResult::RowInserted) => println!("row inserted"),
                Ok(ExecutionResult::Rows(rows)) => println!("{} row(s)", rows.len()),
                Err(error) => println!("error: {error:?}"),
            },
            Err(error) => println!("error: {error:?}"),
        }
    }
    Ok(())
}
