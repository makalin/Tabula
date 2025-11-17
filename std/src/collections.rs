use tabula_runtime::Value;
use anyhow::Result;

pub fn len(list: &[Value]) -> Result<Value> {
    Ok(Value::Number(list.len() as i64))
}

pub fn push(list: &mut Vec<Value>, item: Value) -> Result<Value> {
    list.push(item);
    Ok(Value::None)
}

pub fn pop(list: &mut Vec<Value>) -> Result<Value> {
    list.pop().ok_or_else(|| anyhow::anyhow!("List is empty"))
}

pub fn get(list: &[Value], index: i64) -> Result<Value> {
    let idx = index as usize;
    if idx >= list.len() {
        return Err(anyhow::anyhow!("Index out of bounds"));
    }
    Ok(list[idx].clone())
}

pub fn set(list: &mut Vec<Value>, index: i64, value: Value) -> Result<Value> {
    let idx = index as usize;
    if idx >= list.len() {
        return Err(anyhow::anyhow!("Index out of bounds"));
    }
    list[idx] = value;
    Ok(Value::None)
}

