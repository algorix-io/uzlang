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
    While(Expr, Vec<Stmt>),
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

                // Expect Newline then Indent
                if self.peek() == &Token::Newline {
                    self.advance();
                }

                if self.peek() == &Token::Indent {
                    let body = self.parse_block()?;
                    Some(Stmt::If(condition, body))
                } else {
                     // Single statement? Not supported for now to be safe
                     eprintln!("Agar blokidan keyin Indent kutilgan edi");
                     None
                }
            }
            Token::Toki => {
                self.advance();
                let condition = self.parse_expr()?;

                 if self.peek() == &Token::Newline {
                    self.advance();
                }

                if self.peek() == &Token::Indent {
                    let body = self.parse_block()?;
                    Some(Stmt::While(condition, body))
                } else {
                     eprintln!("Toki blokidan keyin Indent kutilgan edi");
                     None
                }
            }
            Token::Identifier(name) => {
                if self.peek_next() == &Token::Assign {
                     let name = name.clone();
                     self.advance(); // eat id
                     self.advance(); // eat =
                     let expr = self.parse_expr()?;
                     Some(Stmt::Assign(name, expr))
                } else {
                    // Expression stmt? Not supported
                    None
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
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Option<Expr> {
        let mut left = self.parse_comparison()?;

        while let Token::Operator(op) = self.peek().clone() {
            if op == "==" || op == "!=" {
                self.advance();
                let right = self.parse_comparison()?;
                left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
            } else {
                break;
            }
        }
        Some(left)
    }

    fn parse_comparison(&mut self) -> Option<Expr> {
         let mut left = self.parse_term()?;

         // Helper to check if token is comparison op
         while let Token::Operator(op) = self.peek().clone() {
             match op.as_str() {
                 ">" | "<" | ">=" | "<=" => {
                     self.advance();
                     let right = self.parse_term()?;
                     left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
                 }
                 _ => break,
             }
         }
         Some(left)
    }

    fn parse_term(&mut self) -> Option<Expr> {
        let mut left = self.parse_factor()?;

        while matches!(self.peek(), Token::Plus | Token::Minus) {
             let op = match self.advance() {
                 Token::Plus => "+".to_string(),
                 Token::Minus => "-".to_string(),
                 _ => unreachable!(),
             };
             let right = self.parse_factor()?;
             left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
        }
        Some(left)
    }

    fn parse_factor(&mut self) -> Option<Expr> {
        let mut left = self.parse_primary()?;

        while matches!(self.peek(), Token::Star | Token::Slash) {
             let op = match self.advance() {
                 Token::Star => "*".to_string(),
                 Token::Slash => "/".to_string(),
                 _ => unreachable!(),
             };
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
