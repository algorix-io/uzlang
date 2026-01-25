#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Agar, // agar
    Yoz,  // yoz
    Takrorla, // takrorla
    LBrace, // {
    RBrace, // }
    Identifier(String),
    Number(i64),
    StringLiteral(String),
    Operator(String), // ==, >, <, +, -, *, / etc.
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.pos < self.input.len() {
            match self.input[self.pos] {
                ' ' | '\t' | '\n' | '\r' => {
                    self.pos += 1;
                }
                '"' => {
                    tokens.push(self.read_string());
                }
                '0'..='9' => {
                    tokens.push(self.read_number());
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    tokens.push(self.read_identifier());
                }
                '=' | '!' | '>' | '<' | '+' | '-' | '*' | '/' => {
                    tokens.push(self.read_operator());
                }
                '{' => {
                    tokens.push(Token::LBrace);
                    self.pos += 1;
                }
                '}' => {
                    tokens.push(Token::RBrace);
                    self.pos += 1;
                }
                _ => {
                    // Unknown character, skip for now or error
                    self.pos += 1;
                }
            }
        }
        tokens.push(Token::EOF);
        tokens
    }

    fn read_string(&mut self) -> Token {
        self.pos += 1; // skip opening quote
        let mut s = String::new();
        while self.pos < self.input.len() && self.input[self.pos] != '"' {
            s.push(self.input[self.pos]);
            self.pos += 1;
        }
        self.pos += 1; // skip closing quote
        Token::StringLiteral(s)
    }

    fn read_number(&mut self) -> Token {
        let mut s = String::new();
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
            s.push(self.input[self.pos]);
            self.pos += 1;
        }
        Token::Number(s.parse().unwrap_or(0))
    }

    fn read_identifier(&mut self) -> Token {
        let mut s = String::new();
        while self.pos < self.input.len()
            && (self.input[self.pos].is_ascii_alphanumeric() || self.input[self.pos] == '_')
        {
            s.push(self.input[self.pos]);
            self.pos += 1;
        }

        match s.as_str() {
            "agar" => Token::Agar,
            "yoz" => Token::Yoz,
            "takrorla" => Token::Takrorla,
            _ => Token::Identifier(s),
        }
    }

    fn read_operator(&mut self) -> Token {
        let mut s = String::new();
        let current = self.input[self.pos];
        s.push(current);
        self.pos += 1;

        if self.pos < self.input.len() {
            let next = self.input[self.pos];
            if next == '=' {
                s.push(next);
                self.pos += 1;
            }
        }
        Token::Operator(s)
    }
}
