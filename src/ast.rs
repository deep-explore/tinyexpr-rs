use crate::token::{Lexer, Token, LexError};

/// The Abstract Syntax Tree node
#[derive(Debug, Clone, PartialEq)]
pub enum ExprNode {
    Number(f64),
    Variable(String),
    BinaryOp {
        op: char,
        left: Box<ExprNode>,
        right: Box<ExprNode>,
    },
    FunctionCall {
        name: String,
        args: Vec<ExprNode>,
    },
}

/// Errors during parsing
#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEOF,
    Lex(LexError),
}

/// Pratt-style recursive descent parser
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Lexer::new(input);
        let current = lexer.next_token().map_err(ParseError::Lex)?;
        Ok(Self { lexer, current })
    }

    fn bump(&mut self) -> Result<(), ParseError> {
        self.current = self.lexer.next_token().map_err(ParseError::Lex)?;
        Ok(())
    }

    pub fn parse(&mut self) -> Result<ExprNode, ParseError> {
        self.parse_expr()
    }

    fn parse_expr(&mut self) -> Result<ExprNode, ParseError> {
        self.parse_binary_expr(0)
    }

    fn parse_binary_expr(&mut self, min_prec: u8) -> Result<ExprNode, ParseError> {
        let mut lhs = self.parse_primary()?;

        while let Token::Operator(op) = self.current.clone() {
            let prec = get_precedence(op);
            if prec < min_prec {
                break;
            }

            self.bump()?; // consume operator
            let rhs = self.parse_binary_expr(prec + 1)?;

            lhs = ExprNode::BinaryOp {
                op,
                left: Box::new(lhs),
                right: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    fn parse_primary(&mut self) -> Result<ExprNode, ParseError> {
        match &self.current {
            Token::Number(n) => {
                let node = ExprNode::Number(*n);
                self.bump()?;
                Ok(node)
            }
            Token::Identifier(name) => {
                let ident = name.clone();
                self.bump()?;
                if let Token::LParen = self.current {
                    // It's a function call
                    self.bump()?; // consume '('
                    let mut args = Vec::new();
                    if self.current != Token::RParen {
                        loop {
                            args.push(self.parse_expr()?);
                            if self.current == Token::RParen {
                                break;
                            }
                            if self.current != Token::Comma {
                                return Err(ParseError::UnexpectedToken(self.current.clone()));
                            }
                            self.bump()?; // consume ','
                        }
                    }
                    self.bump()?; // consume ')'
                    Ok(ExprNode::FunctionCall { name: ident, args })
                } else {
                    // It's a variable
                    Ok(ExprNode::Variable(ident))
                }
            }
            
            Token::LParen => {
                self.bump()?; // consume '('
                let expr = self.parse_expr()?;
                if self.current != Token::RParen {
                    return Err(ParseError::UnexpectedToken(self.current.clone()));
                }
                self.bump()?; // consume ')'
                Ok(expr)
            }
            tok => Err(ParseError::UnexpectedToken(tok.clone())),
        }
    }
}

fn get_precedence(op: char) -> u8 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        '^' => 3,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_expr() {
        let mut parser = Parser::new("1 + 2 * 3").unwrap();
        let ast = parser.parse().unwrap();

        assert_eq!(
            ast,
            ExprNode::BinaryOp {
                op: '+',
                left: Box::new(ExprNode::Number(1.0)),
                right: Box::new(ExprNode::BinaryOp {
                    op: '*',
                    left: Box::new(ExprNode::Number(2.0)),
                    right: Box::new(ExprNode::Number(3.0)),
                }),
            }
        );
    }
}
