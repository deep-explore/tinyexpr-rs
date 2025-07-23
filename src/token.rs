#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Identifier(String),
    Operator(char),
    LParen,
    RParen,
    Comma,
    EOF,
}

#[derive(Debug)]
pub enum LexError {
    InvalidCharacter(char),
    UnexpectedEOF,
}

pub struct Lexer<'a> {
    input: &'a str,
    chars: std::str::Chars<'a>,
    curr: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let curr = chars.next();
        Self { input, chars, curr }
    }

    fn bump(&mut self) {
        self.curr = self.chars.next();
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.curr {
            if c.is_whitespace() {
                self.bump();
            } else {
                break;
            }
        }
    }

    fn lex_number(&mut self) -> Result<Token, LexError> {
        let mut number = String::new();

        while let Some(c) = self.curr {
            if c.is_ascii_digit() || c == '.' {
                number.push(c);
                self.bump();
            } else {
                break;
            }
        }

        match number.parse::<f64>() {
            Ok(num) => Ok(Token::Number(num)),
            Err(_) => Err(LexError::InvalidCharacter('.')),
        }
    }

    fn lex_identifier(&mut self) -> Token {
        let mut ident = String::new();

        while let Some(c) = self.curr {
            if c.is_ascii_alphanumeric() || c == '_' {
                ident.push(c);
                self.bump();
            } else {
                break;
            }
        }

        Token::Identifier(ident)
    }

    pub fn next_token(&mut self) -> Result<Token, LexError> {
        self.skip_whitespace();

        match self.curr {
            Some(c) if c.is_ascii_digit() => self.lex_number(),
            Some(c) if c.is_ascii_alphabetic() => Ok(self.lex_identifier()),
            Some('+') | Some('-') | Some('*') | Some('/') | Some('^') => {
                let op = self.curr.unwrap();
                self.bump();
                Ok(Token::Operator(op))
            }
            Some('(') => {
                self.bump();
                Ok(Token::LParen)
            }
            Some(')') => {
                self.bump();
                Ok(Token::RParen)
            }
            Some(',') => {
                self.bump();
                Ok(Token::Comma)
            }
            Some(c) => {
                self.bump();
                Err(LexError::InvalidCharacter(c))
            }
            None => Ok(Token::EOF),
        }
    }
}
