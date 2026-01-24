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
                // Simplification: Expect a single statement as the body for now
                let body_stmt = self.parse_stmt()?;
                Some(Stmt::If(condition, vec![body_stmt]))
            }
            _ => None,
        }
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_primary()?;

        while let Token::Operator(op) = self.peek().clone() {
            self.advance();
            let right = self.parse_primary()?;
            left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
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
