use tabula_runtime::Value;
use anyhow::Result;

pub fn abs(n: i64) -> Result<Value> {
    Ok(Value::Number(n.abs()))
}

pub fn max(a: i64, b: i64) -> Result<Value> {
    Ok(Value::Number(a.max(b)))
}

pub fn min(a: i64, b: i64) -> Result<Value> {
    Ok(Value::Number(a.min(b)))
}

pub fn sqrt(n: f64) -> Result<Value> {
    Ok(Value::Float(n.sqrt()))
}

pub fn pow(base: f64, exp: f64) -> Result<Value> {
    Ok(Value::Float(base.powf(exp)))
}

pub fn sin(n: f64) -> Result<Value> {
    Ok(Value::Float(n.sin()))
}

pub fn cos(n: f64) -> Result<Value> {
    Ok(Value::Float(n.cos()))
}

