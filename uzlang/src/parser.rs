use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    StringLiteral(String),
    Identifier(String),
    BinaryOp(Box<Expr>, String, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Expr),
    If(Expr, Vec<Stmt>),
    Loop(Expr, Vec<Stmt>),
    Assign(String, Expr),
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
            match self.peek() {
                Token::Newline => {
                    self.advance();
                }
                _ => {
                    if let Some(stmt) = self.parse_stmt() {
                        stmts.push(stmt);
                    } else {
                        // Skip token to avoid infinite loop on error
                        let token = self.advance();
                        eprintln!("Xatolik: Kutilmagan token: {:?}", token);
                    }
                }
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

    fn peek_next(&self) -> &Token {
        if self.pos + 1 < self.tokens.len() {
            &self.tokens[self.pos + 1]
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
                // Skip one token to attempt recovery, or just break?
                // For MVP, consuming one token matches `parse` loop behavior.
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
                // Expect newline after print?
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
            Token::Identifier(name) => {
                // Check if it is an assignment
                match self.peek_next() {
                    Token::Operator(op) if op == "=" => {
                        let name = name.clone();
                        self.advance(); // consume identifier
                        self.advance(); // consume =
                        let expr = self.parse_expr()?;
                        Some(Stmt::Assign(name, expr))
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn parse_block(&mut self) -> Option<Vec<Stmt>> {
        if self.peek() != &Token::Indent {
            return None;
        }
        self.advance(); // eat Indent

        let mut stmts = Vec::new();

        while self.peek() != &Token::Dedent && self.peek() != &Token::EOF {
             match self.peek() {
                Token::Newline => {
                    self.advance();
                }
                _ => {
                    if let Some(stmt) = self.parse_stmt() {
                        stmts.push(stmt);
                    } else {
                        // Error inside block
                        self.advance();
                    }
                }
            }
        }

        if self.peek() == &Token::Dedent {
            self.advance();
        }

        Some(stmts)
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_comparison()
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
        let mut left = self.parse_primary()?;

        while let Token::Operator(op) = self.peek().clone() {
            if ["*", "/"].contains(&op.as_str()) {
                self.advance();
                let right = self.parse_primary()?;
                left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
            } else {
                break;
            }
        }
        Some(left)
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        match self.advance() {
            Token::Number(n) => Some(Expr::Number(*n)),
            Token::StringLiteral(s) => Some(Expr::StringLiteral(s.clone())),
            Token::Identifier(s) => Some(Expr::Identifier(s.clone())),
            _ => None,
        }
    }
}
