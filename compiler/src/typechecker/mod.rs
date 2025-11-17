use crate::ast::*;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    Float,
    String,
    Boolean,
    List(Box<Type>),
    Function(Vec<Type>, Box<Type>),
    Unknown,
}

pub struct TypeChecker {
    variables: HashMap<String, Type>,
    functions: HashMap<String, (Vec<Type>, Type)>,
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut checker = Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
        };
        checker.register_builtins();
        checker
    }

    fn register_builtins(&mut self) {
        self.functions.insert(
            "print".to_string(),
            (vec![Type::String], Type::Unknown),
        );
    }

    pub fn check(&mut self, program: &Program) -> Result<()> {
        for stmt in &program.statements {
            self.check_statement(stmt)?;
        }
        Ok(())
    }

    fn check_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::Let { name, value } => {
                let value_type = self.check_expression(value)?;
                self.variables.insert(name.clone(), value_type);
            }
            Statement::Function { name, params, body } => {
                let param_types: Vec<Type> = params.iter().map(|_| Type::Unknown).collect();
                let return_type = self.infer_return_type(body)?;
                self.functions.insert(
                    name.clone(),
                    (param_types, return_type),
                );
            }
            Statement::If { condition, then_body, else_body } => {
                let cond_type = self.check_expression(condition)?;
                if cond_type != Type::Boolean {
                    return Err(anyhow::anyhow!("If condition must be boolean"));
                }
                for stmt in then_body {
                    self.check_statement(stmt)?;
                }
                if let Some(else_body) = else_body {
                    for stmt in else_body {
                        self.check_statement(stmt)?;
                    }
                }
            }
            Statement::For { var, iterable, body } => {
                let iter_type = self.check_expression(iterable)?;
                self.variables.insert(var.clone(), Type::Number);
                for stmt in body {
                    self.check_statement(stmt)?;
                }
            }
            Statement::Print { args } => {
                for arg in args {
                    self.check_expression(arg)?;
                }
            }
            Statement::Return { value } => {
                if let Some(v) = value {
                    self.check_expression(v)?;
                }
            }
            Statement::Expression(expr) => {
                self.check_expression(expr)?;
            }
        }
        Ok(())
    }

    fn check_expression(&self, expr: &Expression) -> Result<Type> {
        match expr {
            Expression::Number(_) => Ok(Type::Number),
            Expression::Float(_) => Ok(Type::Float),
            Expression::String(_) => Ok(Type::String),
            Expression::Variable(name) => {
                self.variables
                    .get(name)
                    .cloned()
                    .ok_or_else(|| anyhow::anyhow!("Undefined variable: {}", name))
            }
            Expression::Binary { left, op, right } => {
                let left_type = self.check_expression(left)?;
                let right_type = self.check_expression(right)?;
                match op {
                    BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply | BinaryOp::Divide => {
                        if left_type == Type::Number && right_type == Type::Number {
                            Ok(Type::Number)
                        } else if left_type == Type::Float || right_type == Type::Float {
                            Ok(Type::Float)
                        } else {
                            Err(anyhow::anyhow!("Cannot perform arithmetic on non-numeric types"))
                        }
                    }
                    BinaryOp::Greater | BinaryOp::Less | BinaryOp::Equal => Ok(Type::Boolean),
                }
            }
            Expression::Unary { op: _, expr } => self.check_expression(expr),
            Expression::Call { name, args } => {
                let (param_types, return_type) = self
                    .functions
                    .get(name)
                    .ok_or_else(|| anyhow::anyhow!("Undefined function: {}", name))?
                    .clone();

                if args.len() != param_types.len() {
                    return Err(anyhow::anyhow!(
                        "Function {} expects {} arguments, got {}",
                        name,
                        param_types.len(),
                        args.len()
                    ));
                }

                for (arg, param_type) in args.iter().zip(param_types.iter()) {
                    let arg_type = self.check_expression(arg)?;
                    if arg_type != *param_type && *param_type != Type::Unknown {
                        return Err(anyhow::anyhow!("Type mismatch in function call"));
                    }
                }

                Ok(return_type)
            }
        }
    }

    fn infer_return_type(&self, body: &[Statement]) -> Result<Type> {
        for stmt in body {
            if let Statement::Return { value } = stmt {
                if let Some(v) = value {
                    return self.check_expression(v);
                }
            }
        }
        Ok(Type::Unknown)
    }
}

