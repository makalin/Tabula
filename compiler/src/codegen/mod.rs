use crate::ast::*;
use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

pub struct Codegen {
    // LLVM context and module would go here
}

impl Codegen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_native(&self, program: &Program, output: &Path) -> Result<()> {
        // TODO: Implement LLVM code generation
        // For now, generate a simple C representation
        let mut code = String::from("#include <stdio.h>\n#include <stdlib.h>\n\n");
        code.push_str("int main() {\n");
        
        for stmt in &program.statements {
            code.push_str(&self.generate_statement_c(stmt, 1)?);
        }
        
        code.push_str("  return 0;\n");
        code.push_str("}\n");
        
        std::fs::write(output.with_extension("c"), code)?;
        
        // In a real implementation, we would:
        // 1. Create LLVM module
        // 2. Generate LLVM IR
        // 3. Compile to native binary
        
        Ok(())
    }

    fn generate_statement_c(&self, stmt: &Statement, indent: usize) -> Result<String> {
        let tabs = "  ".repeat(indent);
        match stmt {
            Statement::Let { name, value } => {
                Ok(format!("{}int {} = {};\n", tabs, name, self.generate_expr_c(value)?))
            }
            Statement::Print { args } => {
                let args_str = args
                    .iter()
                    .map(|e| self.generate_expr_c(e))
                    .collect::<Result<Vec<_>>>()?
                    .join(", ");
                Ok(format!("{}printf(\"%d\\n\", {});\n", tabs, args_str))
            }
            Statement::Function { name, params, body } => {
                let params_str = params.join(", ");
                let mut func = format!("{}int {}({}) {{\n", tabs, name, params_str);
                for stmt in body {
                    func.push_str(&self.generate_statement_c(stmt, indent + 1)?);
                }
                func.push_str(&format!("{}}}\n", tabs));
                Ok(func)
            }
            _ => Ok(format!("{}// TODO: {}\n", tabs, format!("{:?}", stmt))),
        }
    }

    fn generate_expr_c(&self, expr: &Expression) -> Result<String> {
        match expr {
            Expression::Number(n) => Ok(n.to_string()),
            Expression::Variable(v) => Ok(v.clone()),
            Expression::Binary { left, op, right } => {
                Ok(format!(
                    "({} {} {})",
                    self.generate_expr_c(left)?,
                    op.format(),
                    self.generate_expr_c(right)?
                ))
            }
            _ => Ok("0".to_string()),
        }
    }
}

pub struct Interpreter {
    variables: HashMap<String, i64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, program: &Program) -> Result<()> {
        for stmt in &program.statements {
            self.execute_statement(stmt)?;
        }
        Ok(())
    }

    fn execute_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::Let { name, value } => {
                let val = self.evaluate_expression(value)?;
                self.variables.insert(name.clone(), val);
            }
            Statement::Print { args } => {
                let values: Vec<String> = args
                    .iter()
                    .map(|e| {
                        self.evaluate_expression(e)
                            .map(|v| v.to_string())
                            .unwrap_or_else(|_| "?".to_string())
                    })
                    .collect();
                println!("{}", values.join(" "));
            }
            Statement::Function { .. } => {
                // Function definitions are stored for later
            }
            Statement::If {
                condition,
                then_body,
                else_body,
            } => {
                let cond_val = self.evaluate_expression(condition)?;
                if cond_val != 0 {
                    for stmt in then_body {
                        self.execute_statement(stmt)?;
                    }
                } else if let Some(else_body) = else_body {
                    for stmt in else_body {
                        self.execute_statement(stmt)?;
                    }
                }
            }
            Statement::For { var, iterable, body } => {
                // Simplified: assume iterable is a number range
                let count = self.evaluate_expression(iterable)?;
                for i in 0..count {
                    self.variables.insert(var.clone(), i);
                    for stmt in body {
                        self.execute_statement(stmt)?;
                    }
                }
            }
            Statement::Return { .. } => {
                // Return handling
            }
            Statement::Expression(expr) => {
                self.evaluate_expression(expr)?;
            }
        }
        Ok(())
    }

    fn evaluate_expression(&self, expr: &Expression) -> Result<i64> {
        match expr {
            Expression::Number(n) => Ok(*n),
            Expression::Variable(v) => {
                self.variables
                    .get(v)
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Undefined variable: {}", v))
            }
            Expression::Binary { left, op, right } => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                Ok(match op {
                    BinaryOp::Add => left_val + right_val,
                    BinaryOp::Subtract => left_val - right_val,
                    BinaryOp::Multiply => left_val * right_val,
                    BinaryOp::Divide => {
                        if right_val == 0 {
                            return Err(anyhow::anyhow!("Division by zero"));
                        }
                        left_val / right_val
                    }
                    BinaryOp::Greater => (left_val > right_val) as i64,
                    BinaryOp::Less => (left_val < right_val) as i64,
                    BinaryOp::Equal => (left_val == right_val) as i64,
                })
            }
            Expression::Unary { op, expr } => {
                let val = self.evaluate_expression(expr)?;
                Ok(match op {
                    UnaryOp::Negate => -val,
                })
            }
            Expression::Call { name, args } => {
                // Built-in functions
                match name.as_str() {
                    "print" => {
                        for arg in args {
                            let val = self.evaluate_expression(arg)?;
                            print!("{} ", val);
                        }
                        println!();
                        Ok(0)
                    }
                    _ => Err(anyhow::anyhow!("Unknown function: {}", name)),
                }
            }
            Expression::String(_) | Expression::Float(_) => {
                Err(anyhow::anyhow!("Unsupported expression type in interpreter"))
            }
        }
    }
}

