use std::fmt;

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn format(&self) -> String {
        self.statements
            .iter()
            .map(|s| s.format(0))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let {
        name: String,
        value: Expression,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },
    If {
        condition: Expression,
        then_body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    },
    For {
        var: String,
        iterable: Expression,
        body: Vec<Statement>,
    },
    Print {
        args: Vec<Expression>,
    },
    Return {
        value: Option<Expression>,
    },
    Expression(Expression),
}

impl Statement {
    pub fn format(&self, indent: usize) -> String {
        let tabs = "\t".repeat(indent);
        match self {
            Statement::Let { name, value } => {
                format!("{}let {}  {}", tabs, name, value.format())
            }
            Statement::Function { name, params, body } => {
                let params_str = params.join("  ");
                let body_str = body
                    .iter()
                    .map(|s| s.format(indent + 1))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("{}func {} {}\n{}", tabs, name, params_str, body_str)
            }
            Statement::If {
                condition,
                then_body,
                else_body,
            } => {
                let mut result = format!("{}if {}\n", tabs, condition.format());
                result.push_str(
                    &then_body
                        .iter()
                        .map(|s| s.format(indent + 1))
                        .collect::<Vec<_>>()
                        .join("\n"),
                );
                if let Some(else_body) = else_body {
                    result.push_str(&format!("\n{}else\n", tabs));
                    result.push_str(
                        &else_body
                            .iter()
                            .map(|s| s.format(indent + 1))
                            .collect::<Vec<_>>()
                            .join("\n"),
                    );
                }
                result
            }
            Statement::For { var, iterable, body } => {
                let mut result = format!("{}for {} in {}\n", tabs, var, iterable.format());
                result.push_str(
                    &body
                        .iter()
                        .map(|s| s.format(indent + 1))
                        .collect::<Vec<_>>()
                        .join("\n"),
                );
                result
            }
            Statement::Print { args } => {
                let args_str = args
                    .iter()
                    .map(|e| e.format())
                    .collect::<Vec<_>>()
                    .join("  ");
                format!("{}print {}", tabs, args_str)
            }
            Statement::Return { value } => {
                if let Some(v) = value {
                    format!("{}return {}", tabs, v.format())
                } else {
                    format!("{}return", tabs)
                }
            }
            Statement::Expression(expr) => format!("{}{}", tabs, expr.format()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Number(i64),
    Float(f64),
    String(String),
    Variable(String),
    Binary {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expression>,
    },
    Call {
        name: String,
        args: Vec<Expression>,
    },
}

impl Expression {
    pub fn format(&self) -> String {
        match self {
            Expression::Number(n) => n.to_string(),
            Expression::Float(f) => f.to_string(),
            Expression::String(s) => format!("\"{}\"", s),
            Expression::Variable(v) => v.clone(),
            Expression::Binary { left, op, right } => {
                format!("{} {} {}", left.format(), op.format(), right.format())
            }
            Expression::Unary { op, expr } => format!("{}{}", op.format(), expr.format()),
            Expression::Call { name, args } => {
                let args_str = args
                    .iter()
                    .map(|e| e.format())
                    .collect::<Vec<_>>()
                    .join("  ");
                format!("{} {}", name, args_str)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Greater,
    Less,
    Equal,
}

impl BinaryOp {
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOp::Add | BinaryOp::Subtract => 1,
            BinaryOp::Multiply | BinaryOp::Divide => 2,
            BinaryOp::Greater | BinaryOp::Less | BinaryOp::Equal => 0,
        }
    }

    pub fn format(&self) -> String {
        match self {
            BinaryOp::Add => "+".to_string(),
            BinaryOp::Subtract => "-".to_string(),
            BinaryOp::Multiply => "*".to_string(),
            BinaryOp::Divide => "/".to_string(),
            BinaryOp::Greater => ">".to_string(),
            BinaryOp::Less => "<".to_string(),
            BinaryOp::Equal => "==".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Negate,
}

impl UnaryOp {
    pub fn format(&self) -> String {
        match self {
            UnaryOp::Negate => "-".to_string(),
        }
    }
}

