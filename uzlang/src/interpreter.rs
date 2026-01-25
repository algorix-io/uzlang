use crate::parser::{Expr, Stmt};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i64),
    String(String),
    Bool(bool),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
        }
    }
}

pub struct Interpreter {
    variables: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: &str, val: Value) {
        self.variables.insert(name.to_string(), val);
    }

    pub fn execute(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.execute_stmt(stmt);
        }
    }

    fn execute_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Print(expr) => {
                let val = self.evaluate(expr);
                println!("{}", val);
            }
            Stmt::If(cond, body) => {
                let val = self.evaluate(cond);
                if self.is_truthy(val) {
                    self.execute(body);
                }
            }
            Stmt::Loop(cond, body) => {
                while {
                    let val = self.evaluate(cond);
                    self.is_truthy(val)
                } {
                    self.execute(body);
                }
            }
            Stmt::Assign(name, expr) => {
                let val = self.evaluate(expr);
                self.set_variable(name, val);
            }
        }
    }

    fn evaluate(&self, expr: &Expr) -> Value {
        match expr {
            Expr::Number(n) => Value::Number(*n),
            Expr::StringLiteral(s) => Value::String(s.clone()),
            Expr::Identifier(name) => self
                .variables
                .get(name)
                .cloned()
                .unwrap_or(Value::Number(0)),
            Expr::BinaryOp(left, op, right) => {
                let l = self.evaluate(left);
                let r = self.evaluate(right);
                self.evaluate_binary(l, op, r)
            }
        }
    }

    fn evaluate_binary(&self, left: Value, op: &str, right: Value) -> Value {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => match op {
                "+" => Value::Number(l + r),
                "-" => Value::Number(l - r),
                "*" => Value::Number(l * r),
                "/" => Value::Number(l / r),
                "==" => Value::Bool(l == r),
                "!=" => Value::Bool(l != r),
                ">" => Value::Bool(l > r),
                "<" => Value::Bool(l < r),
                ">=" => Value::Bool(l >= r),
                "<=" => Value::Bool(l <= r),
                _ => Value::Bool(false),
            },
            (Value::String(l), Value::String(r)) => match op {
                "+" => Value::String(format!("{}{}", l, r)),
                "==" => Value::Bool(l == r),
                "!=" => Value::Bool(l != r),
                _ => Value::Bool(false),
            },
            _ => Value::Bool(false),
        }
    }

    fn is_truthy(&self, val: Value) -> bool {
        match val {
            Value::Bool(b) => b,
            Value::Number(n) => n != 0,
            _ => false,
        }
    }
}
