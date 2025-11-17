use tabula_runtime::Value;
use anyhow::Result;

pub fn print(args: Vec<Value>) -> Result<Value> {
    let output: Vec<String> = args.iter().map(|v| v.to_string()).collect();
    println!("{}", output.join(" "));
    Ok(Value::None)
}

pub fn read_line() -> Result<Value> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(Value::String(input.trim().to_string()))
}

pub fn read_file(path: &str) -> Result<Value> {
    let contents = std::fs::read_to_string(path)?;
    Ok(Value::String(contents))
}

pub fn write_file(path: &str, contents: &str) -> Result<Value> {
    std::fs::write(path, contents)?;
    Ok(Value::None)
}

