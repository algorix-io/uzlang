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
    env_stack: Vec<HashMap<String, Value>>,
    functions: HashMap<String, (Vec<String>, Vec<Stmt>)>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env_stack: vec![HashMap::new()],
            functions: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: &str, val: Value) {
        if let Some(scope) = self.env_stack.last_mut() {
            scope.insert(name.to_string(), val);
        }
    }

    pub fn get_variable(&self, name: &str) -> Value {
        for scope in self.env_stack.iter().rev() {
            if let Some(val) = scope.get(name) {
                return val.clone();
            }
        }
        Value::Number(0)
    }

    pub fn execute(&mut self, stmts: &[Stmt]) -> Option<Value> {
        for stmt in stmts {
            if let Some(val) = self.execute_stmt(stmt) {
                return Some(val);
            }
        }
        None
    }

    fn execute_stmt(&mut self, stmt: &Stmt) -> Option<Value> {
        match stmt {
            Stmt::Print(expr) => {
                let val = self.evaluate(expr);
                println!("{}", val);
                None
            }
            Stmt::If(cond, body) => {
                let val = self.evaluate(cond);
                if self.is_truthy(val) {
                    return self.execute(body);
                }
                None
            }
            Stmt::Loop(cond, body) => {
                while {
                    let val = self.evaluate(cond);
                    self.is_truthy(val)
                } {
                    if let Some(ret) = self.execute(body) {
                        return Some(ret);
                    }
                }
                None
            }
            Stmt::Assign(name, expr) => {
                let val = self.evaluate(expr);
                self.set_variable(name, val);
                None
            }
            Stmt::Function(name, params, body) => {
                self.functions.insert(name.clone(), (params.clone(), body.clone()));
                None
            }
            Stmt::Return(expr) => {
                Some(self.evaluate(expr))
            }
            Stmt::Expr(expr) => {
                self.evaluate(expr);
                None
            }
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Number(n) => Value::Number(*n),
            Expr::StringLiteral(s) => Value::String(s.clone()),
            Expr::Identifier(name) => self.get_variable(name),
            Expr::Input => {
                let mut input = String::new();
                if std::io::stdin().read_line(&mut input).is_ok() {
                    Value::String(input.trim().to_string())
                } else {
                    Value::String(String::new())
                }
            }
            Expr::Call(name, args) => {
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.evaluate(arg));
                }

                // Native functions
                match name.as_str() {
                    "son" => {
                        if let Some(val) = arg_values.first() {
                             match val {
                                Value::String(s) => return Value::Number(s.trim().parse().unwrap_or(0)),
                                Value::Number(n) => return Value::Number(*n),
                                _ => return Value::Number(0),
                             }
                        }
                        return Value::Number(0);
                    }
                    "matn" => {
                        if let Some(val) = arg_values.first() {
                            return Value::String(val.to_string());
                        }
                        return Value::String("".to_string());
                    }
                    "turi" => {
                        if let Some(val) = arg_values.first() {
                            match val {
                                Value::Number(_) => return Value::String("son".to_string()),
                                Value::String(_) => return Value::String("matn".to_string()),
                                Value::Bool(_) => return Value::String("mantiq".to_string()),
                            }
                        }
                         return Value::String("noma'lum".to_string());
                    }
                    _ => {}
                }

                // User functions
                if let Some((params, body)) = self.functions.get(name).cloned() {
                    // Create new scope
                    let mut scope = HashMap::new();
                    for (i, param) in params.iter().enumerate() {
                        if let Some(val) = arg_values.get(i) {
                            scope.insert(param.clone(), val.clone());
                        } else {
                            // Default value for missing args?
                            scope.insert(param.clone(), Value::Number(0));
                        }
                    }

                    self.env_stack.push(scope);
                    let result = self.execute(&body);
                    self.env_stack.pop();

                    return result.unwrap_or(Value::Number(0)); // Default return 0
                }

                eprintln!("Xatolik: Funksiya topilmadi: {}", name);
                Value::Number(0)
            }
            Expr::UnaryOp(op, right) => {
                let val = self.evaluate(right);
                match op.as_str() {
                    "!" => Value::Bool(!self.is_truthy(val)),
                    _ => Value::Bool(false),
                }
            }
            Expr::BinaryOp(left, op, right) => {
                if op == "&&" {
                    let l = self.evaluate(left);
                    if !self.is_truthy(l) {
                        return Value::Bool(false);
                    }
                    let r = self.evaluate(right);
                    return Value::Bool(self.is_truthy(r));
                }
                if op == "||" {
                    let l = self.evaluate(left);
                    if self.is_truthy(l) {
                        return Value::Bool(true);
                    }
                    let r = self.evaluate(right);
                    return Value::Bool(self.is_truthy(r));
                }
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
            (Value::String(l), Value::Number(r)) => match op {
                "+" => Value::String(format!("{}{}", l, r)),
                _ => Value::Bool(false),
            },
            (Value::Number(l), Value::String(r)) => match op {
                "+" => Value::String(format!("{}{}", l, r)),
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
