#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Agar, // agar
    Toki, // toki (while)
    Yoz,  // yoz
    Identifier(String),
    Number(i64),
    StringLiteral(String),
    Operator(String), // ==, >, <, etc.
    Assign, // =
    Plus,   // +
    Minus,  // -
    Star,   // *
    Slash,  // /
    Newline,
    Indent,
    Dedent,
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    indent_stack: Vec<usize>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
            indent_stack: vec![0],
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.pos < self.input.len() {
            match self.input[self.pos] {
                '\n' => {
                    self.handle_newline(&mut tokens);
                }
                ' ' | '\t' | '\r' => {
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
                '+' => {
                    tokens.push(Token::Plus);
                    self.pos += 1;
                }
                '-' => {
                    tokens.push(Token::Minus);
                    self.pos += 1;
                }
                '*' => {
                    tokens.push(Token::Star);
                    self.pos += 1;
                }
                '/' => {
                    tokens.push(Token::Slash);
                    self.pos += 1;
                }
                '=' => {
                    if self.peek_char() == '=' {
                        tokens.push(Token::Operator("==".to_string()));
                        self.pos += 2;
                    } else {
                        tokens.push(Token::Assign);
                        self.pos += 1;
                    }
                }
                '!' | '>' | '<' => {
                    tokens.push(self.read_operator());
                }
                _ => {
                    // Unknown character, skip for now
                    self.pos += 1;
                }
            }
        }

        // Emit Dedents for remaining indentation
        while self.indent_stack.len() > 1 {
            self.indent_stack.pop();
            tokens.push(Token::Dedent);
        }

        tokens.push(Token::EOF);
        tokens
    }

    fn peek_char(&self) -> char {
        if self.pos + 1 < self.input.len() {
            self.input[self.pos + 1]
        } else {
            '\0'
        }
    }

    fn handle_newline(&mut self, tokens: &mut Vec<Token>) {
        self.pos += 1; // skip initial \n

        let mut spaces = 0;
        loop {
            if self.pos >= self.input.len() {
                break;
            }
            match self.input[self.pos] {
                ' ' => {
                    spaces += 1;
                    self.pos += 1;
                }
                '\t' => {
                    spaces += 4; // assume tab = 4 spaces
                    self.pos += 1;
                }
                '\r' => {
                    self.pos += 1;
                }
                '\n' => {
                    // Empty line (just whitespace), reset and continue
                    spaces = 0;
                    self.pos += 1;
                }
                _ => break,
            }
        }

        if self.pos >= self.input.len() {
            return;
        }

        tokens.push(Token::Newline);

        let current_indent = spaces;
        let last_indent = *self.indent_stack.last().unwrap();

        if current_indent > last_indent {
            self.indent_stack.push(current_indent);
            tokens.push(Token::Indent);
        } else if current_indent < last_indent {
            while *self.indent_stack.last().unwrap() > current_indent {
                self.indent_stack.pop();
                tokens.push(Token::Dedent);
            }
        }
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
            "toki" => Token::Toki,
            "yoz" => Token::Yoz,
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
