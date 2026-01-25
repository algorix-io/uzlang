#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Agar, // agar
    Toki, // toki (while)
    Yoz,  // yoz
    Takrorla, // takrorla
    Sora, // so'ra
    Funksiya, // funksiya
    Qaytar, // qaytar
    Uchun, // uchun (for)
    Ichida, // ichida (in)
    And,  // &&
    Or,   // ||
    Not,  // !
    LBrace, // {
    RBrace, // }
    LParen, // (
    RParen, // )
    LBracket, // [
    RBracket, // ]
    Comma,  // ,
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
                ' ' | '\t' | '\r' | '\n' => {
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
                '/' => {
                    if self.pos + 1 < self.input.len() && self.input[self.pos + 1] == '/' {
                        // Skip comment until newline
                        while self.pos < self.input.len() && self.input[self.pos] != '\n' {
                            self.pos += 1;
                        }
                    } else {
                        tokens.push(self.read_operator());
                    }
                }
                '=' | '!' | '>' | '<' | '+' | '-' | '*' | '&' | '|' => {
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
                '(' => {
                    tokens.push(Token::LParen);
                    self.pos += 1;
                }
                ')' => {
                    tokens.push(Token::RParen);
                    self.pos += 1;
                }
                '[' => {
                    tokens.push(Token::LBracket);
                    self.pos += 1;
                }
                ']' => {
                    tokens.push(Token::RBracket);
                    self.pos += 1;
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.pos += 1;
                }
                _ => {
                    // Unknown character, skip for now
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
            if self.input[self.pos] == '\\' && self.pos + 1 < self.input.len() {
                self.pos += 1;
                match self.input[self.pos] {
                    'n' => s.push('\n'),
                    'r' => s.push('\r'),
                    't' => s.push('\t'),
                    '"' => s.push('"'),
                    '\\' => s.push('\\'),
                    c => {
                        s.push('\\');
                        s.push(c);
                    }
                }
            } else {
                s.push(self.input[self.pos]);
            }
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
            && (self.input[self.pos].is_ascii_alphanumeric()
                || self.input[self.pos] == '_'
                || self.input[self.pos] == '\'')
        {
            s.push(self.input[self.pos]);
            self.pos += 1;
        }

        match s.as_str() {
            "agar" => Token::Agar,
            "toki" => Token::Toki,
            "yoz" => Token::Yoz,
            "takrorla" => Token::Takrorla,
            "so'ra" => Token::Sora,
            "funksiya" => Token::Funksiya,
            "qaytar" => Token::Qaytar,
            "uchun" => Token::Uchun,
            "ichida" => Token::Ichida,
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
            if current == '&' && next == '&' {
                s.push(next);
                self.pos += 1;
                return Token::And;
            }
            if current == '|' && next == '|' {
                s.push(next);
                self.pos += 1;
                return Token::Or;
            }
            if next == '=' {
                s.push(next);
                self.pos += 1;
            }
        }

        match s.as_str() {
            "!" => Token::Not,
            _ => Token::Operator(s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_braces() {
        let input = "takrorla { yoz 1 }";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(tokens, vec![
            Token::Takrorla,
            Token::LBrace,
            Token::Yoz,
            Token::Number(1),
            Token::RBrace,
            Token::EOF
        ]);
    }

    #[test]
    fn test_ignore_indentation() {
        let input = "
        takrorla {
            yoz 1
        }";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(tokens, vec![
            Token::Takrorla,
            Token::LBrace,
            Token::Yoz,
            Token::Number(1),
            Token::RBrace,
            Token::EOF
        ]);
    }

    #[test]
    fn test_new_features() {
        let input = "so'ra && || ! // comment";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(tokens, vec![
            Token::Sora,
            Token::And,
            Token::Or,
            Token::Not,
            Token::EOF
        ]);
    }

    #[test]
    fn test_operators_mixed() {
        let input = "!= ! == &&";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(tokens, vec![
            Token::Operator("!=".to_string()),
            Token::Not,
            Token::Operator("==".to_string()),
            Token::And,
            Token::EOF
        ]);
    }

    #[test]
    fn test_functions() {
        let input = "funksiya qosh(a, b) { qaytar a + b }";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(tokens, vec![
            Token::Funksiya,
            Token::Identifier("qosh".to_string()),
            Token::LParen,
            Token::Identifier("a".to_string()),
            Token::Comma,
            Token::Identifier("b".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Qaytar,
            Token::Identifier("a".to_string()),
            Token::Operator("+".to_string()),
            Token::Identifier("b".to_string()),
            Token::RBrace,
            Token::EOF
        ]);
    }

    #[test]
    fn test_arrays_and_loops() {
        let input = "uchun x ichida [1, 2]";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(tokens, vec![
            Token::Uchun,
            Token::Identifier("x".to_string()),
            Token::Ichida,
            Token::LBracket,
            Token::Number(1),
            Token::Comma,
            Token::Number(2),
            Token::RBracket,
            Token::EOF
        ]);
    }
}
