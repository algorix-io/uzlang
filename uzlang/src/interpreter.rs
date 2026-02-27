use crate::parser::{Expr, Stmt};
use reqwest;
use std::collections::HashMap;
use std::io::Read;
use std::net::ToSocketAddrs;
use std::rc::Rc;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i64),
    String(Rc<str>),
    Bool(bool),
    Array(Rc<Vec<Value>>),
}

impl Value {
    pub fn empty_string() -> Self {
        Value::String(Rc::from(""))
    }
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

// Use Rc<str> for function parameters to avoid string cloning on every function call.
type FunctionDef = (Rc<Vec<Rc<str>>>, Rc<Vec<Stmt>>);

pub struct Interpreter {
    env_stack: Vec<HashMap<Rc<str>, Value>>,
    functions: HashMap<String, FunctionDef>,
    client: reqwest::blocking::Client,
}

fn is_safe_ip(ip: std::net::IpAddr) -> bool {
    match ip {
        std::net::IpAddr::V4(ipv4) => {
            let octets = ipv4.octets();
            // Loopback 127.0.0.0/8
            if octets[0] == 127 { return false; }
            // Private 10.0.0.0/8
            if octets[0] == 10 { return false; }
            // Private 172.16.0.0/12
            if octets[0] == 172 && (16..=31).contains(&octets[1]) { return false; }
            // Private 192.168.0.0/16
            if octets[0] == 192 && octets[1] == 168 { return false; }
            // Link-local 169.254.0.0/16
            if octets[0] == 169 && octets[1] == 254 { return false; }
            // Current network 0.0.0.0/8
            if octets[0] == 0 { return false; }
            // CGNAT 100.64.0.0/10
            if octets[0] == 100 && (64..=127).contains(&octets[1]) { return false; }
            // Broadcast 255.255.255.255
            if octets == [255, 255, 255, 255] { return false; }
            true
        },
        std::net::IpAddr::V6(ipv6) => {
            if ipv6.is_loopback() { return false; }
            if ipv6.is_unspecified() { return false; }
            let segments = ipv6.segments();
            // Unique local fc00::/7
            if (segments[0] & 0xfe00) == 0xfc00 { return false; }
            // Link-local fe80::/10
            if (segments[0] & 0xffc0) == 0xfe80 { return false; }
            // IPv4-mapped ::ffff:0:0/96
            if let Some(ipv4) = ipv6.to_ipv4() {
                 return is_safe_ip(std::net::IpAddr::V4(ipv4));
            }
            true
        }
    }
}

fn is_safe_url(url_str: &str) -> bool {
    if let Ok(url) = reqwest::Url::parse(url_str) {
        if url.scheme() != "http" && url.scheme() != "https" {
            return false;
        }
        if let Some(host) = url.host_str() {
            // Defense in depth: Check known bad hosts (string based)
            if host == "localhost" || host == "::1" || host == "[::1]" {
                return false;
            }
            if host.starts_with("127.") {
                return false;
            }
            // Resolve DNS to prevent rebinding/bypasses like localtest.me
            let port = url.port_or_known_default().unwrap_or(80);
            let addr_str = format!("{}:{}", host, port);

            if let Ok(addrs) = addr_str.to_socket_addrs() {
                for addr in addrs {
                    if !is_safe_ip(addr.ip()) {
                        return false;
                    }
                }
            }
            return true;
        }
        // If resolution fails, we cannot verify safety, so we block.
        return false;
    }
    false
}

const MAX_RESPONSE_SIZE: u64 = 5 * 1024 * 1024;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env_stack: vec![HashMap::new()],
            functions: HashMap::new(),
            client: reqwest::blocking::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap(),
        }
    }

    pub fn set_variable(&mut self, name: &str, val: Value) {
        for scope in self.env_stack.iter_mut().rev() {
            if let Some(existing_val) = scope.get_mut(name) {
                *existing_val = val;
                return;
            }
        }

        if let Some(scope) = self.env_stack.last_mut() {
            scope.insert(Rc::from(name), val);
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
                    let var_name_rc: Rc<str> = Rc::from(var_name.as_str());
                    // Optimization: Reuse the same HashMap for scope to avoid allocation in every iteration
                    let mut scope = HashMap::new();
                    for element in elements.iter() {
                        scope.insert(var_name_rc.clone(), element.clone());
                        self.env_stack.push(scope);

                        let ret = self.execute(body);
                        // Retrieve the scope to reuse it
                        scope = self.env_stack.pop().expect("Stack error in For loop");
                        // Clear variables declared in the loop body, but keep allocation
                        scope.clear();

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

                // Optimized Update: Find the variable scope and update in place
                // This avoids cloning the Rc (which happens in get_variable)
                // and allows Rc::make_mut to modify the array without cloning the vector
                // if the ref count is 1.

                let mut found = false;
                for scope in self.env_stack.iter_mut().rev() {
                    if let Some(val) = scope.get_mut(name as &str) {
                        if let Value::Array(rc_arr) = val {
                            if let Value::Number(idx) = index_val {
                                let elements = Rc::make_mut(rc_arr);
                                if idx >= 0 && (idx as usize) < elements.len() {
                                    elements[idx as usize] = value_val;
                                } else {
                                    eprintln!("Xatolik: Indeks chegaradan tashqarida: {}", idx);
                                }
                            } else {
                                eprintln!("Xatolik: Indeks raqam bo'lishi kerak");
                            }
                        } else {
                             eprintln!("Xatolik: O'zgaruvchi massiv emas: {}", name);
                        }
                        found = true;
                        break;
                    }
                }

                if !found {
                     eprintln!("Xatolik: O'zgaruvchi topilmadi: {}", name);
                }
                None
            }
            Stmt::Function(name, params, body) => {
                let params_rc: Vec<Rc<str>> = params.iter().map(|p| Rc::from(p.as_str())).collect();
                self.functions.insert(
                    name.clone(),
                    (Rc::new(params_rc), Rc::new(body.clone())),
                );
                None
            }
            Stmt::Return(expr) => Some(self.evaluate(expr)),
            Stmt::Expr(expr) => {
                self.evaluate(expr);
                None
            }
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Number(n) => Value::Number(*n),
            Expr::StringLiteral(s) => Value::String(Rc::from(s.as_str())),
            Expr::Identifier(name) => self.get_variable(name),
            Expr::Input => {
                let mut input = String::new();
                if std::io::stdin().read_line(&mut input).is_ok() {
                    Value::String(Rc::from(input.trim()))
                } else {
                    Value::empty_string()
                }
            }
            Expr::Array(elements) => {
                let mut values = Vec::new();
                for e in elements {
                    values.push(self.evaluate(e));
                }
                Value::Array(Rc::new(values))
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
                                Value::String(s) => {
                                    return Value::Number(s.trim().parse().unwrap_or(0));
                                }
                                Value::Number(n) => return Value::Number(*n),
                                _ => return Value::Number(0),
                            }
                        }
                        return Value::Number(0);
                    }
                    "matn" => {
                        if let Some(val) = arg_values.first() {
                            return Value::String(Rc::from(val.to_string()));
                        }
                        return Value::empty_string();
                    }
                    "turi" => {
                        if let Some(val) = arg_values.first() {
                            match val {
                                Value::Number(_) => return Value::String(Rc::from("son")),
                                Value::String(_) => return Value::String(Rc::from("matn")),
                                Value::Bool(_) => return Value::String(Rc::from("mantiq")),
                                Value::Array(_) => return Value::String(Rc::from("massiv")),
                            }
                        }
                         return Value::String(Rc::from("noma'lum"));
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
                             if let Value::Array(rc_arr) = &arg_values[0] {
                                 let mut arr = (**rc_arr).clone();
                                 arr.push(arg_values[1].clone());
                                 return Value::Array(Rc::new(arr));
                             } else {
                                 eprintln!("Xatolik: 'qosh' funksiyasining birinchi parametri massiv bo'lishi kerak");
                             }
                        }
                        return Value::Number(0);
                    }
                    "internet_ol" => {
                        if let Some(val) = arg_values.first() {
                            let url = val.to_string();

                            if !is_safe_url(&url) {
                                eprintln!(
                                    "Xatolik: Xavfsizlik qoidasi buzildi - mahalliy yoki xususiy tarmoqqa ulanish taqiqlangan: {}",
                                    url
                                );
                                return Value::empty_string();
                            }

                            // Use shared client that does not follow redirects for security
                            match self.client.get(&url).send() {
                                Ok(resp) => {
                                    let mut buffer = String::new();
                                    if resp.take(MAX_RESPONSE_SIZE).read_to_string(&mut buffer).is_err() {
                                        eprintln!("Xatolik: Javobni o'qishda xatolik");
                                        return Value::empty_string();
                                    }
                                    return Value::String(Rc::from(buffer));
                                },
                                Err(e) => {
                                    eprintln!("Xatolik: Internet so'rovida xatolik: {}", e);
                                    return Value::empty_string();
                                }
                            }
                        }
                        return Value::empty_string();
                    }
                    "internet_yoz" => {
                        if arg_values.len() >= 2 {
                            let url = arg_values[0].to_string();
                            let json_data = arg_values[1].to_string();

                            if !is_safe_url(&url) {
                                eprintln!(
                                    "Xatolik: Xavfsizlik qoidasi buzildi - mahalliy yoki xususiy tarmoqqa ulanish taqiqlangan: {}",
                                    url
                                );
                                return Value::empty_string();
                            }

                            // Use shared client that does not follow redirects for security
                            match self.client
                                .post(&url)
                                .header("Content-Type", "application/json")
                                .body(json_data)
                                .send() {
                                Ok(resp) => {
                                    let mut buffer = String::new();
                                    if resp.take(MAX_RESPONSE_SIZE).read_to_string(&mut buffer).is_err() {
                                        eprintln!("Xatolik: Javobni o'qishda xatolik");
                                        return Value::empty_string();
                                    }
                                    return Value::String(Rc::from(buffer));
                                },
                                Err(e) => {
                                    eprintln!("Xatolik: Internet so'rovida xatolik: {}", e);
                                    return Value::empty_string();
                                }
                            }
                        }
                        return Value::empty_string();
                    }
                    _ => {}
                }

                // User functions
                if let Some((params, body)) = self.functions.get(name) {
                    let params = Rc::clone(params);
                    let body = Rc::clone(body);

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
                "+" => Value::String(Rc::from(format!("{}{}", l, r))),
                "==" => Value::Bool(l == r),
                "!=" => Value::Bool(l != r),
                _ => Value::Bool(false),
            },
            (Value::String(l), Value::Number(r)) => match op {
                "+" => Value::String(Rc::from(format!("{}{}", l, r))),
                _ => Value::Bool(false),
            },
            (Value::Number(l), Value::String(r)) => match op {
                "+" => Value::String(Rc::from(format!("{}{}", l, r))),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe_ip_v4() {
        assert!(!is_safe_ip("127.0.0.1".parse().unwrap()));
        assert!(!is_safe_ip("10.0.0.1".parse().unwrap()));
        assert!(!is_safe_ip("192.168.1.1".parse().unwrap()));
        assert!(!is_safe_ip("172.16.0.1".parse().unwrap()));
        assert!(!is_safe_ip("169.254.1.1".parse().unwrap()));
        assert!(!is_safe_ip("0.0.0.0".parse().unwrap()));
        assert!(is_safe_ip("8.8.8.8".parse().unwrap()));
        assert!(is_safe_ip("1.1.1.1".parse().unwrap()));
    }

    #[test]
    fn test_is_safe_ip_v6() {
        assert!(!is_safe_ip("::1".parse().unwrap()));
        assert!(!is_safe_ip("::".parse().unwrap()));
        assert!(!is_safe_ip("fc00::1".parse().unwrap()));
        assert!(!is_safe_ip("fe80::1".parse().unwrap()));
        assert!(!is_safe_ip("::ffff:127.0.0.1".parse().unwrap()));
        assert!(is_safe_ip("2001:db8::1".parse().unwrap()));
    }
}
