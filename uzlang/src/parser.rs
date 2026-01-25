use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    StringLiteral(String),
    Identifier(String),
    BinaryOp(Box<Expr>, String, Box<Expr>),
    UnaryOp(String, Box<Expr>),
    Call(String, Vec<Expr>),
    Input,
    Array(Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Expr),
    If(Expr, Vec<Stmt>),
    Loop(Expr, Vec<Stmt>), // while
    For(String, Expr, Vec<Stmt>), // for
    Assign(String, Expr),
    AssignIndex(String, Expr, Expr), // name, index, value
    Function(String, Vec<String>, Vec<Stmt>),
    Return(Expr),
    Expr(Expr),
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.peek() != &Token::EOF {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            } else {
                // Skip token to avoid infinite loop on error
                let token = self.advance();
                eprintln!("Xatolik: Kutilmagan token: {:?}", token);
            }
        }
        stmts
    }

    fn peek(&self) -> &Token {
        if self.pos < self.tokens.len() {
            &self.tokens[self.pos]
        } else {
            &Token::EOF
        }
    }

    fn advance(&mut self) -> &Token {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        if self.pos > 0 {
            &self.tokens[self.pos - 1]
        } else {
            &Token::EOF
        }
    }

    fn parse_block(&mut self) -> Option<Vec<Stmt>> {
        if let Token::LBrace = self.peek() {
            self.advance(); // consume {
        } else {
            eprintln!("Xatolik: Blok {{ bilan boshlanishi kerak");
            return None;
        }

        let mut stmts = Vec::new();
        while self.peek() != &Token::RBrace && self.peek() != &Token::EOF {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            } else {
                self.advance();
            }
        }

        if let Token::RBrace = self.peek() {
            self.advance(); // consume }
            Some(stmts)
        } else {
            eprintln!("Xatolik: Blok }} bilan tugashi kerak");
            None
        }
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.peek() {
            Token::Yoz => {
                self.advance();
                let expr = self.parse_expr()?;
                Some(Stmt::Print(expr))
            }
            Token::Agar => {
                self.advance();
                let condition = self.parse_expr()?;
                let body = self.parse_block()?;
                Some(Stmt::If(condition, body))
            }
            Token::Takrorla => {
                self.advance();
                let condition = self.parse_expr()?;
                let body = self.parse_block()?;
                Some(Stmt::Loop(condition, body))
            }
            Token::Uchun => {
                self.advance(); // consume uchun
                if let Token::Identifier(var_name) = self.advance().clone() {
                    if let Token::Ichida = self.peek() {
                        self.advance(); // consume ichida
                        let collection = self.parse_expr()?;
                        let body = self.parse_block()?;
                        return Some(Stmt::For(var_name, collection, body));
                    }
                }
                eprintln!("Xatolik: For tsikli 'uchun <var> ichida <expr>' formatida bo'lishi kerak");
                None
            }
            Token::Funksiya => {
                self.advance();
                if let Token::Identifier(name) = self.advance().clone() {
                    if let Token::LParen = self.advance() {
                        let mut params = Vec::new();
                        if self.peek() != &Token::RParen {
                            loop {
                                if let Token::Identifier(param) = self.advance().clone() {
                                    params.push(param);
                                } else {
                                    eprintln!("Xatolik: Parametr nomi kutilgan");
                                    return None;
                                }

                                if self.peek() == &Token::Comma {
                                    self.advance();
                                } else {
                                    break;
                                }
                            }
                        }

                        if let Token::RParen = self.advance() {
                            let body = self.parse_block()?;
                            return Some(Stmt::Function(name, params, body));
                        }
                    }
                }
                eprintln!("Xatolik: Funksiya deklaratsiyasi noto'g'ri");
                None
            }
            Token::Qaytar => {
                self.advance();
                let expr = self.parse_expr()?;
                Some(Stmt::Return(expr))
            }
            _ => {
                // Expression statement or Assignment
                let expr = self.parse_expr()?;

                if let Token::Operator(op) = self.peek() {
                    if op == "=" {
                        self.advance(); // consume =
                        let value = self.parse_expr()?;

                        match expr {
                            Expr::Identifier(name) => return Some(Stmt::Assign(name, value)),
                            Expr::Index(target, index) => {
                                if let Expr::Identifier(name) = *target {
                                    return Some(Stmt::AssignIndex(name, *index, value));
                                } else {
                                    eprintln!("Xatolik: Faqat o'zgaruvchilarga indeks orqali qiymat berish mumkin");
                                    return None;
                                }
                            }
                            _ => {
                                eprintln!("Xatolik: Noto'g'ri o'zlashtirish, chap tarafda o'zgaruvchi bo'lishi kerak");
                                return None;
                            }
                        }
                    }
                }

                Some(Stmt::Expr(expr))
            }
        }
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Option<Expr> {
        let mut left = self.parse_logical_and()?;

        while let Token::Or = self.peek() {
            self.advance();
            let right = self.parse_logical_and()?;
            left = Expr::BinaryOp(Box::new(left), "||".to_string(), Box::new(right));
        }
        Some(left)
    }

    fn parse_logical_and(&mut self) -> Option<Expr> {
        let mut left = self.parse_comparison()?;

        while let Token::And = self.peek() {
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::BinaryOp(Box::new(left), "&&".to_string(), Box::new(right));
        }
        Some(left)
    }

    fn parse_comparison(&mut self) -> Option<Expr> {
        let mut left = self.parse_term()?;

        while let Token::Operator(op) = self.peek().clone() {
            if ["==", "!=", "<", ">", "<=", ">="].contains(&op.as_str()) {
                self.advance();
                let right = self.parse_term()?;
                left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
            } else {
                break;
            }
        }
        Some(left)
    }

    fn parse_term(&mut self) -> Option<Expr> {
        let mut left = self.parse_factor()?;

        while let Token::Operator(op) = self.peek().clone() {
            if ["+", "-"].contains(&op.as_str()) {
                self.advance();
                let right = self.parse_factor()?;
                left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
            } else {
                break;
            }
        }
        Some(left)
    }

    fn parse_factor(&mut self) -> Option<Expr> {
        let mut left = self.parse_unary()?;

        while let Token::Operator(op) = self.peek().clone() {
            if ["*", "/"].contains(&op.as_str()) {
                self.advance();
                let right = self.parse_unary()?;
                left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
            } else {
                break;
            }
        }
        Some(left)
    }

    fn parse_unary(&mut self) -> Option<Expr> {
        if let Token::Not = self.peek() {
            self.advance();
            let right = self.parse_unary()?;
            Some(Expr::UnaryOp("!".to_string(), Box::new(right)))
        } else {
            self.parse_postfix()
        }
    }

    fn parse_postfix(&mut self) -> Option<Expr> {
        let mut left = self.parse_primary()?;

        loop {
            match self.peek() {
                Token::LBracket => {
                    self.advance(); // consume [
                    let index = self.parse_expr()?;
                    if let Token::RBracket = self.advance() {
                        left = Expr::Index(Box::new(left), Box::new(index));
                    } else {
                        eprintln!("Xatolik: ] kutilgan");
                        return None;
                    }
                }
                Token::LParen => {
                    // Function Call
                    if let Expr::Identifier(name) = left {
                        self.advance(); // consume (
                        let mut args = Vec::new();
                        if self.peek() != &Token::RParen {
                             loop {
                                if let Some(arg) = self.parse_expr() {
                                    args.push(arg);
                                } else {
                                    break;
                                }

                                if self.peek() == &Token::Comma {
                                    self.advance();
                                } else {
                                    break;
                                }
                            }
                        }

                        if let Token::RParen = self.advance() {
                            left = Expr::Call(name, args);
                        } else {
                            eprintln!("Xatolik: ) kutilgan");
                            return None;
                        }
                    } else {
                         // Call on non-identifier (e.g. (func())()) - not supported for now as Call takes String
                         // Or grouping? No, grouping is handled in parse_primary
                         eprintln!("Xatolik: Faqat funksiya nomini chaqirish mumkin");
                         return None;
                    }
                }
                _ => break,
            }
        }
        Some(left)
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        match self.peek() {
            Token::Number(n) => {
                let n = *n;
                self.advance();
                Some(Expr::Number(n))
            }
            Token::StringLiteral(s) => {
                let s = s.clone();
                self.advance();
                Some(Expr::StringLiteral(s))
            }
            Token::Identifier(s) => {
                let s = s.clone();
                self.advance();
                Some(Expr::Identifier(s))
            }
            Token::Sora => {
                self.advance();
                Some(Expr::Input)
            }
            Token::LBracket => {
                self.advance(); // consume [
                let mut elements = Vec::new();
                if self.peek() != &Token::RBracket {
                    loop {
                        elements.push(self.parse_expr()?);
                        if self.peek() == &Token::Comma {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                if let Token::RBracket = self.advance() {
                    Some(Expr::Array(elements))
                } else {
                    eprintln!("Xatolik: ] kutilgan");
                    None
                }
            }
            Token::LParen => {
                self.advance(); // consume (
                let expr = self.parse_expr()?;
                if let Token::RParen = self.advance() {
                    Some(expr)
                } else {
                    eprintln!("Xatolik: ) kutilgan");
                    None
                }
            }
            _ => None,
        }
    }
}
