use crate::parser::{Expr, Stmt};
use std::collections::HashMap;
use std::fs;
use std::io::Write;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i64),
    String(String),
    Bool(bool),
    Array(Vec<Value>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
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
        for scope in self.env_stack.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), val);
                return;
            }
        }

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
            Stmt::For(var_name, collection, body) => {
                let collection_val = self.evaluate(collection);
                if let Value::Array(elements) = collection_val {
                    for element in elements {
                        // Create new scope for loop iteration
                        let mut scope = HashMap::new();
                        scope.insert(var_name.clone(), element);
                        self.env_stack.push(scope);

                        let ret = self.execute(body);
                        self.env_stack.pop();

                        if let Some(val) = ret {
                            return Some(val);
                        }
                    }
                } else {
                    eprintln!("Xatolik: 'uchun' faqat massivlar bilan ishlaydi");
                }
                None
            }
            Stmt::Assign(name, expr) => {
                let val = self.evaluate(expr);
                self.set_variable(name, val);
                None
            }
            Stmt::AssignIndex(name, index_expr, value_expr) => {
                let index_val = self.evaluate(index_expr);
                let value_val = self.evaluate(value_expr);
                let mut arr_val = self.get_variable(name);

                if let Value::Array(ref mut elements) = arr_val {
                    if let Value::Number(idx) = index_val {
                        if idx >= 0 && (idx as usize) < elements.len() {
                            elements[idx as usize] = value_val;
                            self.set_variable(name, arr_val);
                        } else {
                            eprintln!("Xatolik: Indeks chegaradan tashqarida: {}", idx);
                        }
                    } else {
                        eprintln!("Xatolik: Indeks raqam bo'lishi kerak");
                    }
                } else {
                    eprintln!("Xatolik: O'zgaruvchi massiv emas: {}", name);
                }
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
            Expr::Array(elements) => {
                let mut values = Vec::new();
                for e in elements {
                    values.push(self.evaluate(e));
                }
                Value::Array(values)
            }
            Expr::Index(target, index) => {
                let target_val = self.evaluate(target);
                let index_val = self.evaluate(index);

                if let Value::Array(elements) = target_val {
                    if let Value::Number(idx) = index_val {
                        if idx >= 0 && (idx as usize) < elements.len() {
                            return elements[idx as usize].clone();
                        } else {
                            eprintln!("Xatolik: Indeks chegaradan tashqarida: {}", idx);
                            return Value::Number(0);
                        }
                    } else {
                        eprintln!("Xatolik: Indeks raqam bo'lishi kerak");
                        return Value::Number(0);
                    }
                } else {
                    eprintln!("Xatolik: Massiv indekslanishi kerak");
                    return Value::Number(0);
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
                                Value::Array(_) => return Value::String("massiv".to_string()),
                            }
                        }
                         return Value::String("noma'lum".to_string());
                    }
                    "uzunlik" => {
                        if let Some(val) = arg_values.first() {
                            if let Value::Array(arr) = val {
                                return Value::Number(arr.len() as i64);
                            }
                        }
                        return Value::Number(0);
                    }
                    "qosh" => {
                        // qosh(arr, val) -> returns new array
                        if arg_values.len() >= 2 {
                             if let Value::Array(mut arr) = arg_values[0].clone() {
                                 arr.push(arg_values[1].clone());
                                 return Value::Array(arr);
                             } else {
                                 eprintln!("Xatolik: 'qosh' funksiyasining birinchi parametri massiv bo'lishi kerak");
                             }
                        }
                        return Value::Number(0);
                    }
                    "fayl_oqi" => {
                        if let Some(val) = arg_values.first() {
                            if let Value::String(path) = val {
                                return match fs::read_to_string(path) {
                                    Ok(content) => Value::String(content),
                                    Err(_) => Value::String("".to_string()),
                                };
                            }
                        }
                        return Value::String("".to_string());
                    }
                    "fayl_yoz" => {
                        if arg_values.len() >= 2 {
                            if let (Value::String(path), Value::String(content)) =
                                (&arg_values[0], &arg_values[1])
                            {
                                return match fs::write(path, content) {
                                    Ok(_) => Value::Bool(true),
                                    Err(_) => Value::Bool(false),
                                };
                            }
                        }
                        return Value::Bool(false);
                    }
                    "fayl_qosh" => {
                        if arg_values.len() >= 2 {
                            if let (Value::String(path), Value::String(content)) =
                                (&arg_values[0], &arg_values[1])
                            {
                                let mut file = match fs::OpenOptions::new()
                                    .create(true)
                                    .append(true)
                                    .open(path)
                                {
                                    Ok(f) => f,
                                    Err(_) => return Value::Bool(false),
                                };
                                return match file.write_all(content.as_bytes()) {
                                    Ok(_) => Value::Bool(true),
                                    Err(_) => Value::Bool(false),
                                };
                            }
                        }
                        return Value::Bool(false);
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
