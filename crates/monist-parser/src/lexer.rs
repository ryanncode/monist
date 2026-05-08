#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Ident(String),
    Forall,
    Exists,
    Not,
    And,
    Or,
    Impl,
    Iff,
    Eq,
    In,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Bar,
    Dot,
    Comma,
    LessThan,
    EOF,
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek_char()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    fn advance_if(&mut self, expected: char) -> bool {
        if self.peek_char() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if let Some(c) = self.advance() {
            match c {
                '(' => Token::LParen,
                ')' => Token::RParen,
                '{' => Token::LBrace,
                '}' => Token::RBrace,
                '|' => Token::Bar,
                '.' => Token::Dot,
                ',' => Token::Comma,
                '=' => Token::Eq,
                '~' | '¬' => Token::Not,
                '∀' => Token::Forall,
                '∃' => Token::Exists,
                '∧' | '&' => Token::And,
                '∨' => Token::Or,
                '→' => Token::Impl,
                '∈' => Token::In,
                '/' if self.advance_if('\\') => Token::And,
                '\\' if self.advance_if('/') => Token::Or,
                '-' if self.advance_if('>') => Token::Impl,
                '<' if self.advance_if('-') && self.advance_if('>') => Token::Iff,
                '<' => Token::LessThan,
                _ if c.is_alphabetic() => {
                    let mut ident = String::new();
                    ident.push(c);
                    while let Some(nc) = self.peek_char() {
                        if nc.is_alphanumeric() || nc == '_' {
                            ident.push(self.advance().unwrap());
                        } else {
                            break;
                        }
                    }
                    match ident.as_str() {
                        "forall" => Token::Forall,
                        "exists" => Token::Exists,
                        "in" | "e" => Token::In,
                        _ => Token::Ident(ident),
                    }
                }
                _ => panic!("Unexpected character: {}", c),
            }
        } else {
            Token::EOF
        }
    }
}
