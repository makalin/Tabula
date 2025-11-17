use tabula_runtime::Value;
use anyhow::Result;

pub fn len(s: &str) -> Result<Value> {
    Ok(Value::Number(s.len() as i64))
}

pub fn concat(s1: &str, s2: &str) -> Result<Value> {
    Ok(Value::String(format!("{}{}", s1, s2)))
}

pub fn split(s: &str, delimiter: &str) -> Result<Value> {
    let parts: Vec<Value> = s
        .split(delimiter)
        .map(|p| Value::String(p.to_string()))
        .collect();
    Ok(Value::List(parts))
}

pub fn trim(s: &str) -> Result<Value> {
    Ok(Value::String(s.trim().to_string()))
}

pub fn upper(s: &str) -> Result<Value> {
    Ok(Value::String(s.to_uppercase()))
}

pub fn lower(s: &str) -> Result<Value> {
    Ok(Value::String(s.to_lowercase()))
}

