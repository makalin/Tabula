use crate::value::Value;
use anyhow::Result;
use std::collections::HashMap;

pub struct VM {
    stack: Vec<Value>,
    variables: HashMap<String, Value>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            variables: HashMap::new(),
        }
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Result<Value> {
        self.stack
            .pop()
            .ok_or_else(|| anyhow::anyhow!("Stack underflow"))
    }

    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    pub fn clear(&mut self) {
        self.stack.clear();
        self.variables.clear();
    }
}

