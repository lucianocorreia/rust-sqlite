// Formato v1 (little-endian):
// [magic:4] [version:u16] [tables:u32]
// para cada tabela:
//   [table_name_len:u32] [table_name:bytes]
//   [columns:u32]
//   para cada coluna: [column_name_len:u32] [column_name:bytes]
//   [rows:u32]
//   para cada row:
//     [values:u32]
//     para cada value:
//       [tag:u8] + payload
//       tag 0 => Integer(i64)
//       tag 1 => Text([len:u32] [utf8 bytes])

use crate::value::Value;
use crate::{
    database::Database,
    table::{Column, Row, Table},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageError {
    UnexpectedEof,
    InvalidTag(u8),
    InvalidUtf8,
    LengthOverflow,
    Io(String),
}

pub trait Storage {
    fn load(&self) -> Result<Database, StorageError>;
    fn save(&self, database: &Database) -> Result<(), StorageError>;
}

pub struct MemoryStorage {
    data: Vec<u8>,
}

pub struct FileStorage {
    path: std::path::PathBuf,
}

/// Writes a `Value` to the given output buffer in a binary format.
pub fn write_value(output: &mut Vec<u8>, value: &Value) {
    match value {
        Value::Integer(number) => {
            output.push(0);
            output.extend(number.to_le_bytes());
        }
        Value::Text(text) => {
            output.push(1);
            output.extend((text.len() as u32).to_le_bytes());
            output.extend(text.as_bytes());
        }
    }
}

pub fn read_value(input: &[u8], cursor: &mut usize) -> Result<Value, StorageError> {
    let tag = read_u8(input, cursor)?;
    match tag {
        0 => {
            let bytes = read_exact(input, cursor, 8)?;
            let mut buffer = [0u8; 8];
            buffer.copy_from_slice(bytes);
            Ok(Value::Integer(i64::from_le_bytes(buffer)))
        }
        1 => {
            let length = read_u32(input, cursor)? as usize;
            let bytes = read_exact(input, cursor, length)?;
            let text = std::str::from_utf8(bytes).map_err(|_| StorageError::InvalidUtf8)?;
            Ok(Value::Text(text.to_owned()))
        }
        other => Err(StorageError::InvalidTag(other)),
    }
}

pub fn serialize_database(database: &Database) -> Result<Vec<u8>, StorageError> {
    let mut out = Vec::new();
    out.extend(*b"DBL1");
    out.extend((1u16).to_le_bytes());

    let tables = database.tables().collect::<Vec<_>>();
    out.extend((tables.len() as u32).to_le_bytes());
    for table in tables {
        write_string(&mut out, table.name())?;

        out.extend((table.columns().len() as u32).to_le_bytes());
        for column in table.columns() {
            write_string(&mut out, &column.name)?;
        }

        out.extend((table.rows().len() as u32).to_le_bytes());
        for row in table.rows() {
            out.extend((row.values().len() as u32).to_le_bytes());
            for value in row.values() {
                write_value(&mut out, value);
            }
        }
    }
    Ok(out)
}

pub fn deserialize_database(bytes: &[u8]) -> Result<Database, StorageError> {
    let mut cursor = 0;

    let magic = read_exact(bytes, &mut cursor, 4)?;
    if magic != b"DBL1" {
        return Err(StorageError::Io("invalid magic".into()));
    }

    let _version = read_u16(bytes, &mut cursor)?;
    let table_count = read_u32(bytes, &mut cursor)? as usize;

    let mut database = Database::new();
    for _ in 0..table_count {
        let table_name = read_string(bytes, &mut cursor)?;

        let column_count = read_u32(bytes, &mut cursor)? as usize;
        let mut columns = Vec::with_capacity(column_count);
        for _ in 0..column_count {
            columns.push(Column::new(read_string(bytes, &mut cursor)?));
        }

        let mut table = Table::new(table_name, columns);

        let row_count = read_u32(bytes, &mut cursor)? as usize;
        for _ in 0..row_count {
            let value_count = read_u32(bytes, &mut cursor)? as usize;
            let mut values = Vec::with_capacity(value_count);
            for _ in 0..value_count {
                values.push(read_value(bytes, &mut cursor)?);
            }
            table
                .insert(Row::new(values))
                .map_err(|e| StorageError::Io(format!("{e:?}")))?;
        }

        database
            .create_table(table)
            .map_err(|e| StorageError::Io(format!("{e:?}")))?;
    }

    Ok(database)
}

pub fn save_atomically(path: &std::path::Path, bytes: &[u8]) -> std::io::Result<()> {
    let temporary = path.with_extension("tmp");
    {
        let mut writer = std::io::BufWriter::new(std::fs::File::create(&temporary)?);
        use std::io::Write;
        writer.write_all(bytes)?;
        writer.flush()?;
        writer.get_ref().sync_all()?;
    }
    std::fs::rename(temporary, path)
}

fn write_string(output: &mut Vec<u8>, value: &str) -> Result<(), StorageError> {
    output.extend((value.len() as u32).to_le_bytes());
    output.extend(value.as_bytes());
    Ok(())
}

fn read_u16(input: &[u8], cursor: &mut usize) -> Result<u16, StorageError> {
    let bytes = read_exact(input, cursor, 2)?;
    let mut buffer = [0u8; 2];
    buffer.copy_from_slice(bytes);
    Ok(u16::from_le_bytes(buffer))
}

fn read_string(input: &[u8], cursor: &mut usize) -> Result<String, StorageError> {
    let length = read_u32(input, cursor)? as usize;
    let bytes = read_exact(input, cursor, length)?;
    let text = std::str::from_utf8(bytes).map_err(|_| StorageError::InvalidUtf8)?;
    Ok(text.to_owned())
}

fn read_u8(input: &[u8], cursor: &mut usize) -> Result<u8, StorageError> {
    let bytes = read_exact(input, cursor, 1)?;
    Ok(bytes[0])
}

fn read_u32(input: &[u8], cursor: &mut usize) -> Result<u32, StorageError> {
    let bytes = read_exact(input, cursor, 4)?;
    let mut buffer = [0u8; 4];
    buffer.copy_from_slice(bytes);
    Ok(u32::from_le_bytes(buffer))
}

fn read_exact<'a>(
    input: &'a [u8],
    cursor: &mut usize,
    size: usize,
) -> Result<&'a [u8], StorageError> {
    let end = cursor
        .checked_add(size)
        .ok_or(StorageError::LengthOverflow)?;

    if end > input.len() {
        return Err(StorageError::UnexpectedEof);
    }
    let chunk = &input[*cursor..end];
    *cursor = end;
    Ok(chunk)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_round_trip_integer_and_text() {
        let mut bytes = Vec::new();
        write_value(&mut bytes, &Value::Integer(42));
        write_value(&mut bytes, &Value::Text("Ana".into()));

        let mut cursor = 0;
        assert_eq!(read_value(&bytes, &mut cursor).unwrap(), Value::Integer(42));
        assert_eq!(
            read_value(&bytes, &mut cursor).unwrap(),
            Value::Text("Ana".into())
        );
        assert_eq!(cursor, bytes.len());
    }

    #[test]
    fn value_rejects_invalid_tag() {
        let input = vec![9u8];
        assert_eq!(
            read_value(&input, &mut 0usize).unwrap_err(),
            StorageError::InvalidTag(9)
        );
    }

    #[test]
    fn value_rejects_truncated_integer() {
        let input = vec![0u8, 1, 2, 3];
        assert_eq!(
            read_value(&input, &mut 0usize).unwrap_err(),
            StorageError::UnexpectedEof
        );
    }

    #[test]
    fn value_rejects_invalid_utf8() {
        let input = vec![
            1u8, // tag text
            2, 0, 0, 0, // len
            0xFF, 0xFF,
        ];
        assert_eq!(
            read_value(&input, &mut 0usize).unwrap_err(),
            StorageError::InvalidUtf8
        );
    }
}
