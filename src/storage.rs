use crate::value::Value;

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
